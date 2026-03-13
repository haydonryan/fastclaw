use std::path::Path;
use std::process::Command;

#[test]
fn health_matches_openclaw_exactly() {
    if !cfg!(unix) {
        return;
    }
    if !Path::new("/usr/bin/openclaw").exists() {
        return;
    }

    let bin = env!("CARGO_BIN_EXE_fastclaw");
    let fast = Command::new(bin)
        .args(["health", "--timeout", "10000"])
        .output()
        .expect("run fastclaw health");
    let upstream = Command::new("/usr/bin/openclaw")
        .args(["health", "--timeout", "10000"])
        .output()
        .expect("run openclaw health");

    assert_eq!(
        fast.status.code(),
        upstream.status.code(),
        "exit code mismatch"
    );
    assert_eq!(
        normalize(&String::from_utf8_lossy(&fast.stdout)),
        normalize(&String::from_utf8_lossy(&upstream.stdout)),
        "stdout mismatch"
    );
    assert_eq!(
        normalize(&String::from_utf8_lossy(&fast.stderr)),
        normalize(&String::from_utf8_lossy(&upstream.stderr)),
        "stderr mismatch"
    );
}

fn normalize(input: &str) -> String {
    input
        .chars()
        .map(|ch| if ch.is_ascii_digit() { '#' } else { ch })
        .collect()
}
