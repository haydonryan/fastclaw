use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let mut out = String::with_capacity(input.len());
    let mut in_digits = false;
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            if !in_digits {
                out.push('#');
                in_digits = true;
            }
        } else {
            in_digits = false;
            out.push(ch);
        }
    }
    out
}

#[test]
fn native_health_delegates_exact_output_and_exit_code() {
    if !cfg!(unix) {
        return;
    }
    let bin = env!("CARGO_BIN_EXE_fastclaw");
    let sandbox = mktemp_dir("health-delegate");
    let fake = sandbox.join("openclaw");

    fs::write(
        &fake,
        "#!/usr/bin/env bash\n\
printf 'STDOUT:exact\\n'\n\
printf 'STDERR:exact\\n' >&2\n\
printf '%s\\n' \"$*\" >> \"$FASTCLAW_HEALTH_ARGS_LOG\"\n\
exit 7\n",
    )
    .expect("write fake openclaw");
    let mut perms = fs::metadata(&fake).expect("stat fake").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake, perms).expect("chmod fake");

    let args_log = sandbox.join("args.log");
    let out = Command::new(bin)
        .args([
            "health",
            "--json",
            "--timeout",
            "1234",
            "--verbose",
            "--debug",
        ])
        .env("FASTCLAW_OPENCLAW_BIN", &fake)
        .env("FASTCLAW_HEALTH_ARGS_LOG", &args_log)
        .output()
        .expect("run fastclaw health");

    assert_eq!(out.status.code(), Some(7), "expected delegated exit code");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "STDOUT:exact\n");
    assert_eq!(String::from_utf8_lossy(&out.stderr), "STDERR:exact\n");

    let logged_args = fs::read_to_string(args_log).expect("read args log");
    assert_eq!(
        logged_args.trim_end(),
        "health --json --timeout 1234 --verbose --debug"
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
