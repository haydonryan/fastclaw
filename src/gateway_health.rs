use clap::Parser;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};

const DEFAULT_OPENCLAW_BIN: &str = "/usr/bin/openclaw";

#[derive(Debug, Clone, Parser)]
pub struct GatewayHealthOpts {
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
    #[arg(long, default_value_t = 10000)]
    pub timeout: u64,
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

pub fn run_gateway_health(opts: &GatewayHealthOpts) -> i32 {
    let mut args = vec!["gateway".to_string(), "health".to_string()];
    if let Some(url) = &opts.url {
        args.push("--url".to_string());
        args.push(url.clone());
    }
    if let Some(token) = &opts.token {
        args.push("--token".to_string());
        args.push(token.clone());
    }
    if let Some(password) = &opts.password {
        args.push("--password".to_string());
        args.push(password.clone());
    }
    if opts.timeout != 10_000 {
        args.push("--timeout".to_string());
        args.push(opts.timeout.to_string());
    }
    if opts.json {
        args.push("--json".to_string());
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
