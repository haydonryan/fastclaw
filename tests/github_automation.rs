use std::fs;

#[test]
fn github_ci_workflow_has_fmt_and_tests() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/.github/workflows/ci.yml");
    let content = fs::read_to_string(path).expect("failed to read .github/workflows/ci.yml");

    assert!(content.contains("cargo fmt --all -- --check"));
    assert!(content.contains("cargo test --all-targets"));
}

#[test]
fn dependabot_automerge_workflow_targets_dependabot() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.github/workflows/dependabot-automerge.yml"
    );
    let content = fs::read_to_string(path)
        .expect("failed to read .github/workflows/dependabot-automerge.yml");

    assert!(content.contains("dependabot[bot]"));
    assert!(content.contains("enable-pull-request-automerge"));
}

#[test]
fn dependabot_config_includes_cargo_and_actions() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/.github/dependabot.yml");
    let content = fs::read_to_string(path).expect("failed to read .github/dependabot.yml");

    assert!(content.contains("package-ecosystem: \"cargo\""));
    assert!(content.contains("package-ecosystem: \"github-actions\""));
}
