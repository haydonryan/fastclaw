use clap::Parser;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::IsTerminal;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const DEFAULT_GATEWAY_PORT: u16 = 18789;
const SYSTEMD_UNIT: &str = "openclaw-gateway.service";
const ACCENT: (u8, u8, u8) = (255, 90, 45);
const INFO: (u8, u8, u8) = (255, 138, 91);
const SUCCESS: (u8, u8, u8) = (47, 191, 113);
const WARN: (u8, u8, u8) = (255, 176, 32);
const ERROR: (u8, u8, u8) = (226, 61, 45);
const MUTED: (u8, u8, u8) = (139, 127, 119);

#[derive(Debug, Clone, Parser)]
pub struct GatewayStatusOpts {
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
    #[arg(long, default_value_t = 10000)]
    pub timeout: u64,
    #[arg(long = "no-probe", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub probe: bool,
    #[arg(long, default_value_t = false)]
    pub deep: bool,
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

#[derive(Debug, Clone)]
struct UnitInfo {
    exec_start: Option<String>,
    source_path: Option<String>,
    env: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct RuntimeInfo {
    pid: Option<u32>,
    active_state: Option<String>,
    sub_state: Option<String>,
    exit_status: Option<String>,
    result: Option<String>,
}

pub fn run_gateway_status(opts: &GatewayStatusOpts) -> i32 {
    let style = CliStyle::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
    let unit_path = format!("{home}/.config/systemd/user/{SYSTEMD_UNIT}");

    let loaded = Path::new(&unit_path).exists();
    let loaded_text = if loaded {
        "enabled".to_string()
    } else {
        "disabled".to_string()
    };

    let unit_info = parse_unit_file(Path::new(&unit_path)).unwrap_or(UnitInfo {
        exec_start: None,
        source_path: None,
        env: HashMap::new(),
    });

    let runtime = read_runtime_info();

    let cli_config = std::env::var("OPENCLAW_CONFIG_PATH")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| format!("{home}/.openclaw/openclaw.json"));
    let service_config = unit_info
        .env
        .get("OPENCLAW_CONFIG_PATH")
        .cloned()
        .unwrap_or_else(|| cli_config.clone());

    let config = read_json_config(&service_config).unwrap_or(Value::Null);
    let bind_mode = config
        .pointer("/gateway/bind")
        .and_then(Value::as_str)
        .unwrap_or("loopback")
        .to_string();
    let custom_bind_host = config
        .pointer("/gateway/customBindHost")
        .and_then(Value::as_str)
        .map(str::to_string);
    let bind_host = resolve_bind_host(&bind_mode, custom_bind_host.as_deref());

    let port_from_args = unit_info
        .exec_start
        .as_deref()
        .and_then(parse_port_from_exec_start);
    let port_from_env = unit_info
        .env
        .get("OPENCLAW_GATEWAY_PORT")
        .and_then(|v| v.parse::<u16>().ok());
    let port_from_cfg = config
        .pointer("/gateway/port")
        .and_then(Value::as_u64)
        .and_then(|v| u16::try_from(v).ok());
    let (port, port_source) = if let Some(p) = port_from_args {
        (p, "service args")
    } else if let Some(p) = port_from_env {
        (p, "service env")
    } else if let Some(p) = port_from_cfg {
        (p, "config")
    } else {
        (DEFAULT_GATEWAY_PORT, "default")
    };

    let probe_target = opts.url.clone().unwrap_or_else(|| {
        format!(
            "ws://{}:{}",
            probe_host_for_bind(&bind_mode, &bind_host),
            port
        )
    });
    let dashboard = format!("http://127.0.0.1:{port}/");
    let probe_note = match bind_mode.as_str() {
        "loopback" => Some("Loopback-only gateway; only local clients can connect.".to_string()),
        "lan" => Some("bind=lan listens on 0.0.0.0 (all interfaces).".to_string()),
        _ => None,
    };

    let rpc_ok = if opts.probe {
        probe_tcp(&probe_target, Duration::from_millis(opts.timeout))
    } else {
        None
    };

    if opts.json {
        let payload = serde_json::json!({
            "service": {
                "label": "systemd",
                "loaded": loaded,
                "loadedText": loaded_text,
                "command": unit_info.exec_start,
                "sourcePath": unit_info.source_path.as_deref().unwrap_or(unit_path.as_str()),
                "environment": unit_info.env,
                "runtime": {
                    "pid": runtime.pid,
                    "activeState": runtime.active_state,
                    "subState": runtime.sub_state,
                    "exitStatus": runtime.exit_status,
                    "result": runtime.result,
                }
            },
            "config": {
                "cli": cli_config,
                "service": service_config,
            },
            "gateway": {
                "bind": bind_mode,
                "bindHost": bind_host,
                "port": port,
                "portSource": port_source,
                "probeTarget": probe_target,
                "dashboard": dashboard,
                "probeNote": probe_note,
            },
            "rpc": {
                "enabled": opts.probe,
                "ok": rpc_ok,
            },
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string())
        );
        return if rpc_ok.unwrap_or(true) { 0 } else { 1 };
    }

    let service_status = if loaded {
        style.ok("enabled")
    } else {
        style.warn("disabled")
    };
    println!(
        "{} {} ({})",
        style.label("Service:"),
        style.accent("systemd"),
        service_status
    );
    if let Some(log_file) = resolve_log_file() {
        println!(
            "{} {}",
            style.label("File logs:"),
            style.info(shorten_home(log_file.display().to_string(), &home))
        );
    }
    if let Some(cmd) = unit_info.exec_start.as_deref() {
        println!("{} {}", style.label("Command:"), style.info(cmd));
    }
    println!(
        "{} {}",
        style.label("Service file:"),
        style.info(shorten_home(
            unit_info
                .source_path
                .as_deref()
                .unwrap_or(unit_path.as_str())
                .to_string(),
            &home
        ))
    );
    if !unit_info.env.is_empty() {
        let mut env_pairs: Vec<String> = unit_info
            .env
            .iter()
            .filter(|(k, _)| {
                matches!(
                    k.as_str(),
                    "OPENCLAW_PROFILE"
                        | "OPENCLAW_STATE_DIR"
                        | "OPENCLAW_CONFIG_PATH"
                        | "OPENCLAW_GATEWAY_PORT"
                        | "OPENCLAW_NIX_MODE"
                )
            })
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        env_pairs.sort();
        if !env_pairs.is_empty() {
            println!(
                "{} {}",
                style.label("Service env:"),
                style.info(env_pairs.join(" "))
            );
        }
    }
    println!();

    println!(
        "{} {}",
        style.label("Config (cli):"),
        style.info(shorten_home(cli_config, &home))
    );
    println!(
        "{} {}",
        style.label("Config (service):"),
        style.info(shorten_home(service_config, &home))
    );
    println!();

    println!(
        "{} bind={} ({}), port={} ({})",
        style.label("Gateway:"),
        style.info(bind_mode),
        style.info(bind_host),
        style.info(port.to_string()),
        style.info(port_source)
    );
    println!(
        "{} {}",
        style.label("Probe target:"),
        style.info(&probe_target)
    );
    println!("{} {}", style.label("Dashboard:"), style.info(&dashboard));
    if let Some(note) = probe_note.as_deref() {
        println!("{} {}", style.label("Probe note:"), style.info(note));
    }
    println!();

    println!(
        "{} {}",
        style.label("Runtime:"),
        style.runtime(
            format_runtime_line(&runtime),
            runtime.active_state.as_deref()
        )
    );
    if opts.probe {
        match rpc_ok {
            Some(true) => println!("{} {}", style.label("RPC probe:"), style.ok("ok")),
            Some(false) => println!("{} {}", style.label("RPC probe:"), style.error("failed")),
            None => println!("{} {}", style.label("RPC probe:"), style.error("failed")),
        }
        println!();
    }

    if opts.probe && matches!(rpc_ok, Some(true)) {
        if let Some((host, port)) = parse_ws_host_port(&probe_target) {
            println!(
                "{} {}",
                style.label("Listening:"),
                style.info(format!("{host}:{port}"))
            );
        }
    }

    println!(
        "{} run {}",
        style.label("Troubles:"),
        style.accent("openclaw status")
    );
    println!(
        "{} {}",
        style.label("Troubleshooting:"),
        style.info("https://docs.openclaw.ai/troubleshooting")
    );

    if opts.probe {
        if rpc_ok == Some(true) { 0 } else { 1 }
    } else {
        0
    }
}

struct CliStyle {
    rich: bool,
}

impl CliStyle {
    fn new() -> Self {
        let force_color = std::env::var("FORCE_COLOR")
            .ok()
            .map(|v| {
                let t = v.trim();
                !t.is_empty() && t != "0"
            })
            .unwrap_or(false);
        let no_color = std::env::var("NO_COLOR").ok().is_some();
        let rich = if no_color && !force_color {
            false
        } else if force_color {
            true
        } else {
            std::io::stdout().is_terminal()
        };
        Self { rich }
    }

    fn paint(&self, value: impl AsRef<str>, rgb: (u8, u8, u8), bold: bool) -> String {
        let text = value.as_ref();
        if !self.rich {
            return text.to_string();
        }
        let (r, g, b) = rgb;
        if bold {
            format!("\x1b[1;38;2;{r};{g};{b}m{text}\x1b[0m")
        } else {
            format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m")
        }
    }

    fn label(&self, value: impl AsRef<str>) -> String {
        self.paint(value, MUTED, false)
    }

    fn accent(&self, value: impl AsRef<str>) -> String {
        self.paint(value, ACCENT, false)
    }

    fn info(&self, value: impl AsRef<str>) -> String {
        self.paint(value, INFO, false)
    }

    fn ok(&self, value: impl AsRef<str>) -> String {
        self.paint(value, SUCCESS, false)
    }

    fn warn(&self, value: impl AsRef<str>) -> String {
        self.paint(value, WARN, false)
    }

    fn error(&self, value: impl AsRef<str>) -> String {
        self.paint(value, ERROR, false)
    }

    fn runtime(&self, value: impl AsRef<str>, active_state: Option<&str>) -> String {
        let color = match active_state.unwrap_or("unknown") {
            "active" => SUCCESS,
            "inactive" | "failed" => ERROR,
            "unknown" => MUTED,
            _ => WARN,
        };
        self.paint(value, color, false)
    }
}

fn run_capture<const N: usize>(argv: [&str; N]) -> io::Result<String> {
    let mut cmd = Command::new(argv[0]);
    cmd.args(&argv[1..]);
    let out = cmd.output()?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&out.stderr).to_string(),
        ))
    }
}

fn parse_unit_file(path: &Path) -> io::Result<UnitInfo> {
    let content = fs::read_to_string(path)?;
    let mut exec_start = None;
    let mut env = HashMap::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("ExecStart=") {
            if !rest.trim().is_empty() {
                exec_start = Some(rest.trim().to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("Environment=") {
            for token in split_env_tokens(rest) {
                let cleaned = strip_quotes(token.trim());
                if let Some((k, v)) = cleaned.split_once('=') {
                    env.insert(k.trim().to_string(), strip_quotes(v.trim()));
                }
            }
        }
    }

    Ok(UnitInfo {
        exec_start,
        source_path: Some(path.display().to_string()),
        env,
    })
}

fn split_env_tokens(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_quotes = false;
    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                buf.push(ch);
            }
            ' ' | '\t' if !in_quotes => {
                if !buf.trim().is_empty() {
                    out.push(buf.trim().to_string());
                    buf.clear();
                }
            }
            _ => buf.push(ch),
        }
    }
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
    out
}

fn strip_quotes(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
        s[1..bytes.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn parse_port_from_exec_start(exec_start: &str) -> Option<u16> {
    let parts: Vec<&str> = exec_start.split_whitespace().collect();
    for window in parts.windows(2) {
        if window[0] == "--port" {
            if let Ok(port) = window[1].parse::<u16>() {
                return Some(port);
            }
        }
    }
    None
}

fn read_json_config(path: &str) -> io::Result<Value> {
    let content = fs::read_to_string(path)?;
    let v: Value = serde_json::from_str(&content)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
    Ok(v)
}

fn resolve_bind_host(bind_mode: &str, custom_bind_host: Option<&str>) -> String {
    match bind_mode {
        "loopback" => "127.0.0.1".to_string(),
        "lan" => "0.0.0.0".to_string(),
        "custom" => custom_bind_host.unwrap_or("127.0.0.1").to_string(),
        _ => custom_bind_host.unwrap_or("127.0.0.1").to_string(),
    }
}

fn probe_host_for_bind(bind_mode: &str, bind_host: &str) -> String {
    match bind_mode {
        "lan" => "127.0.0.1".to_string(),
        _ => bind_host.to_string(),
    }
}

fn read_runtime_info() -> RuntimeInfo {
    let output = run_capture([
        "systemctl",
        "--user",
        "show",
        SYSTEMD_UNIT,
        "--property",
        "MainPID,ActiveState,SubState,ExecMainStatus,Result",
    ])
    .ok();

    if let Some(raw) = output {
        let mut map = HashMap::new();
        for line in raw.lines() {
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
        return RuntimeInfo {
            pid: map
                .get("MainPID")
                .and_then(|v| v.parse::<u32>().ok())
                .filter(|v| *v > 0),
            active_state: map.get("ActiveState").cloned(),
            sub_state: map.get("SubState").cloned(),
            exit_status: map.get("ExecMainStatus").cloned(),
            result: map.get("Result").cloned(),
        };
    }

    RuntimeInfo {
        pid: None,
        active_state: Some("unknown".to_string()),
        sub_state: None,
        exit_status: None,
        result: None,
    }
}

fn format_runtime_line(runtime: &RuntimeInfo) -> String {
    let status = if runtime.active_state.as_deref() == Some("active") {
        "running"
    } else if runtime.active_state.as_deref() == Some("inactive") {
        "stopped"
    } else {
        "unknown"
    };

    format!(
        "{} (pid {}, state {}, sub {}, last exit {}, reason {})",
        status,
        runtime
            .pid
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string()),
        runtime.active_state.as_deref().unwrap_or("unknown"),
        runtime.sub_state.as_deref().unwrap_or("unknown"),
        runtime.exit_status.as_deref().unwrap_or("?"),
        match runtime.result.as_deref().unwrap_or("?") {
            "success" => "0",
            other => other,
        },
    )
}

fn parse_ws_host_port(url: &str) -> Option<(String, u16)> {
    let rest = if let Some(v) = url.strip_prefix("ws://") {
        v
    } else if let Some(v) = url.strip_prefix("wss://") {
        v
    } else {
        return None;
    };
    let authority = rest.split('/').next()?;
    if let Some((host, port_str)) = authority.rsplit_once(':') {
        if let Ok(port) = port_str.parse::<u16>() {
            return Some((host.to_string(), port));
        }
    }
    None
}

fn probe_tcp(url: &str, timeout: Duration) -> Option<bool> {
    let (host, port) = parse_ws_host_port(url)?;
    let mut addrs = match (host.as_str(), port).to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(_) => return Some(false),
    };
    let addr = match addrs.next() {
        Some(addr) => addr,
        None => return Some(false),
    };
    Some(TcpStream::connect_timeout(&addr, timeout).is_ok())
}

fn resolve_log_file() -> Option<PathBuf> {
    let log_dir = Path::new("/tmp/openclaw");
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    if let Ok(entries) = fs::read_dir(log_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if !(name.starts_with("openclaw-") && name.ends_with(".log")) {
                continue;
            }
            let modified = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            match &newest {
                Some((current, _)) if &modified <= current => {}
                _ => newest = Some((modified, path)),
            }
        }
    }
    if let Some((_, path)) = newest {
        return Some(path);
    }
    let fallback = PathBuf::from("/tmp/openclaw/openclaw.log");
    if fallback.exists() {
        Some(fallback)
    } else {
        None
    }
}

fn shorten_home(input: String, home: &str) -> String {
    if home == "~" || home.is_empty() {
        return input;
    }
    if let Some(rest) = input.strip_prefix(home) {
        format!("~{}", rest)
    } else {
        input
    }
}
