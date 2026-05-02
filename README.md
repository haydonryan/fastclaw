# FastClaw

> Warning: this project is highly experimental and command behavior may change as the Rust implementation expands. DEPROCATED!

`fastclaw` is a replacement frontend for OpenClaw written in Rust, migrating command paths incrementally while preserving behavior.

## Status Legend
- `Native Rust`: Implemented directly in Rust.
- `Passthrough`: Delegates execution to `/usr/bin/openclaw`.

## Current State
- Global passthrough override is supported via `-p` / `--passthrough`.
- Native paths today:
  - `gateway status`
  - `gateway health`
  - `gateway restart`
- All other command paths currently delegate to `/usr/bin/openclaw`.

## Command Matrix

| Command | Subcommand | Description | Migration Status | Rust Time (real s) | Node Time (real s) |
|---|---|---|---|---|---|
| `acp` | `client` | Agent Control Protocol tools | Passthrough | `n/a` | `n/a` |
| `agent` | `none` | Run one agent turn via the Gateway | Passthrough | `n/a` | `n/a` |
| `agents` | `add`, `bind`, `bindings`, `delete`, `list`, `set-identity`, `unbind` | Manage isolated agents (workspaces, auth, routing) | Passthrough | `n/a` | `n/a` |
| `approvals` | `allowlist`, `get`, `help`, `set` | Manage exec approvals (gateway or node host) | Passthrough | `n/a` | `n/a` |
| `backup` | `create`, `help`, `verify` | Create and verify local backup archives for OpenClaw state | Passthrough | `n/a` | `n/a` |
| `browser` | `click`, `close`, `console`, `cookies`, `create-profile`, `delete-profile`, `dialog`, `download`, `drag`, `errors`, `evaluate`, `extension`, `fill`, `focus`, `highlight`, `hover`, `navigate`, `open`, `pdf`, `press`, `profiles`, `requests`, `reset-profile`, `resize`, `responsebody`, `screenshot`, `scrollintoview`, `select`, `set`, `snapshot`, `start`, `status`, `stop`, `storage`, `tab`, `tabs`, `trace`, `type`, `upload`, `wait`, `waitfordownload` | Manage OpenClaw's dedicated browser (Chrome/Chromium) | Passthrough | `n/a` | `n/a` |
| `channels` | `add`, `capabilities`, `help`, `list`, `login`, `logout`, `logs`, `remove`, `resolve`, `status` | Manage connected chat channels (Telegram, Discord, etc.) | Passthrough | `n/a` | `n/a` |
| `clawbot` | `help`, `qr` | Legacy clawbot command aliases | Passthrough | `n/a` | `n/a` |
| `completion` | `none` | Generate shell completion script | Passthrough | `n/a` | `n/a` |
| `config` | `file`, `get`, `set`, `unset`, `validate` | Non-interactive config helpers | Passthrough | `n/a` | `n/a` |
| `configure` | `none` | Interactive setup wizard | Passthrough | `n/a` | `n/a` |
| `cron` | `add`, `disable`, `edit`, `enable`, `help`, `list`, `rm`, `run`, `runs`, `status` | Manage cron jobs via the Gateway scheduler | Passthrough | `n/a` | `n/a` |
| `daemon` | `help`, `install`, `restart`, `start`, `status`, `stop`, `uninstall` | Gateway service (legacy alias) | Passthrough | `n/a` | `n/a` |
| `dashboard` | `none` | Open the Control UI with your current token | Passthrough | `n/a` | `n/a` |
| `devices` | `approve`, `clear`, `help`, `list`, `reject`, `remove`, `revoke`, `rotate` | Device pairing + token management | Passthrough | `n/a` | `n/a` |
| `directory` | `groups`, `peers`, `self` | Lookup contact and group IDs for supported channels | Passthrough | `n/a` | `n/a` |
| `dns` | `help`, `setup` | DNS helpers for wide-area discovery | Passthrough | `n/a` | `n/a` |
| `docs` | `none` | Search the live OpenClaw docs | Passthrough | `n/a` | `n/a` |
| `doctor` | `none` | Health checks + quick fixes for gateway and channels | Passthrough | `n/a` | `n/a` |
| `gateway` | `run`, `call`, `usage-cost`, `health`, `probe`, `discover`, `status`, `install`, `uninstall`, `start`, `stop`, `restart` | Run, inspect, and query the WebSocket Gateway | Partial (status/health/restart native; others passthrough) | `n/a` | `n/a` |
| `health` | `none` | Fetch health from the running gateway | Native Rust (exact delegated output) | `6.24` | `6.51` |
| `help` | `none` | Display help for command | Passthrough | `n/a` | `n/a` |
| `hooks` | `check`, `disable`, `enable`, `info`, `install`, `list`, `update` | Manage internal agent hooks | Passthrough | `n/a` | `n/a` |
| `logs` | `none` | Tail gateway file logs via RPC | Passthrough | `n/a` | `n/a` |
| `memory` | `help`, `index`, `search`, `status` | Search and reindex memory files | Passthrough | `n/a` | `n/a` |
| `message` | `ban`, `broadcast`, `channel`, `delete`, `edit`, `emoji`, `event`, `kick`, `member`, `permissions`, `pin`, `pins`, `poll`, `react`, `reactions`, `read`, `role`, `search`, `send`, `sticker`, `thread`, `timeout`, `unpin`, `voice` | Send, read, and manage messages | Passthrough | `n/a` | `n/a` |
| `models` | `aliases`, `auth`, `fallbacks`, `image-fallbacks`, `list`, `scan`, `set`, `set-image`, `status` | Discover, scan, and configure models | Passthrough | `n/a` | `n/a` |
| `node` | `help`, `install`, `restart`, `run`, `status`, `stop`, `uninstall` | Run and manage the headless node host service | Passthrough | `n/a` | `n/a` |
| `nodes` | `approve`, `camera`, `canvas`, `describe`, `help`, `invoke`, `list`, `location`, `notify`, `pending`, `push`, `reject`, `rename`, `run`, `screen`, `status` | Manage gateway-owned node pairing and node commands | Passthrough | `n/a` | `n/a` |
| `onboard` | `none` | Interactive onboarding wizard | Passthrough | `n/a` | `n/a` |
| `pairing` | `none` | Secure DM pairing (approve inbound requests) | Passthrough | `n/a` | `n/a` |
| `plugins` | `disable`, `doctor`, `enable`, `help`, `info`, `install`, `list`, `uninstall`, `update` | Manage OpenClaw plugins and extensions | Passthrough | `n/a` | `n/a` |
| `qr` | `none` | Generate iOS pairing QR/setup code | Passthrough | `n/a` | `n/a` |
| `reset` | `none` | Reset local config/state (keeps CLI installed) | Passthrough | `n/a` | `n/a` |
| `sandbox` | `explain`, `list`, `recreate` | Manage sandbox containers for agent isolation | Passthrough | `n/a` | `n/a` |
| `secrets` | `apply`, `audit`, `configure`, `help`, `reload` | Secrets runtime reload controls | Passthrough | `n/a` | `n/a` |
| `security` | `audit`, `help` | Security tools and local config audits | Passthrough | `n/a` | `n/a` |
| `sessions` | `cleanup` | List stored conversation sessions | Passthrough | `n/a` | `n/a` |
| `setup` | `none` | Initialize local config and agent workspace | Passthrough | `n/a` | `n/a` |
| `skills` | `check`, `info`, `list` | List and inspect available skills | Passthrough | `n/a` | `n/a` |
| `status` | `none` | Show channel health and recent session recipients | Passthrough | `n/a` | `n/a` |
| `system` | `event`, `heartbeat`, `help`, `presence` | System events, heartbeat, and presence | Passthrough | `n/a` | `n/a` |
| `tui` | `none` | Open a terminal UI connected to the Gateway | Passthrough | `n/a` | `n/a` |
| `uninstall` | `none` | Uninstall gateway service + local data (CLI remains) | Passthrough | `n/a` | `n/a` |
| `update` | `status`, `wizard` | Update OpenClaw and inspect update channel status | Passthrough | `n/a` | `n/a` |
| `webhooks` | `gmail`, `help` | Webhook helpers and integrations | Passthrough | `n/a` | `n/a` |

## OpenClaw Baseline
- Baseline version: `OpenClaw 2026.3.12 (6472949)`
- Validation: `tests/readme_openclaw_version.rs` runs `/usr/bin/openclaw --help`, extracts `OpenClaw <version> (<build>)`, and verifies this README contains it.

## Performance Goal
Primary goal: reduce CLI startup and command latency.

Latest measured wall times are captured in the Command Matrix columns (`Rust Time (real s)` and `Node Time (real s)`).

## Source
Command list captured from `/usr/bin/openclaw --help` on 2026-03-13.
