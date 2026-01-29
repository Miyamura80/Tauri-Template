# Package Management

This repository no longer relies on Python tooling. Bun is the only supported package manager for frontend/watch scripts and documentation builds.

## Workflow

- **Install deps**: `bun install` at the repo root to hydrate `bun.lock`/`bun.lockb` and keep `node_modules` aligned.
- **Run scripts**: Always use `bun run <script>` (or `bunx` when a tool isn’t installed globally) instead of `npm run`/`yarn`/`pnpm`.
- **Docs workspace**: The `docs/` site ships with its own `bun.lock`. Run `cd docs && bun install` whenever you sync the workspace.
- **Lockfile hygiene**: Treat `bun.lock` (and `docs/bun.lock`) as the single source of truth—never edit it manually; use `bun install` to update it.

## Troubleshooting

- If you see `bun: command not found`, install Bun from https://bun.sh/ and re-run `bun install`.
- To get the current Bun version, run `bun --version` so reviewers know what runtime you tested with.

## Do Not

- Do not install dependencies with `npm`, `yarn`, or `pnpm`.
- Do not reintroduce `uv`, `pip`, or other Python dependency managers; the backend is Rust and all JS tooling runs through Bun.
