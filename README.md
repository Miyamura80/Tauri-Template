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
