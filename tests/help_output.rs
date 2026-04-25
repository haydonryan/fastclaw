mod support;

use std::process::Command;

#[test]
fn root_help_lists_passthrough_and_gateway_status() {
    let bin = support::fastclaw_bin();
    let output = Command::new(bin)
        .arg("--help")
        .output()
        .expect("failed to run --help");

    assert!(
        output.status.success(),
        "help failed (code: {:?})\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--passthrough"),
        "missing --passthrough in help"
    );
    assert!(
        stdout.contains("gateway"),
        "missing gateway command in help"
    );
    assert!(stdout.contains("health"), "missing health command in help");
}

#[test]
fn gateway_help_lists_status_subcommand() {
    let bin = support::fastclaw_bin();
    let output = Command::new(bin)
        .args(["gateway", "--help"])
        .output()
        .expect("failed to run gateway --help");

    assert!(
        output.status.success(),
        "gateway help failed (code: {:?})\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("status"), "missing status in gateway help");
    assert!(stdout.contains("health"), "missing health in gateway help");
    assert!(
        stdout.contains("restart"),
        "missing restart in gateway help"
    );
}
