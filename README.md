# Tauri-Template

<p align="center">
  <img src="media/banner.png" alt="banner" width="400">
</p>

<p align="center">
<b>agent ready tauri template</b>
</p>

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#credits">Credits</a> •
  <a href="#about-the-core-contributors">About the Core Contributors</a>
</p>

<p align="center">
  <img alt="Project Version" src="https://img.shields.io/github/v/release/Miyamura80/Tauri-Template?label=version&color=blue">
  <img alt="Rust Version" src="https://img.shields.io/badge/rust-1.75%2B-blue?logo=rust">
  <img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/Miyamura80/Tauri-Template">
  <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Miyamura80/Tauri-Template/rust_checks.yaml?branch=main">
</p>

--- 

## Key Features

Modern stack for cross-platform desktop application development.

| Feature | Tech Stack |
|---------|:----------:|
| **Frontend** | React + TypeScript + Vite |
| **Backend** | Native Rust (Tauri v2) |
| **Engine** | Shared Rust crate (no Tauri dep) |
| **CLI Test Harness** | `appctl` – headless engine runner |
| **Config** | Rust `config` crate (YAML + Env) |
| **Logging** | `tracing` + Redaction Layer |
| **Package Manager** | Bun |
| **Formatting** | Biome + `cargo fmt` |

## Quick Start

1. **Initialize Project**:
   ```bash
   make init name=my-app description="My cool app"
   ```

2. **Install Dependencies**:
   ```bash
   bun install
   ```

3. **Run in Development**:
   ```bash
   bun run tauri dev
   ```

4. **Build**:
   ```bash
   bun run tauri build
   ```

## Asset Generation

- Use `make logo` / `make banner` to regenerate branding assets once per project. The targets run the Rust `asset-gen` CLI and require `APP__GEMINI_API_KEY` (set via `.env`).
- Logos/icons land under `docs/public/`, while the banner image is written to `media/banner.png`.

## CLI Test Harness (`appctl`)

The repo includes a headless CLI that invokes the same engine logic as the GUI,
designed for compatibility testing on real VMs (macOS + Linux) without a window
server.

### Architecture

```
┌──────────────┐    ┌──────────────┐
│  Tauri GUI   │    │  appctl CLI  │
│  (src-tauri) │    │  (crates/cli)│
└──────┬───────┘    └──────┬───────┘
       │                   │
       └───────┬───────────┘
               │
       ┌───────▼───────┐
       │    engine      │
       │ (crates/engine)│
       │                │
       │ Commands │ Probes │ Doctor │
       │ Traits: FS, Net, Clipboard │
       └───────────────┘
```

All backend logic lives in the `engine` crate. Both the Tauri app and the CLI
are thin wrappers. The engine uses trait objects for OS capabilities, so headless
environments get structured `SKIP`/`UNSUPPORTED` results instead of crashes.

### Quick Usage

```bash
# Build the CLI
cargo build -p appctl

# Run environment diagnostics
appctl doctor --json

# Invoke engine commands
appctl call ping --json
appctl call read_file --args '{"path": "/etc/hostname"}' --json

# Probe OS capabilities
appctl probe filesystem --json
appctl probe network --json
appctl probe clipboard --json    # SKIP in headless

# Run a scripted scenario
appctl run-scenario crates/cli/examples/smoke_test.yaml --json

# Start daemon mode (Unix socket)
appctl serve --socket /tmp/appctl.sock

# Desktop event simulation (skeleton, returns UNIMPLEMENTED)
appctl emit tray-click --json
```

Every command outputs a stable JSON result with `run_id`, `status`, `error`,
`timing_ms`, and `env_summary`. Use `--artifacts <dir>` to persist
`result.json` and `events.jsonl` per run.

See [`crates/cli/README.md`](crates/cli/README.md) for full documentation.

## Configuration

Configuration is handled in Rust and exposed to the frontend via Tauri commands.

- **Rust**: Access via `crate::global_config::get_config()`
- **Frontend**: Access via `useConfig()` hook

### Environment Variables
Prefix variables with `APP__` to override YAML settings (e.g., `APP__MODEL_NAME=gpt-4`).

## Credits

This software uses the following tools:
- [Tauri](https://tauri.app/)
- [Bun](https://bun.sh/)
- [Biome](https://biomejs.dev/)
- [Rust](https://www.rust-lang.org/)

## About the Core Contributors

<a href="https://github.com/Miyamura80/Tauri-Template/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Miyamura80/Tauri-Template" />
</a>

Made with [contrib.rocks](https://contrib.rocks).
