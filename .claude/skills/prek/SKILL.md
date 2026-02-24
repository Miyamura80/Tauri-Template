---
name: prek
description: Run pre-commit hooks using prek
user_invocable: true
triggers:
  - /prek
---

# Prek Skill

Runs pre-commit hooks using [prek](https://prek.j178.dev/) to ensure code quality (formatting, linting, static analysis).

## Workflow

### 1. Run Hooks

```bash
make prek
```

### 2. Handle Results

- **Passed**: All checks succeeded.
- **Failed (Auto-fixable)**: `prek` might have automatically fixed some issues (e.g., trailing whitespace, end-of-file, biome formatting). Stage the changes and retry.
- **Failed (Manual Fix)**: Fix reported errors (e.g., clippy warnings, biome lint errors) and retry.

## Notes

- Configuration is in `prek.toml`.
- Uses `bun run prek` internally via `Makefile`.
- `cargo-clippy` may fail locally if system dependencies (GTK/WebKit) are missing, but is verified in CI.
