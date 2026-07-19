---
name: onboarding
description: Interview the user, inspect this Tauri desktop template, run headless setup and rename, and prune unused systems so a new Tauri (Rust + Bun/React) project gets running quickly.
---

# Onboarding

Use this skill when the user wants to turn this Tauri desktop template into a
real project, especially when they invoke `/onboarding`, ask to run onboarding,
or want to remove unused template systems.

This template ships a Rust backend (`src-tauri/` host + `crates/engine` +
`crates/cli`) and a Bun/React frontend (`src/`), wired together by Tauri. The
onboarding entrypoints are the `Makefile` targets `make setup` and
`make init` — read the `Makefile` before running anything so you know exactly
what each target rewrites.

## Workflow

1. Inspect the repo before changing anything:
   - `AGENTS.md` / `CLAUDE.md`, `README.md`, the workspace `Cargo.toml`, and the
     `Makefile` (`setup`, `init`, `banner`, `logo`, `ci` targets).
   - Manifests the rename touches: `package.json`, `src-tauri/tauri.conf.json`,
     `src-tauri/Cargo.toml`.
   - The surfaces themselves: the engine command registry (`crates/engine/`),
     the headless CLI harness (`crates/cli/`, the `appctl` binary), the Tauri
     host (`src-tauri/src/`), the React frontend (`src/`), config
     (`src-tauri/global_config.yaml`), and the docs site (`docs/`).
   - Config source of truth: `src-tauri/global_config.yaml`, overridable with
     `APP__`-prefixed env vars in `.env`.

2. Interview the user briefly. Prefer grouped, high-signal questions whose
   answers change what you do:
   - Project identity: the new name (kebab-case, used for the package name,
     Cargo crate, and Tauri `productName`/bundle identifier) and a one-line
     description.
   - Which surfaces they keep: desktop app only, or also the headless CLI
     (`appctl`) harness for testing engine logic.
   - Whether they want the `docs/` site (separate Next.js dependency tree) and
     generated media (banner/logo/icons).

3. Set up the toolchain and environment first:
   - `make setup` — verifies `rustup` and `bun` are installed, runs
     `bun install`, and copies `.env.example` to `.env` if missing. Have the
     user fill in any API keys in `.env` before running the app.

4. Rename the project with the headless target. `make init` requires both
   parameters and rewrites `package.json`, `src-tauri/tauri.conf.json` (product
   name + `com.<user>.<name>` identifier), `src-tauri/Cargo.toml`, and the
   `README.md` title/description:
   - `make init name=<project-name> description="<one-line description>"`
   - Confirm the resolved name and identifier with the user first; the target
     edits tracked manifests in place.

5. Generate branding assets only if the user wants them (both need
   `APP__GEMINI_API_KEY` in `.env`):
   - `make banner` — writes `media/banner.png`.
   - `make logo` — writes logo, icons, and favicon into `docs/public/`.

6. Verify the selected shape:
   - Desktop app: `bun run tauri dev` launches the app (needs system WebView /
     GTK libraries present).
   - Frontend only: `make dev` runs the Vite dev server.
   - Backend logic: `cd src-tauri && cargo test`, and exercise the engine
     through the CLI harness (`crates/cli`, the `appctl` binary).
   - Broad changes: run `make ci` (Biome + Clippy + knip + tests + link/length
     checks) once the scope is large.

7. Prune systems the user opted out of, after confirming with them. There is no
   automatic pruner in this template — remove by hand and keep the build green:
   - Docs site — delete `docs/` and drop it from the `workspaces` array in
     `package.json`.
   - Headless CLI — if they only want the desktop app, remove `crates/cli` from
     the workspace `Cargo.toml` `members` and delete the crate.
   - Release / translation workflows under `.github/workflows/` are not wired
     into any pruner; update or remove them to match the kept surfaces.
   - After any prune, run `make ci` (or at minimum `make lint` and `make test`)
     and fix fallout before committing.

## Guardrails

- Do not delete the frontend, the docs site, release workflows, or any surface
  without explicit user confirmation.
- Do not push to `main`, force-push, or run destructive git commands. Onboarding
  never commits or pushes on the user's behalf.
- Confirm the resolved project name and bundle identifier before running the
  non-reversible `make init` rename.
- Prefer the `Makefile` targets over hand-editing manifests so all copies of the
  name stay in sync.
- Keep the docs site and generated media framed as optional extras, not core
  template infrastructure.
