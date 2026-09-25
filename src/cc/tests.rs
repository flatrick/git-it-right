use super::*;

fn run(msg: &str) -> Outcome {
    check(msg, &Config::default())
}

fn rules(o: &Outcome) -> (Vec<&str>, Vec<&str>) {
    (o.fixes.iter().map(|f| f.rule).collect(), o.violations.iter().map(|v| v.rule).collect())
}

#[test]
fn spec_examples_pass_untouched() {
    for msg in [
        "feat: allow provided config object to extend other configs\n\nBREAKING CHANGE: `extends` key in config file is now used for extending other config files",
        "feat!: send an email to the customer when a product is shipped",
        "feat(api)!: send an email to the customer when a product is shipped",
        "chore!: drop support for Node 6\n\nBREAKING CHANGE: use JavaScript features not available in Node 6.",
        "docs: correct spelling of CHANGELOG",
        "feat(lang): add Polish language",
        "fix: prevent racing of requests\n\nIntroduce a request id and a reference to latest request. Dismiss\nincoming responses other than from latest request.\n\nRemove timeouts which were used to mitigate the racing issue but are\nobsolete now.\n\nReviewed-by: Z\nRefs: #123",
        "revert: let us never again speak of the noodle incident\n\nRefs: 676104e, a215868",
    ] {
        let o = run(msg);
        assert_eq!(rules(&o), (vec![], vec![]), "{msg}");
        assert_eq!(o.text, msg);
    }
}

#[test]
fn safe_fixes_are_applied_and_reported() {
    let cases = [
        ("Feat: add x", "feat: add x", vec!["type-case"]),
        ("feat :add x", "feat: add x", vec!["header-spacing"]),
        ("feature(api): add x.", "feat(api): add x", vec!["type-alias", "desc-period"]),
        ("Feature: add x", "feat: add x", vec!["type-case", "type-alias"]),
        ("feat(): add x", "feat: add x", vec!["scope-empty"]),
        ("feat: add x\nbody line", "feat: add x\n\nbody line", vec!["body-separator"]),
        ("feat: x\n\nbreaking change: y", "feat: x\n\nBREAKING CHANGE: y", vec!["breaking-footer"]),
        ("feat: x\n\nBreaking-Change:y", "feat: x\n\nBREAKING CHANGE: y", vec!["breaking-footer"]),
        ("  feat: x  \n\n\n\nbody  \n\n", "feat: x\n\nbody", vec!["header-spacing"]),
    ];
    for (input, want, fixes) in cases {
        let o = run(input);
        assert_eq!(o.text, want, "{input:?}");
        assert_eq!(rules(&o), (fixes, vec![]), "{input:?}");
    }
}

#[test]
fn fixes_are_idempotent() {
    for input in ["Feature(api) : Add x.\nbody\n\nbreaking change: y", "feat :x"] {
        let once = run(input);
        let twice = run(&once.text);
        assert_eq!(twice.text, once.text);
        assert!(twice.fixes.is_empty(), "{:?}", twice.fixes);
    }
}

#[test]
fn breaking_change_hyphen_form_is_left_alone() {
    let o = run("feat: x\n\nBREAKING-CHANGE: y");
    assert_eq!(rules(&o), (vec![], vec![]));
}

#[test]
fn unfixable_messages_are_rejected_with_a_hint() {
    let o = run("Update README.md");
    assert_eq!(rules(&o).1, vec!["type-missing"]);
    assert!(o.violations[0].hint.as_deref().unwrap().starts_with("<type>: Update README.md"));

    let o = run("feet: add x");
    assert_eq!(rules(&o).1, vec!["type-unknown"]);
    assert!(o.violations[0].message.contains("did you mean `feat`"), "{}", o.violations[0].message);
    assert!(o.violations[0].hint.as_deref().unwrap().starts_with("feat: add x"));

    assert_eq!(rules(&run("feat: ")).1, vec!["desc-empty"]);
    assert_eq!(rules(&run(&format!("feat: {}", "x".repeat(80)))).1, vec!["header-length"]);
}

#[test]
fn scope_rules_follow_config() {
    let cfg = Config { scopes: vec!["api".into(), "cli".into()], scope_required: true, ..Config::default() };
    assert_eq!(rules(&check("feat: x", &cfg)).1, vec!["scope-required"]);
    assert_eq!(rules(&check("feat(db): x", &cfg)).1, vec!["scope-unknown"]);
    assert_eq!(rules(&check("feat(api): x", &cfg)), (vec![], vec![]));
}

#[test]
fn desc_case_lower_is_opt_in() {
    assert!(run("feat: Add x").fixes.is_empty());
    let cfg = Config { desc_case: DescCase::Lower, ..Config::default() };
    assert_eq!(check("feat: Add x", &cfg).text, "feat: add x");
    assert_eq!(check("feat: API change", &cfg).text, "feat: API change");
}

#[test]
fn special_subjects() {
    assert_eq!(run("fixup! feat: add x").kind, Kind::Autosquash("fixup!".into()));
    assert_eq!(run("amend! feat: add x\n\nfeat: better x").kind, Kind::Autosquash("amend!".into()));
    assert_eq!(run("Merge branch 'main' into topic").kind, Kind::Merge);
    assert_eq!(run("Revert \"feat: add x\"\n\nThis reverts commit abc.").kind, Kind::Revert);
    let strict = Config { allow_merge: false, allow_revert: false, ..Config::default() };
    assert_eq!(rules(&check("Merge branch 'x'", &strict)).1, vec!["merge-commit"]);
    assert_eq!(rules(&check("Revert \"feat: x\"", &strict)).1, vec!["revert-commit"]);
}

#[test]
fn every_emitted_rule_is_listed() {
    let inputs = ["", "x", "feet: x", "feat: ", "Feature(): x.\nb\n\nbreaking change: y"];
    for input in inputs {
        let o = run(input);
        for r in o.fixes.iter().map(|f| f.rule).chain(o.violations.iter().map(|v| v.rule)) {
            assert!(RULES.contains(&r), "{r}");
        }
    }
}

#[test]
fn empty_message_is_rejected_after_normalization() {
    let outcome = run(" \n\t\n");
    assert_eq!(rules(&outcome).1, vec!["empty"]);
    assert!(!outcome.ok());
}

#[test]
fn invalid_conventional_scope_is_rejected_as_spec() {
    let outcome = run("feat(a(b): add x");
    assert_eq!(rules(&outcome), (vec![], vec!["spec"]));
    assert!(!outcome.ok());
}

#[test]
fn description_left_empty_by_period_fix_is_desc_empty() {
    let outcome = run("feat: .");
    assert_eq!(rules(&outcome), (vec!["desc-period"], vec!["desc-empty"]));
}
