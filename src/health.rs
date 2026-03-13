use clap::Parser;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Parser)]
pub struct HealthOpts {
    #[arg(long, default_value_t = false)]
    pub json: bool,
    #[arg(long, default_value_t = 10000)]
    pub timeout: u64,
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

const DEFAULT_OPENCLAW_BIN: &str = "/usr/bin/openclaw";

pub fn run_health(opts: &HealthOpts) -> i32 {
    let mut args = vec!["health".to_string()];
    if opts.json {
        args.push("--json".to_string());
    }
    if opts.timeout != 10_000 {
        args.push("--timeout".to_string());
        args.push(opts.timeout.to_string());
    }
    if opts.verbose {
        args.push("--verbose".to_string());
    }
    if opts.debug {
        args.push("--debug".to_string());
    }

    let bin = std::env::var("FASTCLAW_OPENCLAW_BIN")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_OPENCLAW_BIN.to_string());

    let status = match Command::new(&bin)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => status,
        Err(err) => {
            eprintln!("failed to run {}: {}", bin, err);
            return 1;
        }
    };

    if let Some(code) = status.code() {
        return code;
    }

    #[cfg(unix)]
    {
        if let Some(signal) = status.signal() {
            return 128 + signal;
        }
    }

    1
}
