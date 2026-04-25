mod support;

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn readme_contains_openclaw_baseline_version_from_help() {
    if !Path::new("/usr/bin/openclaw").exists() {
        return;
    }

    let output = Command::new("/usr/bin/openclaw")
        .arg("--help")
        .output()
        .expect("failed to run /usr/bin/openclaw --help");

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let cleaned = strip_ansi(&combined);
    let version = extract_openclaw_version(&cleaned).expect("failed to parse OpenClaw version");

    let readme_path = support::crate_root().join("README.md");
    let readme = fs::read_to_string(readme_path).expect("failed to read README.md");

    assert!(
        readme.contains("# FastClaw"),
        "README title is not FastClaw"
    );
    assert!(
        readme.contains("Rust Time (real s)") && readme.contains("Node Time (real s)"),
        "README is missing timing columns"
    );
    assert!(
        readme.contains(&version),
        "README is missing baseline version '{}'",
        version
    );
}

fn extract_openclaw_version(text: &str) -> Option<String> {
    for line in text.lines() {
        let Some(idx) = line.find("OpenClaw ") else {
            continue;
        };
        let tail = &line[idx..];
        let Some(end) = tail.find(')') else {
            continue;
        };
        let candidate = tail[..=end].trim();
        if candidate.starts_with("OpenClaw ") {
            return Some(candidate.to_string());
        }
    }
    None
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
