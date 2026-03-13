use clap::Parser;
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
    let _force_passthrough = cli.passthrough;

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
