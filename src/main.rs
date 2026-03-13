mod gateway_status;

use clap::{Parser, Subcommand};
use gateway_status::{GatewayStatusOpts, run_gateway_status};
use std::env;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitCode, Stdio};

const OPENCLAW_BIN: &str = "/usr/bin/openclaw";

#[derive(Debug, Parser)]
#[command(
    name = "openclaw",
    about = "OpenClaw CLI passthrough (Rust replacement)",
    disable_help_subcommand = true
)]
struct Cli {
    /// Force execution by passing the command to the system OpenClaw CLI.
    #[arg(short = 'p', long = "passthrough", global = true)]
    passthrough: bool,

    #[command(subcommand)]
    command: Option<TopCommand>,
}

#[derive(Debug, Subcommand)]
enum TopCommand {
    /// Run, inspect, and query the WebSocket Gateway.
    Gateway(GatewayCommand),
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Debug, Parser)]
struct GatewayCommand {
    #[command(subcommand)]
    command: Option<GatewaySubcommand>,
}

#[derive(Debug, Subcommand)]
enum GatewaySubcommand {
    /// Show gateway service status + probe the Gateway.
    Status(GatewayStatusOpts),
    #[command(external_subcommand)]
    External(Vec<String>),
}

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().collect();
    let cli = match Cli::try_parse_from(&raw_args) {
        Ok(cli) => cli,
        Err(err) => {
            use clap::error::ErrorKind;
            match err.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                    let _ = err.print();
                    return ExitCode::SUCCESS;
                }
                _ => {
                    return passthrough_args(&raw_args[1..]);
                }
            }
        }
    };

    if !cli.passthrough
        && let Some(TopCommand::Gateway(GatewayCommand {
            command: Some(GatewaySubcommand::Status(opts)),
        })) = &cli.command
    {
        return ExitCode::from(run_gateway_status(opts) as u8);
    }

    let passthrough = reconstruct_passthrough_args(cli.command);
    passthrough_args(&passthrough)
}

fn reconstruct_passthrough_args(command: Option<TopCommand>) -> Vec<String> {
    match command {
        Some(TopCommand::External(args)) => args,
        Some(TopCommand::Gateway(gw)) => match gw.command {
            Some(GatewaySubcommand::Status(opts)) => gateway_status_to_args(opts),
            Some(GatewaySubcommand::External(args)) => {
                let mut out = vec!["gateway".to_string()];
                out.extend(args);
                out
            }
            None => vec!["gateway".to_string()],
        },
        None => Vec::new(),
    }
}

fn gateway_status_to_args(opts: GatewayStatusOpts) -> Vec<String> {
    let mut out = vec!["gateway".to_string(), "status".to_string()];
    if let Some(url) = opts.url {
        out.push("--url".to_string());
        out.push(url);
    }
    if let Some(token) = opts.token {
        out.push("--token".to_string());
        out.push(token);
    }
    if let Some(password) = opts.password {
        out.push("--password".to_string());
        out.push(password);
    }
    if opts.timeout != 10_000 {
        out.push("--timeout".to_string());
        out.push(opts.timeout.to_string());
    }
    if !opts.probe {
        out.push("--no-probe".to_string());
    }
    if opts.deep {
        out.push("--deep".to_string());
    }
    if opts.json {
        out.push("--json".to_string());
    }
    out
}

fn passthrough_args(args: &[String]) -> ExitCode {
    let status = match Command::new(OPENCLAW_BIN)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => status,
        Err(err) => {
            eprintln!("failed to run {}: {}", OPENCLAW_BIN, err);
            return ExitCode::from(1);
        }
    };

    if let Some(code) = status.code() {
        return ExitCode::from(code as u8);
    }

    #[cfg(unix)]
    {
        if let Some(signal) = status.signal() {
            return ExitCode::from((128 + signal) as u8);
        }
    }

    ExitCode::from(1)
}
