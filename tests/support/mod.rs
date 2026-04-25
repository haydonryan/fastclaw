#![allow(dead_code)]
use std::path::PathBuf;

pub fn fastclaw_bin() -> PathBuf {
    let exe = std::env::current_exe().expect("current test exe path");
    let profile_dir = exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("target/{debug,release} dir");
    let bin = profile_dir.join(format!("fastclaw{}", std::env::consts::EXE_SUFFIX));
    assert!(
        bin.exists(),
        "fastclaw binary not found at {}",
        bin.display()
    );
    bin
}

pub fn crate_root() -> PathBuf {
    let mut dir = std::env::current_exe()
        .expect("current test exe path")
        .parent()
        .expect("test exe dir")
        .to_path_buf();
    loop {
        if dir.join("Cargo.toml").exists() {
            return dir;
        }
        let Some(parent) = dir.parent() else {
            panic!("failed to locate crate root");
        };
        dir = parent.to_path_buf();
    }
}
