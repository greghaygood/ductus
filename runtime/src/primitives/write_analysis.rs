//! `write-analysis` — record that `/ductus:analyze` ran, by writing
//! `specs/NNN/analysis.md`: the record in its frontmatter, the findings in a
//! fixed body skeleton. `spec.md` is not written (spec 057).
//!
//! The pipeline is `implement → review → analyze → done`, and until this
//! primitive existed only half of it left a trace. `check-review-gate` read
//! the `review:` block; Family 19 checked its freshness; Family 31 held it
//! against `review.md`. Analyze wrote nothing, so a spec that had passed both
//! gates and a spec that had passed only the first were **byte-identical on
//! disk**. Nothing could tell them apart, which meant nothing could enforce
//! the second gate, which meant the only thing holding it was whoever
//! remembered — the diligence dependency §design-principles rejects outright.
//!
//! That state was not hypothetical. On 2026-09-05 two specs were advanced to
//! `done` on the review gate alone and one of them was published to crates.io
//! before anyone noticed, because there was nothing to notice: every signal
//! the repository had said the spec was complete. The gap was found by being
//! asked, which is the definition of a diligence dependency.
//!
//! **This changes `/ductus:analyze`'s read-only contract, deliberately, and
//! the new line is between the subject and the observation.** Analyze still
//! never mutates an artifact it audits; `--fix` remains the only path that
//! does. Recording that the audit happened is not mutating the subject — it is
//! precisely what `write-review` does for the other gate, and precisely why
//! that gate was enforceable and this one was not.
//!
//! The block deliberately is **not** a copy of `review:`:
//!
//! - `advisory` is recorded and never gated on. An outstanding SHOULD blocks
//!   `done` at the review gate because §implement-phase says advisory is not
//!   ignorable there. Analyze's advisory tier is a different contract: its
//!   members are checks introduced advisory *with published promotion
//!   criteria* — grounding, Applicable-Rules citations, decision drift — and
//!   gating on them here would promote every one of them at once, past the
//!   criteria each declares.
//! - `unexamined` has no counterpart in `review:` at all, and is the field
//!   that makes this record honest. A clean analyze is two states, not one,
//!   and the command's own contract says so: "clean with nothing skipped is
//!   verified-clean, clean with something skipped is partially examined." A
//!   record carrying only finding counts would collapse that into the
//!   reassuring reading — inside the artifact a later gate trusts, which is
//!   the worst possible place for `QUAL-CLAIM-001`.
//!
//! Defined by `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, read_text, rel_path, split_frontmatter, validate_no_traversal,
    write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{WriteAnalysisArgs, WriteAnalysisResult};

/// Execute the `write-analysis` primitive against the given repo root.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
///   carries a parent-directory component.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory does not
///   exist.
/// - [`PrimitiveError::Io`] when `spec.md` cannot be read or written.
/// - [`PrimitiveError::Yaml`] when the frontmatter block is malformed —
///   never repaired here. A spec whose frontmatter does not parse is one the
///   analysis itself would have hard-failed on, and writing a record of a
///   clean run into it would be the exact inversion this primitive exists to
///   prevent.
pub fn run(args: &WriteAnalysisArgs, repo: &Path) -> Result<WriteAnalysisResult> {
    validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }

    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;

    // Parse before writing. The value is not used, but a frontmatter block
    // that does not deserialize must not receive a record asserting a clean
    // analysis — see the `Yaml` note above.
    let _: crate::schema::primitives::Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    let replaced = fm_text
        .lines()
        .any(|line| !line.starts_with([' ', '\t']) && line.starts_with("analyze:"));

    let blocking = args.hard_fail > 0 || args.blocking_findings > 0;
    // The breakdown is the authority when supplied: a total a caller can
    // contradict is a total that will eventually be contradicted, which is
    // the same reason `blocking` is derived rather than accepted.
    let by_reason: BTreeMap<String, u32> =
        args.unexamined_by_reason
            .iter()
            .fold(BTreeMap::new(), |mut acc, (reason, count)| {
                *acc.entry(reason.clone()).or_default() += *count;
                acc
            });
    // A registered shared constitution this run could not read is an unexamined
    // *input* to the analysis, not an unexamined target, but it lands in the same
    // breakdown because it makes the same claim false: that everything bearing on
    // the verdict was looked at. Derived here rather than accepted as an argument,
    // for the reason the comment above gives — a caller that had to supply it
    // could omit it, and `QUAL-CLAIM-001` exists precisely to stop a clean result
    // standing in for an unexamined one (spec 055, AC8).
    let mut by_reason = by_reason;
    // Deliberately NOT `?`. This primitive's contract is that it writes a
    // record on every run, because the record's *absence* is what a later
    // gate reads as "never analyzed" — so a config that will not parse must
    // not be able to suppress it. An unreadable registry is recorded as its
    // own reason instead, which is the honest answer: nothing registered was
    // loaded, and the run cannot say how many sources that was.
    match crate::primitives::resolve_constitutions::run(
        &crate::schema::primitives::ResolveConstitutionsArgs {},
        repo,
    ) {
        Ok(governance) if !governance.skipped.is_empty() => {
            let count = u32::try_from(governance.skipped.len()).unwrap_or(u32::MAX);
            *by_reason
                .entry("constitution-unresolved".to_string())
                .or_default() += count;
        }
        Ok(_) => {}
        Err(_) => {
            *by_reason
                .entry("constitution-registry-unreadable".to_string())
                .or_default() += 1;
        }
    }
    let by_reason = by_reason;

    let unexamined = if by_reason.is_empty() {
        args.unexamined
    } else {
        by_reason.values().copied().sum()
    };
    // The digest of what this run examined, taken here rather than accepted
    // as an argument. `/{project}:analyze` is read-only, so the subjects are
    // byte-identical to the ones its passes read moments ago — and deriving it
    // means no caller can record a digest it did not take, the same discipline
    // that derives `blocking` and sums `unexamined` from its breakdown. It
    // also guarantees this digest and the one the gate recomputes come from
    // one function, which is what makes the two surfaces agree.
    let subjects = crate::primitives::analyze_subjects::subject_digest(
        &spec_path
            .parent()
            .map_or_else(|| repo.to_path_buf(), std::path::Path::to_path_buf),
        crate::primitives::analyze_subjects::is_analyze_subject,
    );
    // The artifact is written on EVERY run — clean, empty-scope, blocking
    // alike. A later gate reads its absence as "never analyzed" (spec 057 AC5),
    // so a run that declined to write because it had nothing to report would
    // make "no analysis" and "a clean analysis" the same state on disk. That is
    // the byte-identical failure `write-analysis` was built to end, one level
    // out.
    let analysis_path = feature_dir.join(crate::primitives::ANALYSIS_RECORD_FILE);
    let report = render_analysis(args, blocking, unexamined, &by_reason, &subjects);
    write_atomic(&analysis_path, &report)?;

    Ok(WriteAnalysisResult {
        spec_path: rel_path(&spec_path, repo),
        blocking,
        unexamined,
        replaced,
        captured_issues: u32::try_from(args.captured_issues.len()).unwrap_or(u32::MAX),
    })
}

/// Render `analysis.md` — the record's frontmatter plus the fixed skeleton.
///
/// The counterpart to `write-review`'s `render_report`, and deliberately the
/// same shape: one artifact per command, carrying its own record and a report
/// a person can read. Overwritten whole on every run, so the file always
/// describes the current analysis and git carries the history.
fn render_analysis(
    args: &WriteAnalysisArgs,
    blocking: bool,
    unexamined: u32,
    by_reason: &BTreeMap<String, u32>,
    subjects: &crate::primitives::analyze_subjects::SubjectDigest,
) -> String {
    let feature = &args.feature;
    let mut out = String::from("---\n");
    let _ = writeln!(out, "spec: {}", single_line(feature));
    let _ = writeln!(out, "last-run: {}", single_line(&args.analyzed_at));
    let _ = writeln!(
        out,
        "analyzed-against: {}",
        single_line(&args.analyzed_against)
    );
    let _ = writeln!(out, "hard-fail: {}", args.hard_fail);
    let _ = writeln!(out, "blocking-findings: {}", args.blocking_findings);
    let _ = writeln!(out, "advisory: {}", args.advisory);
    let _ = writeln!(out, "unexamined: {unexamined}");
    let _ = writeln!(out, "captured-issues: {}", args.captured_issues.len());
    if !subjects.digests.is_empty() {
        let _ = writeln!(out, "analyzed-digest:");
        for (path, digest) in &subjects.digests {
            let _ = writeln!(out, "  {path}: {digest}");
        }
    }
    if !subjects.unreadable.is_empty() {
        let _ = writeln!(out, "analyzed-unreadable:");
        for path in &subjects.unreadable {
            let _ = writeln!(out, "  - {path}");
        }
    }
    if !by_reason.is_empty() {
        let _ = writeln!(out, "unexamined-by-reason:");
        for (reason, count) in by_reason {
            let _ = writeln!(out, "  {reason}: {count}");
        }
    }
    let _ = writeln!(out, "blocking: {blocking}");
    out.push_str("---\n\n");

    let _ = writeln!(out, "# Analysis — {feature}\n");
    let verdict = if blocking { "blocking" } else { "not blocking" };
    let _ = writeln!(out, "## Summary\n");
    let _ = writeln!(
        out,
        "{} hard-fail, {} blocking, {} advisory; {verdict}. {} unexamined target(s). \
         Findings route to the inbox — this report records them, `/{{project}}:groom` routes them.\n",
        args.hard_fail, args.blocking_findings, args.advisory, unexamined,
    );
    let _ = writeln!(out, "## Hard failures\n\n{}\n", tier_line(args.hard_fail));
    let _ = writeln!(
        out,
        "## Blocking findings\n\n{}\n",
        tier_line(args.blocking_findings)
    );
    let _ = writeln!(
        out,
        "## Advisory findings\n\n{}\n",
        tier_line(args.advisory)
    );
    let _ = writeln!(
        out,
        "## Unexamined targets\n\n{}\n",
        render_unexamined(unexamined, by_reason)
    );
    let _ = write!(
        out,
        "## Captured issues\n\n{}\n",
        render_captured_plain(&args.captured_issues)
    );
    out
}

/// A tier's one-line count, or `*None.*`.
fn tier_line(count: u32) -> String {
    if count == 0 {
        "*None.*".to_string()
    } else {
        format!("{count} finding(s) — see **Captured issues** below and the inbox.")
    }
}

/// The unexamined breakdown, or `*None — every target was examined.*`
///
/// Zero with no breakdown is stated explicitly rather than left blank: a clean
/// run with nothing skipped and a clean run with something skipped are two
/// results, and an empty section would read as the first while meaning either.
fn render_unexamined(unexamined: u32, by_reason: &BTreeMap<String, u32>) -> String {
    if unexamined == 0 && by_reason.is_empty() {
        return "*None — every target was examined.*".to_string();
    }
    if by_reason.is_empty() {
        return format!("{unexamined} target(s) unexamined; no breakdown recorded.");
    }
    by_reason
        .iter()
        .map(|(reason, count)| format!("- {reason}: {count}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The captured findings as plain bullets, with any checkbox marker stripped.
///
/// The strip is the mechanical half of spec 057 AC13: these strings are inbox
/// bullets, and the inbox's form *is* `- [ ] …`. Rendering them verbatim would
/// make this report a second triage queue — the parallel surface 047 rejected —
/// so the guarantee is enforced here rather than asked of every caller.
fn render_captured_plain(issues: &[String]) -> String {
    if issues.is_empty() {
        return "*None.*".to_string();
    }
    issues
        .iter()
        .map(|line| {
            let text = line.trim();
            let text = text.strip_prefix("- ").unwrap_or(text);
            let text = text
                .strip_prefix("[ ] ")
                .or_else(|| text.strip_prefix("[x] "))
                .or_else(|| text.strip_prefix("[X] "))
                .unwrap_or(text);
            format!("- {}", text.trim())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Collapse any line break in a host-supplied scalar to a space.
///
/// `write-review` rejects such a value outright; this one flattens instead,
/// because both of these fields are machine-generated (a timestamp and a sha)
/// and a newline in either is a caller defect with no legitimate reading —
/// there is no user intent to preserve, only an injection to defuse.
fn single_line(value: &str) -> String {
    value.replace(['\n', '\r'], " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn spec_repo(frontmatter: &str) -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/042-demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}\n---\n\n# 042 — Demo\n\n## Behavior\n\nText.\n"),
        )
        .unwrap();
        tmp
    }

    /// `analysis.md`'s text, after a run.
    fn analysis_md(tmp: &TempDir) -> String {
        fs::read_to_string(tmp.path().join("specs/042-demo/analysis.md")).unwrap()
    }

    fn analysis_exists(tmp: &TempDir) -> bool {
        tmp.path().join("specs/042-demo/analysis.md").exists()
    }

    /// The artifact is written on a run with nothing whatsoever to report.
    ///
    /// This is the load-bearing case, not the trivial one: a later gate reads
    /// the file's *absence* as never-analyzed, so a clean run that skipped the
    /// write would make "no analysis" and "a clean analysis" identical on disk
    /// (spec 057 AC19).
    #[test]
    fn a_clean_run_still_writes_the_artifact() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(&args(), tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        let (fm, body) =
            crate::primitives::split_frontmatter(&report, Path::new("analysis.md")).unwrap();
        let record: crate::schema::primitives::AnalyzeBlock =
            serde_norway::from_str(fm).expect("frontmatter deserializes into the record");

        assert_eq!(record.last_run.as_deref(), Some("2026-09-05T18:00:00Z"));
        assert!(!record.blocking);
        assert!(
            body.contains("*None — every target was examined.*"),
            "a clean run says so rather than leaving the section blank: {body}"
        );
    }

    /// Every section of the skeleton is present on every run, in order.
    #[test]
    fn the_skeleton_is_fixed_and_complete() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(&args(), tmp.path()).unwrap();
        let report = analysis_md(&tmp);

        let expected = [
            "# Analysis — 042-demo",
            "## Summary",
            "## Hard failures",
            "## Blocking findings",
            "## Advisory findings",
            "## Unexamined targets",
            "## Captured issues",
        ];
        let mut cursor = 0;
        for heading in expected {
            let found = report[cursor..]
                .find(heading)
                .unwrap_or_else(|| panic!("{heading} missing or out of order in:\n{report}"));
            cursor += found + heading.len();
        }
    }

    /// No section carries a checkbox — the mechanical half of AC13.
    ///
    /// The captured bullets arrive in the inbox's own `- [ ] …` form, so the
    /// guarantee has to be enforced by the renderer rather than asked of the
    /// caller. Feeding it exactly that form is the point of the fixture.
    #[test]
    fn the_report_carries_no_checkbox_even_when_captures_arrive_with_one() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 2;
        a.captured_issues = vec![
            "- [ ] convention: criterion-path-existence — specs/system.md missing — spec.md".into(),
            "bug: task-consistency — task 4 has no Done when — tasks.md".into(),
        ];
        run(&a, tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        assert!(
            !report.contains("- [ ]") && !report.contains("- [x]"),
            "analysis.md must never become a second triage queue:\n{report}"
        );
        assert!(
            report.contains("- convention: criterion-path-existence"),
            "the finding's text still lands, just without the marker:\n{report}"
        );
        assert!(report.contains("- bug: task-consistency"), "{report}");
    }

    /// An unexamined breakdown reaches the report, not just the record.
    #[test]
    fn unexamined_reasons_are_rendered() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.unexamined_by_reason = vec![("no-readable-state".into(), 3)];
        run(&a, tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        assert!(report.contains("- no-readable-state: 3"), "{report}");
    }

    fn args() -> WriteAnalysisArgs {
        WriteAnalysisArgs {
            feature: "042-demo".into(),
            analyzed_at: "2026-09-05T18:00:00Z".into(),
            analyzed_against: "abc123".into(),
            hard_fail: 0,
            blocking_findings: 0,
            advisory: 0,
            unexamined: 0,
            unexamined_by_reason: vec![],
            captured_issues: vec![],
        }
    }

    #[test]
    fn records_captured_issues_beside_advisory() {
        // The pair is the point: `advisory` says how many findings the run
        // produced, `captured-issues` how many it landed in the inbox. Before
        // this field a run that recorded `advisory: 3` and captured nothing
        // was byte-identical to one that captured all three, and nothing
        // could tell them apart. The cross-artifact comparison that once
        // covered the review side is retired (spec 057), so this field is the
        // only thing that separates them.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 3;
        a.captured_issues = vec![
            "convention: a.rs names a retired path".into(),
            "bug: b.rs drops an error".into(),
            "perf: c.rs re-reads the config per call".into(),
        ];
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(result.captured_issues, 3);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("advisory: 3"));
        assert!(spec.contains("captured-issues: 3"));
    }

    #[test]
    fn a_divergence_between_advisory_and_captured_is_recorded_not_smoothed() {
        // The two numbers are NOT required to agree, and the field would be
        // worse than useless if they were forced to: `append-inbox`'s
        // dedup-prefix guard legitimately suppresses a re-append, so a correct
        // re-run captures fewer than it found. What must not happen is the
        // divergence being invisible.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 4;
        a.captured_issues = vec![];
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(result.captured_issues, 0);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("advisory: 4"));
        assert!(spec.contains("captured-issues: 0"));
    }

    /// A first run creates the artifact, and leaves `spec.md` alone.
    ///
    /// Was `inserts_the_block_when_absent`: the record used to be spliced into
    /// spec frontmatter, so the contract was "insert without disturbing the
    /// neighbours". With one home per record there are no neighbours, and the
    /// contract worth pinning is that the spec is not touched at all.
    #[test]
    fn a_first_run_writes_the_artifact_and_leaves_the_spec_untouched() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let before = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.replaced);
        assert!(!result.blocking);

        let report = analysis_md(&tmp);
        assert!(
            report.contains("last-run: 2026-09-05T18:00:00Z"),
            "{report}"
        );
        assert!(report.contains("blocking: false"), "{report}");

        let after = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert_eq!(before, after, "the spec is not a write target any more");
    }

    /// A re-run overwrites its own artifact whole, and the review record beside
    /// it is untouched.
    ///
    /// The sibling-preservation contract survives the relocation; what changed
    /// is that the sibling is a *file* rather than an adjacent frontmatter
    /// block, which makes it harder to damage rather than easier.
    #[test]
    fn a_rerun_overwrites_its_own_record_and_leaves_the_review_record_alone() {
        let tmp = spec_repo("status: done\ndependencies: []\nnext-criterion: 7");
        let dir = tmp.path().join("specs/042-demo");
        fs::write(
            dir.join("analysis.md"),
            "---\nspec: 042-demo\nlast-run: 2019-01-01T00:00:00Z\nblocking: true\n---\n\n# Analysis\n",
        )
        .unwrap();
        let review = "---\nspec: 042-demo\nlast-run: 2020-01-01T00:00:00Z\nblocking: false\n---\n\n# Review\n";
        fs::write(dir.join("review.md"), review).unwrap();

        run(&args(), tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        assert!(!report.contains("2019-01-01T00:00:00Z"), "{report}");
        assert!(
            report.contains("last-run: 2026-09-05T18:00:00Z"),
            "{report}"
        );
        assert_eq!(
            fs::read_to_string(dir.join("review.md")).unwrap(),
            review,
            "the review record is not this command's to write"
        );
        assert!(
            fs::read_to_string(dir.join("spec.md"))
                .unwrap()
                .contains("next-criterion: 7")
        );
    }

    #[test]
    fn blocking_is_set_by_either_gating_tier() {
        for (hard, blocking_findings) in [(1, 0), (0, 1), (2, 3)] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let result = run(
                &WriteAnalysisArgs {
                    hard_fail: hard,
                    blocking_findings,
                    ..args()
                },
                tmp.path(),
            )
            .unwrap();
            assert!(result.blocking, "hard={hard} blocking={blocking_findings}");
        }
    }

    /// Advisory findings are recorded and never gate — the asymmetry with the
    /// review block is the design, not an omission.
    #[test]
    fn advisory_findings_are_recorded_but_do_not_block() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                advisory: 9,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.blocking);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("advisory: 9"));
        assert!(spec.contains("blocking: false"));
    }

    /// A bare total answers *that* something was unexamined and nothing
    /// about what — and the reasons are not equivalent. The breakdown is what
    /// makes the number actionable.
    #[test]
    fn the_breakdown_is_written_and_sorted() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined_by_reason: vec![
                    ("ships-to-adopter".into(), 10),
                    ("not-a-live-claim".into(), 81),
                    ("artifact-unreadable".into(), 1),
                ],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 92);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 92"));
        // BTreeMap order, so the rendering is byte-stable across runs.
        let block = spec.split("unexamined-by-reason:").nth(1).unwrap();
        let order: Vec<&str> = block
            .lines()
            .skip(1) // the remainder of the `unexamined-by-reason:` line itself
            .take_while(|l| l.starts_with("  "))
            .map(|l| l.trim().split(':').next().unwrap())
            .collect();
        assert_eq!(
            order,
            vec![
                "artifact-unreadable",
                "not-a-live-claim",
                "ships-to-adopter"
            ]
        );
    }

    /// The breakdown is the authority: a total a caller can contradict is a
    /// total that will eventually be contradicted, which is why `blocking` is
    /// derived too.
    #[test]
    fn a_supplied_total_cannot_contradict_its_breakdown() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 999,
                unexamined_by_reason: vec![("root-absent".into(), 4)],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 4);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 4"));
        // Anchored to the field, not to the bare digits: the block now also
        // carries hex digests, and a bare `999` match can land inside one.
        assert!(!spec.contains("unexamined: 999"));
    }

    /// A fully-examined run carries no map rather than a map of zeroes.
    #[test]
    fn an_empty_breakdown_omits_the_map_and_keeps_the_supplied_total() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 3);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 3"));
        assert!(!spec.contains("unexamined-by-reason"));
    }

    /// The `QUAL-CLAIM-001` field: a clean run that could not examine
    /// everything must not record the same thing as one that could.
    #[test]
    fn unexamined_count_survives_a_clean_run() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 3"));
        assert!(spec.contains("blocking: false"));
    }

    #[test]
    fn newline_in_a_scalar_cannot_inject_frontmatter_keys() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                analyzed_against: "abc\nstatus: done".into(),
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let report = analysis_md(&tmp);
        assert!(
            report.contains("analyzed-against: abc status: done"),
            "{report}"
        );
        assert!(
            !report.contains("\nstatus: done"),
            "an embedded newline must not become a frontmatter key: {report}"
        );
        // And the spec it was aimed at is untouched.
        assert!(
            fs::read_to_string(tmp.path().join("specs/042-demo/spec.md"))
                .unwrap()
                .contains("status: in-progress")
        );
    }

    #[test]
    fn malformed_frontmatter_is_never_given_a_clean_record() {
        let tmp = spec_repo("status: in-progress\ndependencies: [oops");
        assert!(matches!(
            run(&args(), tmp.path()).unwrap_err(),
            PrimitiveError::Yaml { .. }
        ));
        assert!(
            !analysis_exists(&tmp),
            "a spec that will not parse receives no record at all — the artifact's \
             absence is what a later gate reads as never-analyzed"
        );
    }

    #[test]
    fn missing_feature_is_an_error() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        assert!(matches!(
            run(
                &WriteAnalysisArgs {
                    feature: "999-absent".into(),
                    ..args()
                },
                tmp.path()
            )
            .unwrap_err(),
            PrimitiveError::FeatureNotFound { .. }
        ));
    }

    /// Register one `[constitutions.*]` entry pointing at `path`.
    fn with_constitution(tmp: &TempDir, alias: &str, path: &str) {
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("config.toml"),
            format!(
                "[constitutions.{alias}]\nrepo = \"https://example.test/g\"\npath = \"{path}\"\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn an_unresolved_constitution_is_recorded_as_unexamined() {
        // AC8: an analysis that ran without a registered source must not record a
        // clean, fully-examined run. The reason lands in the breakdown the record
        // already carries, so the pre-done gate and any reader see it.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "nowhere");

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 1);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 1"), "{spec}");
        assert!(spec.contains("constitution-unresolved: 1"), "{spec}");
    }

    #[test]
    fn a_resolved_constitution_records_nothing_unexamined() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "gov");
        let gov = tmp.path().join("gov");
        fs::create_dir_all(&gov).unwrap();
        fs::write(gov.join("constitution.md"), "# House rules\n").unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            result.unexamined, 0,
            "a source that was read is not unexamined"
        );
    }

    #[test]
    fn no_registered_constitution_leaves_the_count_untouched() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 0);
        let spec = analysis_md(&tmp);
        assert!(
            !spec.contains("constitution-unresolved"),
            "a project with none registered reads exactly as before: {spec}"
        );
    }

    #[test]
    fn a_malformed_config_still_records_the_run() {
        // Regression: the registry read was `?`-propagated, so an unrelated
        // config typo suppressed the analyze record entirely -- and an absent
        // record is exactly what the pre-done gate reads as "never analyzed".
        // This primitive writes a record on every run, by contract.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), "[constitutions.acme]\nrepo = \n").unwrap();

        let result = run(&args(), tmp.path());
        assert!(result.is_ok(), "a config typo must not suppress the record");

        let spec = analysis_md(&tmp);
        assert!(
            spec.contains("constitution-registry-unreadable: 1"),
            "{spec}"
        );
        assert!(spec.contains("unexamined: 1"), "{spec}");
    }
}
