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
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::primitives::{PrimitiveError, Result, derive_slug, write_atomic};
use crate::schema::paths;
use crate::schema::primitives::{
    SessionNotice, SessionNoticeKind, SessionPeer, SessionSource, SessionTarget,
};

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

// -- time ---------------------------------------------------------------------

/// How long a per-process target may go neither resolved nor written before
/// the next write removes it: seven days (spec 062, Resolved Questions).
pub const IDLE_EXPIRY: Duration = Duration::from_secs(7 * 86_400);

/// Format `now` as an RFC 3339 / ISO 8601 UTC timestamp
/// (`YYYY-MM-DDTHH:MM:SSZ`) — the shape `set-at` has always had.
///
/// Uses Howard Hinnant's date algorithms — the standard branchless
/// civil-from-days computation. A `now` earlier than the epoch falls back to
/// `1970-01-01T00:00:00Z`, which a session file never produces in practice.
#[must_use]
pub fn iso8601_utc(now: SystemTime) -> String {
    let secs = now.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let days = secs / 86_400;
    let tod = secs % 86_400;
    let hour = tod / 3600;
    let min = (tod / 60) % 60;
    let sec = tod % 60;

    // `days` from a post-1970 SystemTime fits in i64 with enormous headroom.
    #[allow(clippy::cast_possible_wrap)]
    let (year, month, day) = civil_from_days(days as i64);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

/// Parse a timestamp [`iso8601_utc`] wrote back into a `SystemTime`. `None`
/// for any other shape: a hand-edited or foreign value is not guessed at.
#[must_use]
pub fn parse_iso8601_utc(text: &str) -> Option<SystemTime> {
    let b = text.as_bytes();
    let shape_ok = b.len() == 20
        && b[4] == b'-'
        && b[7] == b'-'
        && b[10] == b'T'
        && b[13] == b':'
        && b[16] == b':'
        && b[19] == b'Z';
    if !shape_ok {
        return None;
    }
    let num = |range: std::ops::Range<usize>| text.get(range)?.parse::<u32>().ok();
    let (year, month, day) = (num(0..4)?, num(5..7)?, num(8..10)?);
    let (hour, min, sec) = (num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || min > 59 || sec > 59 {
        return None;
    }
    let days = u64::try_from(days_from_civil(i64::from(year), month, day)).ok()?;
    let secs = days * 86_400 + u64::from(hour) * 3600 + u64::from(min) * 60 + u64::from(sec);
    Some(UNIX_EPOCH + Duration::from_secs(secs))
}

/// Convert days-since-1970-01-01 (Gregorian) into `(year, month, day)`.
///
/// Howard Hinnant's standard civil-from-days algorithm. The intermediate
/// casts are part of the algorithm and safe for any post-1970, pre-year-9999
/// input a session file will ever produce.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = y + i64::from(month <= 2);
    (year, month as u32, day as u32)
}

/// The inverse of [`civil_from_days`]: Howard Hinnant's days-from-civil.
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = year - i64::from(month <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = i64::from((month + 9) % 12);
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Whether a target last used at `used_at` has been idle past
/// [`IDLE_EXPIRY`] at `now`. `None` when `used_at` is missing or unparseable:
/// idleness that cannot be established is never assumed.
fn idle_past_expiry(used_at: Option<&str>, now: SystemTime) -> Option<bool> {
    let used = parse_iso8601_utc(used_at?)?;
    Some(
        now.duration_since(used)
            .is_ok_and(|idle| idle > IDLE_EXPIRY),
    )
}

// -- the shared default ---------------------------------------------------------

/// On-disk shape of the shared default, `.ductus/session.toml`. Field order is
/// the wire contract — the parity byte-equality check depends on it — and is
/// unchanged by spec 062. `cli-config-dir` is serialized last so the target
/// block keeps its byte-for-byte order.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DefaultRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    feature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scenario: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scenario_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    set_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cli_config_dir: Option<String>,
}

impl DefaultRecord {
    fn target(&self) -> Option<SessionTarget> {
        Some(SessionTarget {
            feature: self.feature.clone()?,
            path: self.path.clone().unwrap_or_default(),
            scenario: self.scenario.clone(),
            scenario_path: self.scenario_path.clone(),
        })
    }

    fn set_target(&mut self, target: Option<&SessionTarget>, set_at: Option<String>) {
        self.feature = target.map(|t| t.feature.clone());
        self.path = target.map(|t| t.path.clone());
        self.scenario = target.and_then(|t| t.scenario.clone());
        self.scenario_path = target.and_then(|t| t.scenario_path.clone());
        self.set_at = set_at;
    }

    fn store(&self, path: &Path) -> Result<()> {
        // A struct of `Option<String>` has no unrepresentable value.
        #[allow(clippy::expect_used)]
        let body = toml::to_string(self).expect("session TOML serializes infallibly");
        write_atomic(path, &body)
    }
}

/// Best-effort read for a *write*: a missing or malformed default yields an
/// empty record, so a write simply has nothing to preserve rather than failing
/// — the write replaces the broken file, as it always has.
fn read_default_lenient(path: &Path) -> DefaultRecord {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default()
}

/// Strict read for a *resolution*: `None` when no default exists, an error
/// naming the file when it does not parse — reported, never treated as absent
/// (AC22), exactly as `dashboard` has always treated it.
fn read_default_strict(repo: &Path) -> Result<Option<DefaultRecord>> {
    let path = paths::session_path(repo);
    if !path.is_file() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path).map_err(|source| PrimitiveError::Io {
        path: path.clone(),
        source,
    })?;
    toml::from_str(&content)
        .map(Some)
        .map_err(|source| PrimitiveError::Toml { path, source })
}

// -- per-process files ----------------------------------------------------------

impl ProcessRecord {
    fn target(&self) -> Option<SessionTarget> {
        if self.cleared {
            return None;
        }
        Some(SessionTarget {
            feature: self.feature.clone()?,
            path: self.path.clone().unwrap_or_default(),
            scenario: self.scenario.clone(),
            scenario_path: self.scenario_path.clone(),
        })
    }

    fn set_target(&mut self, target: Option<&SessionTarget>) {
        self.feature = target.map(|t| t.feature.clone());
        self.path = target.map(|t| t.path.clone());
        self.scenario = target.and_then(|t| t.scenario.clone());
        self.scenario_path = target.and_then(|t| t.scenario_path.clone());
        self.cleared = target.is_none();
    }
}

/// Every per-process file under `repo`, as `(identity, path)`, sorted by key
/// so every scan visits them in one order. Dotfiles — the lock and the
/// `.gitignore` — are not sessions.
fn session_files(repo: &Path) -> Result<Vec<(Identity, PathBuf)>> {
    let dir = sessions_dir(repo);
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(PrimitiveError::Io { path: dir, source }),
    };
    let mut files: Vec<(Identity, PathBuf)> = entries
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .filter_map(|path| {
            let key = path.file_stem()?.to_str()?.to_owned();
            (!key.starts_with('.')).then(|| {
                (
                    Identity {
                        key,
                        source: NAMED_SOURCE.to_owned(),
                    },
                    path,
                )
            })
        })
        .collect();
    files.sort_by(|a, b| a.0.key.cmp(&b.0.key));
    Ok(files)
}

impl Identity {
    /// Recover an identity from a per-process file's stem and recorded source.
    fn with_source(mut self, source: Option<&str>) -> Self {
        if let Some(source) = source {
            source.clone_into(&mut self.source);
        }
        self
    }
}

/// Other unexpired sessions whose target names `feature`, and the files that
/// could not be parsed on the way. `own` is excluded.
fn peers_of(
    repo: &Path,
    own: Option<&Identity>,
    feature: &str,
    now: SystemTime,
) -> Result<(Vec<SessionPeer>, Vec<PathBuf>)> {
    let mut peers = Vec::new();
    let mut unreadable = Vec::new();
    for (identity, path) in session_files(repo)? {
        if own.is_some_and(|own| own.key == identity.key) {
            continue;
        }
        let Ok(record) = ProcessRecord::load(&path) else {
            unreadable.push(path);
            continue;
        };
        let Some(target) = record.target() else {
            continue;
        };
        if target.feature != feature
            || idle_past_expiry(record.used_at.as_deref(), now) == Some(true)
        {
            continue;
        }
        peers.push(SessionPeer {
            session: identity.with_source(record.source.as_deref()).label(),
            feature: target.feature,
            scenario: target.scenario,
            last_used: record.used_at,
        });
    }
    Ok((peers, unreadable))
}

/// Remove every per-process file idle past [`IDLE_EXPIRY`], never `keep`'s.
/// Returns the removed sessions' labels and the files that could not be
/// examined — unparseable, or with no parseable `used-at` — which are left in
/// place: a file that cannot be read cannot be proven idle.
///
/// Callers hold the lock, and resolutions refresh `used-at` under the same
/// lock, so a target used after this read its `used-at` cannot be removed
/// (AC18).
fn sweep(
    repo: &Path,
    keep: Option<&Identity>,
    now: SystemTime,
) -> Result<(Vec<String>, Vec<PathBuf>)> {
    let mut expired = Vec::new();
    let mut unreadable = Vec::new();
    for (identity, path) in session_files(repo)? {
        if keep.is_some_and(|keep| keep.key == identity.key) {
            continue;
        }
        let Ok(record) = ProcessRecord::load(&path) else {
            unreadable.push(path);
            continue;
        };
        match idle_past_expiry(record.used_at.as_deref(), now) {
            Some(true) => {
                std::fs::remove_file(&path).map_err(|source| PrimitiveError::Io {
                    path: path.clone(),
                    source,
                })?;
                expired.push(identity.with_source(record.source.as_deref()).label());
            }
            Some(false) => {}
            None => unreadable.push(path),
        }
    }
    Ok((expired, unreadable))
}

// -- notices --------------------------------------------------------------------

fn adopted_notice(identity: &Identity, target: &SessionTarget) -> SessionNotice {
    SessionNotice {
        kind: SessionNoticeKind::Adopted,
        message: format!(
            "Session {} had no target of its own and adopted {} from the shared default.",
            identity.label(),
            target.display()
        ),
    }
}

fn co_target_notice(feature: &str, peers: &[SessionPeer]) -> SessionNotice {
    let described: Vec<String> = peers
        .iter()
        .map(|peer| match &peer.last_used {
            Some(when) => format!("{} (last used {when})", peer.session),
            None => peer.session.clone(),
        })
        .collect();
    let (noun, verb) = if peers.len() == 1 {
        ("Session", "also targets")
    } else {
        ("Sessions", "also target")
    };
    SessionNotice {
        kind: SessionNoticeKind::CoTarget,
        message: format!("{noun} {} {verb} {feature}.", described.join(", ")),
    }
}

/// The `by` recorded when the acting process is unidentified.
const UNIDENTIFIED_ACTOR: &str = "unidentified";

fn describe_actor(by: &str) -> String {
    if by == UNIDENTIFIED_ACTOR {
        "an unidentified session".to_owned()
    } else {
        format!("session {by}")
    }
}

fn removal_notice(notice: &RemovalNotice) -> SessionNotice {
    let by = describe_actor(&notice.by);
    match &notice.to {
        Some(to) => SessionNotice {
            kind: SessionNoticeKind::Folded,
            message: format!(
                "Target {} was folded into {to} by {by}; this session now targets {to}.",
                notice.from
            ),
        },
        None => SessionNotice {
            kind: SessionNoticeKind::Consolidated,
            message: format!(
                "Target {} was removed by a consolidation in {by}; this session's target is \
                 cleared — set a new one with the target command.",
                notice.from
            ),
        },
    }
}

/// Recompute `record`'s peers on `feature`, and notice them when the set
/// differs from the one this session was last told about (AC19). A set that
/// empties is recorded silently: nobody left to name.
fn refresh_peers(
    repo: &Path,
    identity: &Identity,
    record: &mut ProcessRecord,
    feature: &str,
    now: SystemTime,
    notices: &mut Vec<SessionNotice>,
) -> Result<()> {
    let (peers, _unreadable) = peers_of(repo, Some(identity), feature, now)?;
    let labels: Vec<String> = peers.iter().map(|peer| peer.session.clone()).collect();
    if labels != record.seen_peers {
        if !peers.is_empty() {
            notices.push(co_target_notice(feature, &peers));
        }
        record.seen_peers = labels;
    }
    Ok(())
}

// -- resolve --------------------------------------------------------------------

/// The outcome of resolving a process's target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolution {
    /// The acting identity's display label; `None` when unidentified.
    pub identity: Option<String>,
    /// Where the target came from.
    pub source: SessionSource,
    /// The resolved target; `None` for [`SessionSource::Cleared`] and
    /// [`SessionSource::None`].
    pub target: Option<SessionTarget>,
    /// Notices to display once.
    pub notices: Vec<SessionNotice>,
}

/// Resolve the target of the process acting as `identity` in `repo`.
///
/// Unidentified (or on a legacy layout): the shared default, read-only. An
/// identified process with a file of its own gets its own target — or none,
/// when cleared — with `used-at` refreshed, a pending removal notice delivered
/// and removed, and a co-target notice when its peers changed. One with no file
/// adopts the shared default, pins it as its own and says so (AC17); with no
/// default there is nothing to adopt and nothing is written.
///
/// # Errors
///
/// [`PrimitiveError::Toml`] naming the file when the process's own file, or the
/// default it would adopt, does not parse — never adoption in its place (AC22);
/// [`PrimitiveError::Io`] on a failed read, lock or write.
pub fn resolve(repo: &Path, identity: Option<&Identity>, now: SystemTime) -> Result<Resolution> {
    let Some(identity) = effective(repo, identity) else {
        return resolve_unidentified(repo);
    };

    let _lock = lock(repo)?;
    let own = own_path(repo, identity);
    let mut notices = Vec::new();

    if own.exists() {
        let mut record = ProcessRecord::load(&own)?;
        if let Some(notice) = record.notice.take() {
            notices.push(removal_notice(&notice));
        }
        let target = record.target();
        match &target {
            Some(target) => {
                refresh_peers(
                    repo,
                    identity,
                    &mut record,
                    &target.feature,
                    now,
                    &mut notices,
                )?;
            }
            None => record.seen_peers.clear(),
        }
        record.used_at = Some(iso8601_utc(now));
        record.store(&own)?;
        return Ok(Resolution {
            identity: Some(identity.label()),
            source: if target.is_some() {
                SessionSource::Own
            } else {
                SessionSource::Cleared
            },
            target,
            notices,
        });
    }

    let default = read_default_strict(repo)?;
    let Some(target) = default.as_ref().and_then(DefaultRecord::target) else {
        return Ok(Resolution {
            identity: Some(identity.label()),
            source: SessionSource::None,
            target: None,
            notices,
        });
    };
    let mut record = ProcessRecord {
        source: Some(identity.source.clone()),
        set_at: default.and_then(|d| d.set_at),
        used_at: Some(iso8601_utc(now)),
        ..ProcessRecord::default()
    };
    record.set_target(Some(&target));
    notices.push(adopted_notice(identity, &target));
    refresh_peers(
        repo,
        identity,
        &mut record,
        &target.feature,
        now,
        &mut notices,
    )?;
    record.store(&own)?;
    Ok(Resolution {
        identity: Some(identity.label()),
        source: SessionSource::Adopted,
        target: Some(target),
        notices,
    })
}

/// An unidentified process's resolution: the shared default, read-only.
fn resolve_unidentified(repo: &Path) -> Result<Resolution> {
    let target = read_default_strict(repo)?.and_then(|record| record.target());
    Ok(Resolution {
        identity: None,
        source: if target.is_some() {
            SessionSource::Default
        } else {
            SessionSource::None
        },
        target,
        notices: Vec::new(),
    })
}

/// What [`resolve`] would answer for `identity`, with **nothing written and
/// nothing consumed**: no adoption pinned, no `used-at` refreshed, no notice
/// delivered, and no sessions directory created.
///
/// For a caller that needs the target before the command that owns the
/// resolution has run — the `ductus exec` seed. Were the seed to resolve for
/// real, it would consume the once-only notices and pin the adoption before
/// the walked command's own `resolve-session` step could report them, and the
/// operator would never see either. A process with no file of its own peeks
/// the default with source `adopted`, since that is what its first real
/// resolution will pin.
///
/// # Errors
///
/// As [`resolve`], minus the write failures it cannot have.
pub fn peek(repo: &Path, identity: Option<&Identity>) -> Result<Resolution> {
    let Some(identity) = effective(repo, identity) else {
        return resolve_unidentified(repo);
    };
    let _lock = lock_if_shared(repo)?;
    let own = own_path(repo, identity);
    if own.exists() {
        let target = ProcessRecord::load(&own)?.target();
        return Ok(Resolution {
            identity: Some(identity.label()),
            source: if target.is_some() {
                SessionSource::Own
            } else {
                SessionSource::Cleared
            },
            target,
            notices: Vec::new(),
        });
    }
    let target = read_default_strict(repo)?.and_then(|record| record.target());
    Ok(Resolution {
        identity: Some(identity.label()),
        source: if target.is_some() {
            SessionSource::Adopted
        } else {
            SessionSource::None
        },
        target,
        notices: Vec::new(),
    })
}

// -- retarget -------------------------------------------------------------------

/// Why a feature directory was removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemovalCause {
    /// A fold moved the content upstream: sessions follow it.
    Fold,
    /// A consolidation removed a spec nobody chose to work on: sessions are
    /// cleared.
    Consolidate,
}

impl RemovalCause {
    fn as_str(self) -> &'static str {
        match self {
            Self::Fold => "fold",
            Self::Consolidate => "consolidate",
        }
    }
}

/// What a removal did to the working tree's sessions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RetargetOutcome {
    /// Labels of the sessions re-pointed; `default` for the shared default.
    pub retargeted: Vec<String>,
    /// Labels of the sessions cleared; `default` for the shared default.
    pub cleared: Vec<String>,
    /// Files that could not be parsed, left in place — a removal cannot prove
    /// they do not name the removed feature.
    pub unreadable: Vec<PathBuf>,
}

/// Re-point (`to` supplied) or clear (`to` absent) every session in `repo`
/// whose target names `from`, and the shared default when it does (AC5,
/// AC21). §concurrent-features' removal rule, extended from the acting
/// session to every session the working tree holds.
///
/// Every session but the acting one gets a pending notice naming the cause
/// and the acting session, delivered by its next resolution. Sessions naming
/// another feature are not written at all. `set-at` and `used-at` are left
/// alone: this is not the other session's own write or use.
///
/// # Errors
///
/// [`PrimitiveError::Io`] on a failed lock, read or write.
pub fn retarget(
    repo: &Path,
    identity: Option<&Identity>,
    from: &str,
    to: Option<&SessionTarget>,
    cause: RemovalCause,
    now: SystemTime,
) -> Result<RetargetOutcome> {
    let identity = effective(repo, identity);
    let _lock = lock_if_shared(repo)?;
    let mut outcome = RetargetOutcome::default();
    let mut record_outcome = |label: String| {
        if to.is_some() {
            outcome.retargeted.push(label);
        } else {
            outcome.cleared.push(label);
        }
    };

    let default_path = paths::session_path_for_write(repo);
    if default_path.is_file() {
        match read_default_strict(repo) {
            Ok(Some(mut default)) if default.feature.as_deref() == Some(from) => {
                default.set_target(to, to.map(|_| iso8601_utc(now)));
                default.store(&default_path)?;
                record_outcome("default".to_owned());
            }
            Ok(_) => {}
            Err(_) => outcome.unreadable.push(default_path),
        }
    }

    let actor = identity.map_or_else(|| UNIDENTIFIED_ACTOR.to_owned(), Identity::label);
    for (file_identity, path) in session_files(repo)? {
        let Ok(mut record) = ProcessRecord::load(&path) else {
            outcome.unreadable.push(path);
            continue;
        };
        if record.target().is_none_or(|target| target.feature != from) {
            continue;
        }
        record.set_target(to);
        let label = file_identity.with_source(record.source.as_deref()).label();
        if identity.is_none_or(|acting| acting.key != file_identity_key(&path)) {
            record.notice = Some(RemovalNotice {
                cause: cause.as_str().to_owned(),
                from: from.to_owned(),
                to: to.map(|t| t.feature.clone()),
                by: actor.clone(),
            });
        }
        record.store(&path)?;
        record_outcome(label);
    }
    Ok(outcome)
}

/// The identity key a per-process file belongs to: its stem.
fn file_identity_key(path: &Path) -> &str {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
}

// -- write ----------------------------------------------------------------------

/// The three write shapes `write-session` has always had.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteShape {
    /// Set the target.
    Target(SessionTarget),
    /// Remove the target.
    Clear,
    /// Set only `cli-config-dir`, preserving the default's target.
    HostConfig,
}

/// What a write did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriteOutcome {
    /// The shared default written.
    pub default_path: PathBuf,
    /// `true` when the default did not exist before the write.
    pub created: bool,
    /// The acting identity's display label; `None` when unidentified.
    pub identity: Option<String>,
    /// The per-process file written, when the process is identified.
    pub own_path: Option<PathBuf>,
    /// Other sessions targeting the written feature.
    pub peers: Vec<SessionPeer>,
    /// Labels of per-process targets the expiry sweep removed.
    pub expired: Vec<String>,
    /// Per-process files the write could not examine, left in place.
    pub unreadable: Vec<PathBuf>,
}

/// Write the target of the process acting as `identity` in `repo`.
///
/// A target or clear write updates the shared default exactly as before spec
/// 062 — preserving `cli-config-dir` unless `cli_config_dir` overrides it — and,
/// for an identified process, its own file too, so the default always holds the
/// working tree's most recent target change (AC16). A clear leaves the process
/// with no target that never adopts the default (AC20). A host-config write
/// touches the default alone. Target and clear writes then run the expiry sweep.
///
/// # Errors
///
/// [`PrimitiveError::Io`] on a failed lock, read or write.
pub fn write(
    repo: &Path,
    identity: Option<&Identity>,
    shape: &WriteShape,
    cli_config_dir: Option<String>,
    now: SystemTime,
) -> Result<WriteOutcome> {
    let identity = effective(repo, identity);
    let lock = match identity {
        Some(_) => Some(lock(repo)?),
        None => lock_if_shared(repo)?,
    };

    let default_path = paths::session_path_for_write(repo);
    let created = !default_path.exists();
    let mut default = read_default_lenient(&default_path);
    let stamp = iso8601_utc(now);

    let mut outcome = WriteOutcome {
        default_path: default_path.clone(),
        created,
        identity: identity.map(Identity::label),
        own_path: None,
        peers: Vec::new(),
        expired: Vec::new(),
        unreadable: Vec::new(),
    };

    match shape {
        WriteShape::Target(target) => {
            default.set_target(Some(target), Some(stamp.clone()));
            if let Some(identity) = identity {
                let (peers, unreadable) = peers_of(repo, Some(identity), &target.feature, now)?;
                let mut record = ProcessRecord {
                    source: Some(identity.source.clone()),
                    set_at: Some(stamp.clone()),
                    used_at: Some(stamp),
                    seen_peers: peers.iter().map(|peer| peer.session.clone()).collect(),
                    ..ProcessRecord::default()
                };
                record.set_target(Some(target));
                let own = own_path(repo, identity);
                record.store(&own)?;
                outcome.own_path = Some(own);
                outcome.peers = peers;
                outcome.unreadable = unreadable;
            }
        }
        WriteShape::Clear => {
            default.set_target(None, None);
            if let Some(identity) = identity {
                let mut record = ProcessRecord {
                    source: Some(identity.source.clone()),
                    used_at: Some(stamp),
                    ..ProcessRecord::default()
                };
                record.set_target(None);
                let own = own_path(repo, identity);
                record.store(&own)?;
                outcome.own_path = Some(own);
            }
        }
        WriteShape::HostConfig => {}
    }
    if let Some(dir) = cli_config_dir {
        default.cli_config_dir = Some(dir);
    } else if *shape == WriteShape::HostConfig {
        // A host-config write with no value records none, as it always has.
        default.cli_config_dir = None;
    }
    default.store(&default_path)?;

    if lock.is_some() && *shape != WriteShape::HostConfig {
        let (expired, unreadable) = sweep(repo, identity, now)?;
        outcome.expired = expired;
        for path in unreadable {
            if !outcome.unreadable.contains(&path) {
                outcome.unreadable.push(path);
            }
        }
    }
    Ok(outcome)
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

    // -- resolve, write, sweep (task 2) --------------------------------------

    fn t0() -> SystemTime {
        // 2026-09-29T12:00:00Z
        UNIX_EPOCH + Duration::from_secs(1_790_683_200)
    }

    fn at(secs: u64) -> SystemTime {
        t0() + Duration::from_secs(secs)
    }

    fn id(name: &str) -> Identity {
        Identity::named(name).unwrap()
    }

    fn target(feature: &str) -> SessionTarget {
        SessionTarget {
            feature: feature.into(),
            path: format!("specs/{feature}"),
            scenario: None,
            scenario_path: None,
        }
    }

    fn repo() -> tempfile::TempDir {
        let tmp = tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        tmp
    }

    fn set(repo: &Path, who: Option<&Identity>, feature: &str, now: SystemTime) -> WriteOutcome {
        write(repo, who, &WriteShape::Target(target(feature)), None, now).unwrap()
    }

    fn default_text(repo: &Path) -> String {
        std::fs::read_to_string(repo.join(".ductus/session.toml")).unwrap()
    }

    fn feature_of(resolution: &Resolution) -> Option<&str> {
        resolution.target.as_ref().map(|t| t.feature.as_str())
    }

    #[test]
    fn a_target_write_in_one_process_leaves_the_others_target_alone() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        set(tmp.path(), Some(&x), "055-a", t0());
        set(tmp.path(), Some(&y), "056-b", at(1));
        let rx = resolve(tmp.path(), Some(&x), at(2)).unwrap();
        let ry = resolve(tmp.path(), Some(&y), at(2)).unwrap();
        assert_eq!(
            (rx.source, feature_of(&rx)),
            (SessionSource::Own, Some("055-a"))
        );
        assert_eq!(
            (ry.source, feature_of(&ry)),
            (SessionSource::Own, Some("056-b"))
        );
    }

    #[test]
    fn a_process_adopts_the_default_once_and_is_pinned_to_it() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        set(tmp.path(), None, "055-a", t0());

        let first = resolve(tmp.path(), Some(&x), at(1)).unwrap();
        assert_eq!(first.source, SessionSource::Adopted);
        assert_eq!(feature_of(&first), Some("055-a"));
        assert_eq!(first.notices.len(), 1);
        assert_eq!(first.notices[0].kind, SessionNoticeKind::Adopted);
        assert!(
            first.notices[0].message.contains("055-a"),
            "{:?}",
            first.notices
        );

        set(tmp.path(), Some(&y), "056-b", at(2));
        assert!(default_text(tmp.path()).contains("056-b"), "default moved");
        let again = resolve(tmp.path(), Some(&x), at(3)).unwrap();
        assert_eq!(
            (again.source, feature_of(&again)),
            (SessionSource::Own, Some("055-a"))
        );
        assert!(again.notices.is_empty(), "adoption is announced once");
    }

    #[test]
    fn a_restarted_agent_resumes_the_most_recent_target() {
        let tmp = repo();
        set(tmp.path(), Some(&id("before-restart")), "055-a", t0());
        let after = resolve(tmp.path(), Some(&id("after-restart")), at(1)).unwrap();
        assert_eq!(feature_of(&after), Some("055-a"));
    }

    #[test]
    fn an_empty_default_pins_nothing() {
        let tmp = repo();
        let x = id("x");
        let r = resolve(tmp.path(), Some(&x), t0()).unwrap();
        assert_eq!((r.source, r.target), (SessionSource::None, None));
        assert!(!own_path(tmp.path(), &x).exists(), "nothing written");

        set(tmp.path(), None, "055-a", at(1));
        let later = resolve(tmp.path(), Some(&x), at(2)).unwrap();
        assert_eq!(
            later.source,
            SessionSource::Adopted,
            "nothing had been pinned"
        );
    }

    #[test]
    fn a_cleared_process_never_adopts_the_default() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        set(tmp.path(), Some(&x), "055-a", t0());
        write(tmp.path(), Some(&x), &WriteShape::Clear, None, at(1)).unwrap();
        set(tmp.path(), Some(&y), "056-b", at(2));
        let r = resolve(tmp.path(), Some(&x), at(3)).unwrap();
        assert_eq!((r.source, r.target), (SessionSource::Cleared, None));
    }

    #[test]
    fn the_default_always_holds_the_latest_target_change() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        set(tmp.path(), Some(&x), "055-a", t0());
        set(tmp.path(), Some(&y), "056-b", at(1));
        assert!(default_text(tmp.path()).contains("feature = \"056-b\""));
        write(tmp.path(), Some(&x), &WriteShape::Clear, None, at(2)).unwrap();
        assert!(
            !default_text(tmp.path()).contains("feature"),
            "a clear clears it"
        );
    }

    #[test]
    fn cli_config_dir_survives_every_write() {
        let tmp = repo();
        write(
            tmp.path(),
            None,
            &WriteShape::HostConfig,
            Some(".claude".into()),
            t0(),
        )
        .unwrap();
        let x = id("x");
        set(tmp.path(), Some(&x), "055-a", at(1));
        assert!(default_text(tmp.path()).contains("cli-config-dir = \".claude\""));
        write(tmp.path(), Some(&x), &WriteShape::Clear, None, at(2)).unwrap();
        assert!(default_text(tmp.path()).contains("cli-config-dir = \".claude\""));
        set(tmp.path(), None, "056-b", at(3));
        assert_eq!(
            default_text(tmp.path()),
            "feature = \"056-b\"\npath = \"specs/056-b\"\nset-at = \"2026-09-29T12:00:03Z\"\n\
             cli-config-dir = \".claude\"\n"
        );
    }

    #[test]
    fn an_unidentified_process_touches_only_the_default() {
        let tmp = repo();
        let out = set(tmp.path(), None, "055-a", t0());
        assert_eq!((out.identity, out.own_path), (None, None));
        assert!(
            !tmp.path().join(".ductus/sessions").exists(),
            "nothing created"
        );
        let r = resolve(tmp.path(), None, at(1)).unwrap();
        assert_eq!(
            (r.source, feature_of(&r)),
            (SessionSource::Default, Some("055-a"))
        );

        set(tmp.path(), None, "056-b", at(2));
        let shared = resolve(tmp.path(), None, at(3)).unwrap();
        assert_eq!(
            feature_of(&shared),
            Some("056-b"),
            "unidentified processes share it"
        );
    }

    #[test]
    fn co_targeting_is_noticed_once_per_side_and_again_when_the_set_changes() {
        let tmp = repo();
        let (x, y, z) = (id("x"), id("y"), id("z"));
        set(tmp.path(), Some(&x), "055-a", t0());

        let joined = set(tmp.path(), Some(&y), "055-a", at(1));
        assert_eq!(joined.peers.len(), 1, "the writer is told at write time");
        assert_eq!(joined.peers[0].session, "x");
        assert_eq!(
            joined.peers[0].last_used.as_deref(),
            Some("2026-09-29T12:00:00Z")
        );
        let writer_next = resolve(tmp.path(), Some(&y), at(2)).unwrap();
        assert!(
            writer_next.notices.is_empty(),
            "the writer is not told twice"
        );

        let told = resolve(tmp.path(), Some(&x), at(3)).unwrap();
        assert_eq!(told.notices.len(), 1);
        assert_eq!(told.notices[0].kind, SessionNoticeKind::CoTarget);
        assert!(told.notices[0].message.contains('y'), "{:?}", told.notices);
        let quiet = resolve(tmp.path(), Some(&x), at(4)).unwrap();
        assert!(quiet.notices.is_empty(), "told once");

        set(tmp.path(), Some(&z), "055-a", at(5));
        let changed = resolve(tmp.path(), Some(&x), at(6)).unwrap();
        assert_eq!(changed.notices.len(), 1, "the set changed");
        assert!(
            changed.notices[0].message.starts_with("Sessions y"),
            "{:?}",
            changed.notices
        );
    }

    #[test]
    fn a_target_idle_past_seven_days_is_swept_and_one_within_it_is_kept() {
        let tmp = repo();
        let (old, edge, writer) = (id("old"), id("edge"), id("writer"));
        let week = IDLE_EXPIRY.as_secs();
        set(tmp.path(), Some(&old), "055-a", t0());
        set(tmp.path(), Some(&edge), "055-a", at(1));
        let out = set(tmp.path(), Some(&writer), "056-b", at(week + 1));
        assert_eq!(out.expired, vec!["old".to_owned()], "idle 7d+1s is swept");
        assert!(!own_path(tmp.path(), &old).exists());
        assert!(
            own_path(tmp.path(), &edge).exists(),
            "idle exactly 7d is kept"
        );
    }

    #[test]
    fn resolving_counts_as_use_and_the_writer_is_never_swept() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        let week = IDLE_EXPIRY.as_secs();
        set(tmp.path(), Some(&x), "055-a", t0());
        resolve(tmp.path(), Some(&x), at(week)).unwrap();
        let out = set(tmp.path(), Some(&y), "056-b", at(week + 10));
        assert!(out.expired.is_empty(), "{:?}", out.expired);

        let late = set(tmp.path(), Some(&x), "055-a", at(10 * week));
        assert!(!late.expired.contains(&"x".to_owned()));
        assert!(own_path(tmp.path(), &x).exists());
    }

    #[test]
    fn an_unparseable_file_is_reported_by_the_sweep_and_left_in_place() {
        let tmp = repo();
        let x = id("x");
        set(tmp.path(), Some(&x), "055-a", t0());
        let bad = tmp.path().join(".ductus/sessions/broken.toml");
        std::fs::write(&bad, "feature = [").unwrap();
        let out = set(
            tmp.path(),
            Some(&x),
            "055-a",
            at(100 * IDLE_EXPIRY.as_secs()),
        );
        assert_eq!(out.unreadable, vec![bad.clone()]);
        assert!(bad.exists());
    }

    #[test]
    fn a_malformed_own_file_is_an_error_never_an_adoption() {
        let tmp = repo();
        let x = id("x");
        set(tmp.path(), None, "055-a", t0());
        let own = own_path(tmp.path(), &x);
        ensure_sessions_dir(tmp.path()).unwrap();
        std::fs::write(&own, "feature = [").unwrap();
        let err = resolve(tmp.path(), Some(&x), at(1)).unwrap_err();
        assert!(err.to_string().contains("x.toml"), "{err}");
        assert_eq!(std::fs::read_to_string(&own).unwrap(), "feature = [");
    }

    #[test]
    fn a_malformed_default_is_an_error_when_it_would_be_adopted() {
        let tmp = repo();
        std::fs::write(tmp.path().join(".ductus/session.toml"), "feature = [").unwrap();
        let err = resolve(tmp.path(), Some(&id("x")), t0()).unwrap_err();
        assert!(err.to_string().contains("session.toml"), "{err}");
        let err = resolve(tmp.path(), None, t0()).unwrap_err();
        assert!(err.to_string().contains("session.toml"), "{err}");
    }

    #[test]
    fn a_pending_removal_notice_is_delivered_once() {
        let tmp = repo();
        let x = id("x");
        set(tmp.path(), Some(&x), "055-a", t0());
        let own = own_path(tmp.path(), &x);
        let mut record = ProcessRecord::load(&own).unwrap();
        record.notice = Some(RemovalNotice {
            cause: "fold".into(),
            from: "1234.1-b".into(),
            to: Some("055-a".into()),
            by: "y".into(),
        });
        record.store(&own).unwrap();

        let first = resolve(tmp.path(), Some(&x), at(1)).unwrap();
        assert_eq!(first.notices.len(), 1);
        assert_eq!(first.notices[0].kind, SessionNoticeKind::Folded);
        assert!(first.notices[0].message.contains("1234.1-b"));
        assert!(
            resolve(tmp.path(), Some(&x), at(2))
                .unwrap()
                .notices
                .is_empty()
        );
    }

    #[test]
    fn a_legacy_layout_resolves_and_writes_as_unidentified() {
        let tmp = tempdir().unwrap();
        std::fs::write(
            tmp.path().join(".govern.session.toml"),
            "feature = \"055-a\"\npath = \"specs/055-a\"\n",
        )
        .unwrap();
        let x = id("x");
        let r = resolve(tmp.path(), Some(&x), t0()).unwrap();
        assert_eq!((r.identity, r.source), (None, SessionSource::Default));
        let out = set(tmp.path(), Some(&x), "056-b", at(1));
        assert_eq!(out.own_path, None);
        assert!(
            !tmp.path().join(".ductus").exists(),
            "no .ductus state created"
        );
    }

    #[test]
    fn a_peek_answers_like_resolve_and_writes_nothing() {
        let tmp = repo();
        let (x, y) = (id("x"), id("y"));
        set(tmp.path(), None, "055-a", t0());

        let peeked = peek(tmp.path(), Some(&x)).unwrap();
        assert_eq!(
            (peeked.source, feature_of(&peeked)),
            (SessionSource::Adopted, Some("055-a"))
        );
        assert!(peeked.notices.is_empty());
        assert!(
            !tmp.path().join(".ductus/sessions").exists(),
            "nothing created or pinned"
        );

        set(tmp.path(), Some(&y), "055-a", at(1));
        let before = std::fs::read_to_string(own_path(tmp.path(), &y)).unwrap();
        set(tmp.path(), Some(&x), "055-a", at(2));
        let own = peek(tmp.path(), Some(&y)).unwrap();
        assert_eq!(own.source, SessionSource::Own);
        assert!(own.notices.is_empty());
        assert_eq!(
            std::fs::read_to_string(own_path(tmp.path(), &y)).unwrap(),
            before,
            "no used-at refresh, no notice consumed"
        );
        let real = resolve(tmp.path(), Some(&y), at(3)).unwrap();
        assert_eq!(
            real.notices.len(),
            1,
            "the co-target notice survived the peek"
        );
    }

    // -- retarget (task 5) ----------------------------------------------------

    #[test]
    fn a_fold_retargets_every_session_naming_the_spec_and_notifies_the_others() {
        let tmp = repo();
        let (actor, other, bystander) = (id("actor"), id("other"), id("bystander"));
        set(tmp.path(), Some(&other), "1234.1-b", t0());
        set(tmp.path(), Some(&bystander), "057-c", at(1));
        set(tmp.path(), Some(&actor), "1234.1-b", at(2));
        let bystander_before = std::fs::read_to_string(own_path(tmp.path(), &bystander)).unwrap();

        let out = retarget(
            tmp.path(),
            Some(&actor),
            "1234.1-b",
            Some(&target("055-a")),
            RemovalCause::Fold,
            at(3),
        )
        .unwrap();
        // The default first, then per-process files in key order.
        assert_eq!(out.retargeted, ["default", "actor", "other"]);
        assert!(out.cleared.is_empty());
        assert!(default_text(tmp.path()).contains("feature = \"055-a\""));
        assert_eq!(
            std::fs::read_to_string(own_path(tmp.path(), &bystander)).unwrap(),
            bystander_before,
            "a session naming another feature is not written"
        );

        let acting = resolve(tmp.path(), Some(&actor), at(4)).unwrap();
        assert_eq!(feature_of(&acting), Some("055-a"));
        assert!(
            acting
                .notices
                .iter()
                .all(|n| n.kind != SessionNoticeKind::Folded),
            "the acting session is not told what it just did: {:?}",
            acting.notices
        );
        let told = resolve(tmp.path(), Some(&other), at(4)).unwrap();
        assert_eq!(feature_of(&told), Some("055-a"));
        let folded: Vec<_> = told
            .notices
            .iter()
            .filter(|n| n.kind == SessionNoticeKind::Folded)
            .collect();
        assert_eq!(folded.len(), 1, "{:?}", told.notices);
        assert!(folded[0].message.contains("1234.1-b"));
        assert!(folded[0].message.contains("session actor"));
    }

    #[test]
    fn a_consolidation_clears_every_session_naming_the_spec() {
        let tmp = repo();
        let (actor, other) = (id("actor"), id("other"));
        set(tmp.path(), Some(&other), "058-gone", t0());
        let out = retarget(
            tmp.path(),
            Some(&actor),
            "058-gone",
            None,
            RemovalCause::Consolidate,
            at(1),
        )
        .unwrap();
        assert_eq!(out.cleared, ["default", "other"]);
        assert!(!default_text(tmp.path()).contains("058-gone"));

        let r = resolve(tmp.path(), Some(&other), at(2)).unwrap();
        assert_eq!((r.source, r.target.clone()), (SessionSource::Cleared, None));
        assert_eq!(r.notices.len(), 1);
        assert_eq!(r.notices[0].kind, SessionNoticeKind::Consolidated);
        assert!(r.notices[0].message.contains("session actor"));

        set(tmp.path(), None, "059-new", at(3));
        let later = resolve(tmp.path(), Some(&other), at(4)).unwrap();
        assert_eq!(
            later.source,
            SessionSource::Cleared,
            "a cleared session never adopts"
        );
    }

    #[test]
    fn an_unidentified_actor_is_named_as_such() {
        let tmp = repo();
        let other = id("other");
        set(tmp.path(), Some(&other), "058-gone", t0());
        retarget(
            tmp.path(),
            None,
            "058-gone",
            None,
            RemovalCause::Consolidate,
            at(1),
        )
        .unwrap();
        let r = resolve(tmp.path(), Some(&other), at(2)).unwrap();
        assert!(
            r.notices[0].message.contains("an unidentified session"),
            "{:?}",
            r.notices
        );
    }

    #[test]
    fn a_removal_reports_files_it_cannot_read_and_leaves_them() {
        let tmp = repo();
        set(tmp.path(), Some(&id("x")), "057-c", t0());
        let bad = tmp.path().join(".ductus/sessions/broken.toml");
        std::fs::write(&bad, "feature = [").unwrap();
        let out = retarget(
            tmp.path(),
            None,
            "058-gone",
            None,
            RemovalCause::Consolidate,
            at(1),
        )
        .unwrap();
        assert_eq!(out.unreadable, vec![bad.clone()]);
        assert!(out.cleared.is_empty() && out.retargeted.is_empty());
        assert_eq!(std::fs::read_to_string(&bad).unwrap(), "feature = [");
    }

    #[test]
    fn timestamps_round_trip_and_foreign_shapes_are_refused() {
        for secs in [0, 1_700_000_000, 1_790_683_200, 4_102_444_799] {
            let t = UNIX_EPOCH + Duration::from_secs(secs);
            assert_eq!(parse_iso8601_utc(&iso8601_utc(t)), Some(t), "{secs}");
        }
        for bad in [
            "",
            "2026-09-29",
            "2026-09-29T12:00:00",
            "2026-13-01T00:00:00Z",
            "x",
        ] {
            assert_eq!(parse_iso8601_utc(bad), None, "{bad:?}");
        }
    }
}
