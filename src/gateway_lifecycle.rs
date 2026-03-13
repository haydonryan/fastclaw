use clap::Parser;
use serde_json::json;
use std::path::Path;
use std::process::Command;

const SYSTEMD_UNIT: &str = "openclaw-gateway.service";

#[derive(Debug, Clone, Parser)]
pub struct GatewayRestartOpts {
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

pub fn run_gateway_restart(opts: &GatewayRestartOpts) -> i32 {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
    let unit_path = format!("{home}/.config/systemd/user/{SYSTEMD_UNIT}");
    let loaded = Path::new(&unit_path).exists();

    if !loaded {
        if opts.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "ok": true,
                    "action": "restart",
                    "result": "not-loaded",
                    "message": "Gateway service disabled.",
                    "service": {
                        "label": "systemd",
                        "loaded": false,
                        "loadedText": "enabled",
                        "notLoadedText": "disabled"
                    }
                }))
                .unwrap_or_else(|_| "{}".to_string())
            );
        } else {
            println!("Gateway service disabled.");
        }
        return 0;
    }

    let output = Command::new("systemctl")
        .args(["--user", "restart", SYSTEMD_UNIT])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            if opts.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "ok": true,
                        "action": "restart",
                        "result": "restarted",
                        "service": {
                            "label": "systemd",
                            "loaded": true,
                            "loadedText": "enabled",
                            "notLoadedText": "disabled"
                        }
                    }))
                    .unwrap_or_else(|_| "{}".to_string())
                );
            }
            0
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            if opts.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "ok": false,
                        "action": "restart",
                        "error": format!("Gateway restart failed: {}", stderr.trim()),
                    }))
                    .unwrap_or_else(|_| "{}".to_string())
                );
            } else {
                eprintln!("Gateway restart failed: {}", stderr.trim());
            }
            1
        }
        Err(err) => {
            if opts.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "ok": false,
                        "action": "restart",
                        "error": format!("Gateway restart failed: {}", err),
                    }))
                    .unwrap_or_else(|_| "{}".to_string())
                );
            } else {
                eprintln!("Gateway restart failed: {}", err);
            }
            1
        }
    }
}
