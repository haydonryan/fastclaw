use std::path::Path;
use std::process::Command;

#[test]
fn gateway_status_matches_openclaw_output_except_header() {
    if !cfg!(unix) {
        return;
    }
    if !Path::new("/usr/bin/openclaw").exists() {
        return;
    }

    let rust_bin = env!("CARGO_BIN_EXE_fastclaw");

    let rust_output = Command::new(rust_bin)
        .args(["gateway", "status"])
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to run rust openclaw gateway status");
    assert!(
        rust_output.status.success(),
        "rust gateway status failed (code: {:?})\nstdout:\n{}\nstderr:\n{}",
        rust_output.status.code(),
        String::from_utf8_lossy(&rust_output.stdout),
        String::from_utf8_lossy(&rust_output.stderr)
    );

    let openclaw_output = Command::new("/usr/bin/openclaw")
        .args(["gateway", "status"])
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to run /usr/bin/openclaw gateway status");
    assert!(
        openclaw_output.status.success(),
        "openclaw gateway status failed (code: {:?})\nstdout:\n{}\nstderr:\n{}",
        openclaw_output.status.code(),
        String::from_utf8_lossy(&openclaw_output.stdout),
        String::from_utf8_lossy(&openclaw_output.stderr)
    );

    let rust_norm = normalize_gateway_status_output(&String::from_utf8_lossy(&rust_output.stdout));
    let openclaw_norm =
        normalize_gateway_status_output(&String::from_utf8_lossy(&openclaw_output.stdout));

    assert_eq!(
        rust_norm, openclaw_norm,
        "gateway status output mismatch after header normalization"
    );
}

fn normalize_gateway_status_output(raw: &str) -> String {
    let stripped = strip_ansi(raw).replace("\r\n", "\n");
    let lines: Vec<String> = stripped
        .lines()
        .map(|line| line.trim_end().to_string())
        .collect();

    let start = lines
        .iter()
        .position(|line| line.starts_with("Service:"))
        .unwrap_or(0);

    lines[start..].join("\n").trim().to_string()
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && matches!(chars.peek(), Some('[')) {
            let _ = chars.next();
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }
        out.push(ch);
    }
    out
}
