//! Session core — per-process session targets (spec 062).
//!
//! The shared default, `.ductus/session.toml`, holds one target for the whole
//! working tree. Two agent processes in one tree used to share it, so a target
//! written by one silently moved the other. This module gives each *identified*
//! process its own target, at `.ductus/sessions/{key}.toml`, while the default
//! keeps its path, keys and resolution chain.
//!
//! Everything that reads or writes session state goes through here —
//! `write-session`, `resolve-session`, `retarget-sessions`, `dashboard` and the
//! `ductus exec` seed — because pin-on-first-resolution and once-only notices
//! hold only if every reader follows the same rule.
//!
//! The identity is a **parameter** to every operation. Only the edges (the MCP
//! handlers, the CLI entry, the exec seed) read it from the process
//! environment, through [`identity_from_process_env`]; tests pass identities
//! directly rather than mutating the environment, which is `unsafe` in edition
//! 2024 and racy across parallel tests.

use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::primitives::{PrimitiveError, Result, derive_slug, write_atomic};
use crate::schema::paths;

/// The environment variable an operator sets at launch to name a session.
pub const DUCTUS_SESSION_VAR: &str = "DUCTUS_SESSION";

/// The source recorded for an identity taken from [`DUCTUS_SESSION_VAR`].
pub const NAMED_SOURCE: &str = "named";

/// Platform session identities, checked in order after [`DUCTUS_SESSION_VAR`]:
/// the environment variable an agent passes to the processes it spawns, and
/// the source label recorded for it.
///
/// One row per agent whose variable has been **verified**, never guessed.
/// Claude Code places `CLAUDE_CODE_SESSION_ID` in both its MCP servers' and
/// its shell tool's environment (spec 062, Resolved Questions, grounded
/// 2026-09-29). An agent with no row still gets isolation through
/// `DUCTUS_SESSION`.
pub const PLATFORM_IDENTITIES: &[(&str, &str)] = &[("CLAUDE_CODE_SESSION_ID", "claude-code")];

/// How many characters of a platform key a display label shows.
const PLATFORM_LABEL_CHARS: usize = 8;

/// One process's session identity: the sanitized key that names its file, and
/// where the identity came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    /// The sanitized identity — the stem of `.ductus/sessions/{key}.toml`.
    /// Always matches the slug grammar, so it can never escape the directory.
    pub key: String,
    /// [`NAMED_SOURCE`] or a platform source from [`PLATFORM_IDENTITIES`].
    pub source: String,
}

impl Identity {
    /// A named identity, sanitized with the slug rule. `None` when the name
    /// sanitizes to nothing.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        Self::from_raw(name, NAMED_SOURCE)
    }

    fn from_raw(raw: &str, source: &str) -> Option<Self> {
        let key = derive_slug(raw.trim());
        (!key.is_empty()).then(|| Self {
            key,
            source: source.to_owned(),
        })
    }

    /// The label a notice shows an operator: the name for a named session, and
    /// the source plus the key's first characters for a platform one — a UUID
    /// in full is noise, and its prefix is enough to tell sessions apart.
    #[must_use]
    pub fn label(&self) -> String {
        if self.source == NAMED_SOURCE {
            self.key.clone()
        } else {
            let prefix: String = self.key.chars().take(PLATFORM_LABEL_CHARS).collect();
            format!("{}:{prefix}", self.source)
        }
    }
}

/// Resolve the session identity from an environment `lookup`.
///
/// In order: an operator-set [`DUCTUS_SESSION_VAR`]; otherwise the first
/// [`PLATFORM_IDENTITIES`] variable present; otherwise `None`, and the process
/// uses the shared default exactly as before spec 062.
///
/// An empty `DUCTUS_SESSION` is unset. A value is sanitized with the slug rule
/// before use, so it can never name a path outside the sessions directory.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] naming `DUCTUS_SESSION` when its
/// value sanitizes to nothing: the operator asked for a session and would
/// otherwise silently get none. A platform value that sanitizes to nothing is
/// passed over instead — it is not the operator's to fix.
pub fn identity_from_env(lookup: impl Fn(&str) -> Option<String>) -> Result<Option<Identity>> {
    if let Some(raw) = lookup(DUCTUS_SESSION_VAR)
        && !raw.trim().is_empty()
    {
        return Identity::named(&raw)
            .map(Some)
            .ok_or_else(|| PrimitiveError::InvalidArgument {
                primitive: "session".into(),
                argument: DUCTUS_SESSION_VAR.into(),
                reason: format!(
                    "{raw:?} sanitizes to an empty session name (no ASCII letters or \
                     digits); set a name such as `review`, or unset the variable"
                ),
            });
    }
    Ok(PLATFORM_IDENTITIES
        .iter()
        .find_map(|(var, source)| lookup(var).and_then(|raw| Identity::from_raw(&raw, source))))
}

/// [`identity_from_env`] against this process's own environment — the one call
/// the edges make.
///
/// # Errors
///
/// As [`identity_from_env`].
pub fn identity_from_process_env() -> Result<Option<Identity>> {
    identity_from_env(|var| std::env::var(var).ok())
}

/// Whether this repo can hold per-process targets: only on the `.ductus/`
/// layout. When the active session file is a legacy tier, creating
/// `.ductus/sessions/` would put `.ductus/` state beside a lingering legacy
/// file, which 042's cutover rule forbids ([`paths::session_path_for_write`]);
/// the process is treated as unidentified until `/ductus` migrates the repo.
#[must_use]
pub fn per_process_enabled(repo: &Path) -> bool {
    paths::session_path_for_write(repo) == repo.join(paths::SESSION_FILE)
}

/// The identity to act under in `repo`: `identity`, unless the layout cannot
/// hold per-process targets.
#[must_use]
pub fn effective<'a>(repo: &Path, identity: Option<&'a Identity>) -> Option<&'a Identity> {
    identity.filter(|_| per_process_enabled(repo))
}

/// `.ductus/sessions/` under `repo`.
#[must_use]
pub fn sessions_dir(repo: &Path) -> PathBuf {
    repo.join(paths::SESSIONS_DIR)
}

/// The per-process file for `identity` under `repo`.
#[must_use]
pub fn own_path(repo: &Path, identity: &Identity) -> PathBuf {
    sessions_dir(repo).join(format!("{}.toml", identity.key))
}

/// A pending removal notice: what a fold or consolidation did to a session's
/// target, delivered by that session's next resolution.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RemovalNotice {
    /// `fold` or `consolidate`.
    pub cause: String,
    /// The removed feature the session had targeted.
    pub from: String,
    /// The feature the session now targets; absent for a consolidation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Label of the session whose command removed the target.
    pub by: String,
}

/// On-disk shape of `.ductus/sessions/{key}.toml` (spec 062 data model).
///
/// Field order is the serialized order. `notice` is last because it is a TOML
/// table, and a table must follow every plain key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcessRecord {
    /// [`NAMED_SOURCE`] or a platform source; used to render the label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Targeted feature; absent when cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// Repo-relative spec directory; absent when cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Targeted scenario slug, when one is targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    /// Repo-relative scenario file, with `scenario`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario_path: Option<String>,
    /// When this process last wrote its target (ISO 8601 UTC).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_at: Option<String>,
    /// When this process last resolved or wrote its target; drives expiry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_at: Option<String>,
    /// The process cleared its target: it has none, and never adopts the
    /// default in its place.
    #[serde(default, skip_serializing_if = "is_false")]
    pub cleared: bool,
    /// Labels of the sessions sharing this feature that this session was last
    /// told about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seen_peers: Vec<String>,
    /// A removal notice not yet delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice: Option<RemovalNotice>,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde's `skip_serializing_if` passes a reference
fn is_false(value: &bool) -> bool {
    !*value
}

impl ProcessRecord {
    /// Read and parse a per-process file.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::Io`] or [`PrimitiveError::Toml`], each naming the
    /// file: an unreadable record is reported, never treated as absent (AC22).
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|source| PrimitiveError::Io {
            path: path.into(),
            source,
        })?;
        toml::from_str(&text).map_err(|source| PrimitiveError::Toml {
            path: path.into(),
            source,
        })
    }

    /// Atomically write this record to `path`.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::Io`] on a failed write.
    ///
    /// # Panics
    ///
    /// Never in practice: the record holds only strings, a bool, a string
    /// list and a table of strings, none of which TOML cannot represent.
    pub fn store(&self, path: &Path) -> Result<()> {
        // A struct of strings, bools, a string list and a string table has no
        // unrepresentable value, so serialization cannot fail.
        #[allow(clippy::expect_used)]
        let body = toml::to_string(self).expect("process record serializes infallibly");
        write_atomic(path, &body)
    }
}

/// Create `.ductus/sessions/` when absent, with a `.gitignore` of `*` so the
/// per-process files never show as untracked — whatever the state of the
/// repository's own `.gitignore` (spec 062 plan, §Gitignore).
///
/// # Errors
///
/// [`PrimitiveError::Io`] on a failed create or write.
pub fn ensure_sessions_dir(repo: &Path) -> Result<PathBuf> {
    let dir = sessions_dir(repo);
    std::fs::create_dir_all(&dir).map_err(|source| PrimitiveError::Io {
        path: dir.clone(),
        source,
    })?;
    let ignore = dir.join(".gitignore");
    if !ignore.exists() {
        write_atomic(&ignore, "*\n")?;
    }
    Ok(dir)
}

/// An exclusive hold on the session lock, released when dropped.
#[derive(Debug)]
pub struct SessionLock {
    _file: File,
}

/// Take the exclusive advisory lock on `.ductus/sessions/.lock`, creating the
/// directory first. Blocks until any other holder releases it; the OS releases
/// it if the holder dies, so there is no stale-lock recovery to get wrong.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when the lock file cannot be opened or locked.
pub fn lock(repo: &Path) -> Result<SessionLock> {
    ensure_sessions_dir(repo)?;
    let path = repo.join(paths::SESSIONS_LOCK);
    let io = |source| PrimitiveError::Io {
        path: path.clone(),
        source,
    };
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&path)
        .map_err(io)?;
    file.lock().map_err(io)?;
    Ok(SessionLock { _file: file })
}

/// Take the lock only when per-process state exists to race with.
///
/// An unidentified process in a repo nobody has used per-process targets in
/// writes the default exactly as before spec 062 — atomically, last writer
/// wins — and creates nothing (AC3).
///
/// # Errors
///
/// As [`lock`].
pub fn lock_if_shared(repo: &Path) -> Result<Option<SessionLock>> {
    if sessions_dir(repo).is_dir() {
        lock(repo).map(Some)
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn ductus_session_takes_precedence_over_the_platform_identity() {
        let id = identity_from_env(env(&[
            ("DUCTUS_SESSION", "review"),
            (
                "CLAUDE_CODE_SESSION_ID",
                "3f2a9c1d-0000-4000-8000-000000000000",
            ),
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(id.key, "review");
        assert_eq!(id.source, NAMED_SOURCE);
    }

    #[test]
    fn two_processes_with_the_same_name_share_one_identity() {
        let a = identity_from_env(env(&[("DUCTUS_SESSION", "Review")])).unwrap();
        let b = identity_from_env(env(&[("DUCTUS_SESSION", "review")])).unwrap();
        assert_eq!(a, b);
        assert_eq!(
            own_path(Path::new("/r"), &a.unwrap()),
            PathBuf::from("/r/.ductus/sessions/review.toml")
        );
    }

    #[test]
    fn claude_code_session_id_identifies_a_process_with_no_name() {
        let id = identity_from_env(env(&[(
            "CLAUDE_CODE_SESSION_ID",
            "3F2A9C1D-0000-4000-8000-000000000000",
        )]))
        .unwrap()
        .unwrap();
        assert_eq!(id.key, "3f2a9c1d-0000-4000-8000-000000000000");
        assert_eq!(id.source, "claude-code");
        assert_eq!(id.label(), "claude-code:3f2a9c1d");
    }

    #[test]
    fn distinct_platform_ids_are_distinct_identities() {
        let a = identity_from_env(env(&[("CLAUDE_CODE_SESSION_ID", "aaaa-1")])).unwrap();
        let b = identity_from_env(env(&[("CLAUDE_CODE_SESSION_ID", "bbbb-2")])).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn empty_ductus_session_is_unset() {
        let id = identity_from_env(env(&[
            ("DUCTUS_SESSION", "   "),
            ("CLAUDE_CODE_SESSION_ID", "abc"),
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(id.source, "claude-code", "falls through to the platform");
        assert_eq!(
            identity_from_env(env(&[("DUCTUS_SESSION", "")])).unwrap(),
            None
        );
    }

    #[test]
    fn no_identity_at_all_is_none() {
        assert_eq!(identity_from_env(env(&[])).unwrap(), None);
    }

    #[test]
    fn a_name_is_sanitized_by_the_slug_rule() {
        let id = identity_from_env(env(&[("DUCTUS_SESSION", "../My Review!")]))
            .unwrap()
            .unwrap();
        assert_eq!(id.key, "my-review");
        assert_eq!(id.label(), "my-review");
    }

    #[test]
    fn a_name_that_sanitizes_to_nothing_is_refused_naming_the_variable() {
        let err = identity_from_env(env(&[("DUCTUS_SESSION", "!!!")])).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("DUCTUS_SESSION"), "{message}");
        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
    }

    #[test]
    fn a_platform_value_that_sanitizes_to_nothing_is_passed_over() {
        assert_eq!(
            identity_from_env(env(&[("CLAUDE_CODE_SESSION_ID", "---")])).unwrap(),
            None
        );
    }

    #[test]
    fn per_process_targets_need_the_ductus_layout() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        assert!(per_process_enabled(repo), "a fresh project cuts over");

        std::fs::write(repo.join(".govern.session.toml"), "feature = \"x\"\n").unwrap();
        assert!(!per_process_enabled(repo), "legacy root tier is active");
        let id = Identity::named("review").unwrap();
        assert_eq!(effective(repo, Some(&id)), None);

        std::fs::create_dir_all(repo.join(".ductus")).unwrap();
        std::fs::write(repo.join(".ductus/session.toml"), "").unwrap();
        assert!(
            per_process_enabled(repo),
            "the .ductus tier wins once it exists"
        );
        assert_eq!(effective(repo, Some(&id)), Some(&id));
    }

    #[test]
    fn the_sessions_directory_ignores_itself() {
        let tmp = tempdir().unwrap();
        let _held = lock(tmp.path()).unwrap();
        let ignore =
            std::fs::read_to_string(tmp.path().join(".ductus/sessions/.gitignore")).unwrap();
        assert_eq!(ignore, "*\n");
    }

    #[test]
    fn lock_if_shared_creates_nothing_in_an_unused_repo() {
        let tmp = tempdir().unwrap();
        assert!(lock_if_shared(tmp.path()).unwrap().is_none());
        assert!(!tmp.path().join(".ductus/sessions").exists());
    }

    #[test]
    fn a_process_record_round_trips_with_its_notice_table_last() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("r.toml");
        let record = ProcessRecord {
            source: Some("named".into()),
            feature: Some("055-x".into()),
            path: Some("specs/055-x".into()),
            used_at: Some("2026-09-29T00:00:00Z".into()),
            seen_peers: vec!["a".into()],
            notice: Some(RemovalNotice {
                cause: "fold".into(),
                from: "1.1-y".into(),
                to: Some("055-x".into()),
                by: "b".into(),
            }),
            ..ProcessRecord::default()
        };
        record.store(&path).unwrap();
        assert_eq!(ProcessRecord::load(&path).unwrap(), record);
    }

    #[test]
    fn an_unparseable_record_names_its_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("bad.toml");
        std::fs::write(&path, "feature = [unterminated").unwrap();
        let err = ProcessRecord::load(&path).unwrap_err();
        assert!(err.to_string().contains("bad.toml"), "{err}");
    }
}
