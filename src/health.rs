use clap::Parser;
use serde_json::Value;
use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::time::{Duration, Instant};

const DEFAULT_GATEWAY_PORT: u16 = 18789;

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

pub fn run_health(opts: &HealthOpts) -> i32 {
    let cfg_path = resolve_config_path();
    let cfg = read_json_config(cfg_path.as_path()).unwrap_or(Value::Null);
    let bind_mode = cfg
        .pointer("/gateway/bind")
        .and_then(Value::as_str)
        .unwrap_or("loopback");
    let custom_bind_host = cfg
        .pointer("/gateway/customBindHost")
        .and_then(Value::as_str)
        .unwrap_or("127.0.0.1");
    let port = cfg
        .pointer("/gateway/port")
        .and_then(Value::as_u64)
        .and_then(|v| u16::try_from(v).ok())
        .unwrap_or_else(|| {
            std::env::var("OPENCLAW_GATEWAY_PORT")
                .ok()
                .and_then(|v| v.parse::<u16>().ok())
                .unwrap_or(DEFAULT_GATEWAY_PORT)
        });

    let host = match bind_mode {
        "custom" => custom_bind_host,
        _ => "127.0.0.1",
    };
    let url = format!("ws://{host}:{port}");
    let started = Instant::now();
    let ok = probe_tcp(host, port, Duration::from_millis(opts.timeout));
    let duration_ms = started.elapsed().as_millis() as u64;

    if opts.json {
        let payload = serde_json::json!({
            "ok": ok,
            "ts": now_ms(),
            "durationMs": duration_ms,
            "gateway": {
                "url": url,
                "configPath": cfg_path.display().to_string(),
                "bind": bind_mode,
            }
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string())
        );
        return if ok { 0 } else { 1 };
    }

    println!("Gateway Health");
    if ok {
        println!("OK ({duration_ms}ms)");
        return 0;
    }

    eprintln!("Gateway health failed");
    eprintln!("Gateway target: {url}");
    eprintln!("Source: local {bind_mode}");
    eprintln!("Config: {}", cfg_path.display());
    eprintln!("Bind: {bind_mode}");
    1
}

fn resolve_config_path() -> PathBuf {
    if let Ok(path) = std::env::var("OPENCLAW_CONFIG_PATH")
        && !path.trim().is_empty()
    {
        return PathBuf::from(path);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
    PathBuf::from(format!("{home}/.openclaw/openclaw.json"))
}

fn read_json_config(path: &std::path::Path) -> Result<Value, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let v: Value = serde_json::from_str(&content)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;
    Ok(v)
}

fn probe_tcp(host: &str, port: u16, timeout: Duration) -> bool {
    let mut addrs = match (host, port).to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(_) => return false,
    };
    let Some(addr) = addrs.next() else {
        return false;
    };
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
