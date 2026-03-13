use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn gateway_restart_calls_systemctl_user_restart() {
    if !cfg!(unix) {
        return;
    }

    let bin = env!("CARGO_BIN_EXE_fastclaw");
    let sandbox = mktemp_dir("gateway-restart-test");
    let fake_bin_dir = sandbox.join("bin");
    let fake_home = sandbox.join("home");
    let log_file = sandbox.join("systemctl.log");
    let unit_dir = fake_home.join(".config/systemd/user");
    fs::create_dir_all(&fake_bin_dir).expect("create fake bin dir");
    fs::create_dir_all(&unit_dir).expect("create unit dir");
    fs::write(
        unit_dir.join("openclaw-gateway.service"),
        "[Service]\nExecStart=/bin/true\n",
    )
    .expect("write fake unit");

    let fake_systemctl = fake_bin_dir.join("systemctl");
    fs::write(
        &fake_systemctl,
        format!(
            "#!/usr/bin/env bash\nprintf '%s\\n' \"$*\" >> \"{}\"\nexit 0\n",
            log_file.display()
        ),
    )
    .expect("write fake systemctl");
    let mut perms = fs::metadata(&fake_systemctl)
        .expect("stat fake systemctl")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake_systemctl, perms).expect("chmod fake systemctl");

    let path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new(bin)
        .args(["gateway", "restart"])
        .env("HOME", &fake_home)
        .env("PATH", format!("{}:{}", fake_bin_dir.display(), path))
        .output()
        .expect("run gateway restart");

    assert!(
        output.status.success(),
        "restart failed (code: {:?})\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let logged = fs::read_to_string(&log_file).expect("read systemctl log");
    assert!(
        logged
            .lines()
            .any(|line| line == "--user restart openclaw-gateway.service"),
        "expected systemctl restart call, got:\n{}",
        logged
    );
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
