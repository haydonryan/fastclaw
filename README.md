# FastClaw

`fastclaw` is a replacement frontend for OpenClaw written in Rust, migrating command paths incrementally while preserving behavior.

## Status Legend
- `Native Rust`: Implemented directly in Rust.
- `Passthrough`: Delegates execution to `/usr/bin/openclaw`.

## Current State
- Global passthrough override is supported via `-p` / `--passthrough`.
- Native paths today:
  - `gateway status`
  - `gateway restart`
  - `health`
- All other command paths currently delegate to `/usr/bin/openclaw`.

## Command Matrix

| Command | Subcommand | Description | Migration Status |
|---|---|---|---|
| `acp` | `client` | Agent Control Protocol tools | Passthrough |
| `agent` | `none` | Run one agent turn via the Gateway | Passthrough |
| `agents` | `add`, `bind`, `bindings`, `delete`, `list`, `set-identity`, `unbind` | Manage isolated agents (workspaces, auth, routing) | Passthrough |
| `approvals` | `allowlist`, `get`, `help`, `set` | Manage exec approvals (gateway or node host) | Passthrough |
| `backup` | `create`, `help`, `verify` | Create and verify local backup archives for OpenClaw state | Passthrough |
| `browser` | `click`, `close`, `console`, `cookies`, `create-profile`, `delete-profile`, `dialog`, `download`, `drag`, `errors`, `evaluate`, `extension`, `fill`, `focus`, `highlight`, `hover`, `navigate`, `open`, `pdf`, `press`, `profiles`, `requests`, `reset-profile`, `resize`, `responsebody`, `screenshot`, `scrollintoview`, `select`, `set`, `snapshot`, `start`, `status`, `stop`, `storage`, `tab`, `tabs`, `trace`, `type`, `upload`, `wait`, `waitfordownload` | Manage OpenClaw's dedicated browser (Chrome/Chromium) | Passthrough |
| `channels` | `add`, `capabilities`, `help`, `list`, `login`, `logout`, `logs`, `remove`, `resolve`, `status` | Manage connected chat channels (Telegram, Discord, etc.) | Passthrough |
| `clawbot` | `help`, `qr` | Legacy clawbot command aliases | Passthrough |
| `completion` | `none` | Generate shell completion script | Passthrough |
| `config` | `file`, `get`, `set`, `unset`, `validate` | Non-interactive config helpers | Passthrough |
| `configure` | `none` | Interactive setup wizard | Passthrough |
| `cron` | `add`, `disable`, `edit`, `enable`, `help`, `list`, `rm`, `run`, `runs`, `status` | Manage cron jobs via the Gateway scheduler | Passthrough |
| `daemon` | `help`, `install`, `restart`, `start`, `status`, `stop`, `uninstall` | Gateway service (legacy alias) | Passthrough |
| `dashboard` | `none` | Open the Control UI with your current token | Passthrough |
| `devices` | `approve`, `clear`, `help`, `list`, `reject`, `remove`, `revoke`, `rotate` | Device pairing + token management | Passthrough |
| `directory` | `groups`, `peers`, `self` | Lookup contact and group IDs for supported channels | Passthrough |
| `dns` | `help`, `setup` | DNS helpers for wide-area discovery | Passthrough |
| `docs` | `none` | Search the live OpenClaw docs | Passthrough |
| `doctor` | `none` | Health checks + quick fixes for gateway and channels | Passthrough |
| `gateway` | `run`, `call`, `usage-cost`, `health`, `probe`, `discover`, `status`, `install`, `uninstall`, `start`, `stop`, `restart` | Run, inspect, and query the WebSocket Gateway | Partial (status/restart native; others passthrough) |
| `health` | `none` | Fetch health from the running gateway | Native Rust |
| `help` | `none` | Display help for command | Passthrough |
| `hooks` | `check`, `disable`, `enable`, `info`, `install`, `list`, `update` | Manage internal agent hooks | Passthrough |
| `logs` | `none` | Tail gateway file logs via RPC | Passthrough |
| `memory` | `help`, `index`, `search`, `status` | Search and reindex memory files | Passthrough |
| `message` | `ban`, `broadcast`, `channel`, `delete`, `edit`, `emoji`, `event`, `kick`, `member`, `permissions`, `pin`, `pins`, `poll`, `react`, `reactions`, `read`, `role`, `search`, `send`, `sticker`, `thread`, `timeout`, `unpin`, `voice` | Send, read, and manage messages | Passthrough |
| `models` | `aliases`, `auth`, `fallbacks`, `image-fallbacks`, `list`, `scan`, `set`, `set-image`, `status` | Discover, scan, and configure models | Passthrough |
| `node` | `help`, `install`, `restart`, `run`, `status`, `stop`, `uninstall` | Run and manage the headless node host service | Passthrough |
| `nodes` | `approve`, `camera`, `canvas`, `describe`, `help`, `invoke`, `list`, `location`, `notify`, `pending`, `push`, `reject`, `rename`, `run`, `screen`, `status` | Manage gateway-owned node pairing and node commands | Passthrough |
| `onboard` | `none` | Interactive onboarding wizard | Passthrough |
| `pairing` | `none` | Secure DM pairing (approve inbound requests) | Passthrough |
| `plugins` | `disable`, `doctor`, `enable`, `help`, `info`, `install`, `list`, `uninstall`, `update` | Manage OpenClaw plugins and extensions | Passthrough |
| `qr` | `none` | Generate iOS pairing QR/setup code | Passthrough |
| `reset` | `none` | Reset local config/state (keeps CLI installed) | Passthrough |
| `sandbox` | `explain`, `list`, `recreate` | Manage sandbox containers for agent isolation | Passthrough |
| `secrets` | `apply`, `audit`, `configure`, `help`, `reload` | Secrets runtime reload controls | Passthrough |
| `security` | `audit`, `help` | Security tools and local config audits | Passthrough |
| `sessions` | `cleanup` | List stored conversation sessions | Passthrough |
| `setup` | `none` | Initialize local config and agent workspace | Passthrough |
| `skills` | `check`, `info`, `list` | List and inspect available skills | Passthrough |
| `status` | `none` | Show channel health and recent session recipients | Passthrough |
| `system` | `event`, `heartbeat`, `help`, `presence` | System events, heartbeat, and presence | Passthrough |
| `tui` | `none` | Open a terminal UI connected to the Gateway | Passthrough |
| `uninstall` | `none` | Uninstall gateway service + local data (CLI remains) | Passthrough |
| `update` | `status`, `wizard` | Update OpenClaw and inspect update channel status | Passthrough |
| `webhooks` | `gmail`, `help` | Webhook helpers and integrations | Passthrough |

## OpenClaw Baseline
- Baseline version: `OpenClaw 2026.3.12 (6472949)`
- Validation: `tests/readme_openclaw_version.rs` runs `/usr/bin/openclaw --help`, extracts `OpenClaw <version> (<build>)`, and verifies this README contains it.

## Performance Goal
Primary goal: reduce CLI startup and command latency.

Observed local example (`gateway status`):
- Rust CLI: about `0.016s` wall time
- Node CLI: about `8.273s` wall time

## Source
Command list captured from `/usr/bin/openclaw --help` on 2026-03-13.
