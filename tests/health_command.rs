use std::fs;
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn health_reports_ok_when_gateway_port_is_listening() {
    let bin = env!("CARGO_BIN_EXE_fastclaw");
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test listener");
    let port = listener.local_addr().expect("local addr").port();

    let sandbox = mktemp_dir("health-cmd-ok");
    let config_path = sandbox.join("openclaw.json");
    fs::write(
        &config_path,
        format!(r#"{{"gateway":{{"bind":"loopback","port":{port}}}}}"#),
    )
    .expect("write config");

    let output = Command::new(bin)
        .args(["health", "--json", "--timeout", "2000"])
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .output()
        .expect("run health");

    assert!(
        output.status.success(),
        "health should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"ok\": true"), "expected ok=true json");
}

#[test]
fn health_reports_failure_when_gateway_port_is_closed() {
    let bin = env!("CARGO_BIN_EXE_fastclaw");
    let sandbox = mktemp_dir("health-cmd-fail");
    let config_path = sandbox.join("openclaw.json");
    fs::write(
        &config_path,
        r#"{"gateway":{"bind":"loopback","port":6553}}"#,
    )
    .expect("write config");

    let output = Command::new(bin)
        .args(["health", "--json", "--timeout", "200"])
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .output()
        .expect("run health");

    assert!(
        !output.status.success(),
        "health should fail when port is closed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"ok\": false"), "expected ok=false json");
}

fn mktemp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("{prefix}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}
