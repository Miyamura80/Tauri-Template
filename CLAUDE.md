# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tauri template for desktop application development with React and TypeScript.
**Note:** This project has migrated away from Python. Use Rust for backend logic and Node/Bun for frontend/scripts.

## Common Commands

```bash
# Frontend / Tauri
bun install             # Install dependencies
bun run tauri dev       # Run the app in development mode
bun run build           # Build the frontend
bun run tauri build     # Build the Tauri application
bun run check           # Run formatting and linting (Biome)

# Rust / Backend
cargo test              # Run Rust tests
cargo check             # Check Rust code
cargo clippy            # Run Rust linter
```

## Architecture

- **src/** - Tauri frontend (React + TypeScript + Vite)
- **src-tauri/** - Tauri backend (Rust)
  - **src/global_config.rs** - Application configuration (migrated from Python)
  - **src/logging.rs** - Tracing setup
  - **src/lib.rs** - Main library entry point
- **docs/** - Documentation (Next.js app)

## Code Style

### TypeScript (Frontend)
- `camelCase` for functions/variables
- `PascalCase` for components/classes
- Use Biome for formatting/linting

### Rust (Backend)
- `snake_case` for functions/modules/variables
- `PascalCase` for structs/enums
- Follow standard Rust formatting (`cargo fmt`)

## Configuration Pattern

Configuration is handled in Rust and exposed to the frontend.
Source of truth: `src-tauri/global_config.yaml` (and `.env` overrides).

```rust
// Accessing config in Rust
let config = crate::global_config::get_config();
println!("Model: {}", config.default_llm.default_model);
```

## Commit Message Convention

Use emoji prefixes indicating change type and magnitude (multiple emojis = 5+ files):
- 🏗️ initial implementation
- 🔨 feature changes
- 🐛 bugfix
- ✨ formatting/linting only
- ✅ feature complete with E2E tests
- ⚙️ config changes
- 💽 DB schema/migrations

## Long-Running Code Pattern

Structure as: `init()` → `continue(id)` → `cleanup(id)`
- Keep state serializable
- Use descriptive IDs (runId, taskId)
- Handle rate limits, timeouts, retries at system boundaries

## Git Workflow
- **Review**: Always trigger Greptile review MCP before pushing a PR and resolve any branch issues.
- **Protected Branch**: `main` is protected. Do not push directly to `main`. Use PRs.
- **Merge Strategy**: Squash and merge.
