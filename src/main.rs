mod gateway_status;

use clap::Parser;
use gateway_status::{GatewayStatusOpts, run_gateway_status};
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

    /// Arguments to pass through to the system OpenClaw CLI.
    #[arg(value_name = "ARGS", num_args = 0.., trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if !cli.passthrough
        && let Some(status_opts) = parse_gateway_status_args(&cli.args)
    {
        return ExitCode::from(run_gateway_status(&status_opts) as u8);
    }

    let status = match Command::new(OPENCLAW_BIN)
        .args(&cli.args)
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

fn parse_gateway_status_args(args: &[String]) -> Option<GatewayStatusOpts> {
    if args.len() < 2 || args.first()? != "gateway" || args.get(1)? != "status" {
        return None;
    }

    let parse_input = std::iter::once(String::from("gateway-status"))
        .chain(args.iter().skip(2).cloned())
        .collect::<Vec<String>>();

    GatewayStatusOpts::try_parse_from(parse_input).ok()
}
