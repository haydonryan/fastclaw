# AGENTS.md

## Mission
Build `fastclaw`, a replacement OpenClaw CLI in Rust using `clap`, progressively migrating commands from the existing Node CLI.

## Non-Negotiable Rules
1. For every new addition, add a validation test in the same change.
2. After every task, run `cargo fmt`.
3. Preserve command behavior parity with `/usr/bin/openclaw` unless a migration explicitly changes behavior.
4. Keep `--passthrough` / `-p` working so users can force delegation to `/usr/bin/openclaw`.

## Migration Policy
1. Migrate one command path at a time.
2. Prefer exact output parity for migrated commands.
3. For parity checks, compare Rust CLI output against `/usr/bin/openclaw` output.
4. Update `README.md` migration table whenever command status changes.

## Testing Expectations
1. Add at least one validation test per addition.
2. For migrated commands, include a parity-style test when practical.
3. New tests should be runnable via `cargo test`.

## Current State (as of 2026-03-13)
1. `gateway status` is implemented natively in Rust.
2. Other command paths still default to passthrough.
