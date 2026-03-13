# FastClaw Migration Tracker

This project is building `fastclaw`, a Rust replacement frontend for the OpenClaw CLI, using `clap` with incremental command migration from the Node implementation.

## Status Legend
- `Native Rust`: Implemented directly in Rust.
- `Passthrough`: Delegates execution to `/usr/bin/openclaw`.

## Current State
- Global passthrough override is supported via `-p` / `--passthrough`.
- Migrated command paths:
  - `openclaw gateway status`
  - `openclaw gateway restart`
- All other commands currently run through passthrough.

## Command Migration Table

| Command | Description | Migration Status |
|---|---|---|
| `acp` | Agent Control Protocol tools | Passthrough |
| `agent` | Run one agent turn via the Gateway | Passthrough |
| `agents` | Manage isolated agents (workspaces, auth, routing) | Passthrough |
| `approvals` | Manage exec approvals (gateway or node host) | Passthrough |
| `backup` | Create and verify local backup archives for OpenClaw state | Passthrough |
| `browser` | Manage OpenClaw's dedicated browser (Chrome/Chromium) | Passthrough |
| `channels` | Manage connected chat channels (Telegram, Discord, etc.) | Passthrough |
| `clawbot` | Legacy clawbot command aliases | Passthrough |
| `completion` | Generate shell completion script | Passthrough |
| `config` | Non-interactive config helpers | Passthrough |
| `configure` | Interactive setup wizard | Passthrough |
| `cron` | Manage cron jobs via the Gateway scheduler | Passthrough |
| `daemon` | Gateway service (legacy alias) | Passthrough |
| `dashboard` | Open the Control UI with your current token | Passthrough |
| `devices` | Device pairing + token management | Passthrough |
| `directory` | Lookup contact and group IDs for supported channels | Passthrough |
| `dns` | DNS helpers for wide-area discovery | Passthrough |
| `docs` | Search the live OpenClaw docs | Passthrough |
| `doctor` | Health checks + quick fixes for gateway and channels | Passthrough |
| `gateway` | Run, inspect, and query the WebSocket Gateway | Passthrough |
| `gateway status` | Show gateway runtime and probe status | Native Rust |
| `gateway restart` | Restart the gateway service | Native Rust |
| `health` | Fetch health from the running gateway | Passthrough |
| `help` | Display help for command | Passthrough |
| `hooks` | Manage internal agent hooks | Passthrough |
| `logs` | Tail gateway file logs via RPC | Passthrough |
| `memory` | Search and reindex memory files | Passthrough |
| `message` | Send, read, and manage messages | Passthrough |
| `models` | Discover, scan, and configure models | Passthrough |
| `node` | Run and manage the headless node host service | Passthrough |
| `nodes` | Manage gateway-owned node pairing and node commands | Passthrough |
| `onboard` | Interactive onboarding wizard | Passthrough |
| `pairing` | Secure DM pairing (approve inbound requests) | Passthrough |
| `plugins` | Manage OpenClaw plugins and extensions | Passthrough |
| `qr` | Generate iOS pairing QR/setup code | Passthrough |
| `reset` | Reset local config/state (keeps CLI installed) | Passthrough |
| `sandbox` | Manage sandbox containers for agent isolation | Passthrough |
| `secrets` | Secrets runtime reload controls | Passthrough |
| `security` | Security tools and local config audits | Passthrough |
| `sessions` | List stored conversation sessions | Passthrough |
| `setup` | Initialize local config and agent workspace | Passthrough |
| `skills` | List and inspect available skills | Passthrough |
| `status` | Show channel health and recent session recipients | Passthrough |
| `system` | System events, heartbeat, and presence | Passthrough |
| `tui` | Open a terminal UI connected to the Gateway | Passthrough |
| `uninstall` | Uninstall gateway service + local data (CLI remains) | Passthrough |
| `update` | Update OpenClaw and inspect update channel status | Passthrough |
| `webhooks` | Webhook helpers and integrations | Passthrough |

## Why This Migration
Primary goal: reduce CLI startup and command latency.

Observed local example (`gateway status`):
- Rust CLI: about `0.016s` wall time
- Node CLI: about `8.273s` wall time

## Source
Command list captured from `/usr/bin/openclaw --help` on 2026-03-13.
