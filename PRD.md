# Product Requirements Document: Python-to-Rust Tauri Migration

## 1. Overview
Refactor the existing Python-based Tauri template into a native Rust/Tauri architecture. The goal is to remove the legacy Python scaffolding (Config, Logging, DSPy/Langfuse) and replace the core infrastructure with robust Rust equivalents. This includes migrating validation tests to Rust and strictly enforcing `bun` for frontend package management.

## 2. Core Constraints
- **Runtime**: Remove all Python dependencies (`src_python`, `python_common`, `python_utils`).
- **Package Manager**: Use `bun` exclusively. Documentation and scripts must reflect this.
- **LLM Logic**: Deprecate and remove DSPy/Langfuse logic completely (no reimplementation required).
- **Testing**: Port relevant existing Python logic/unit tests (Config validation, etc.) to Rust (`cargo test`).

## 3. Architecture & File Structure Changes

### Before (Current)
```text
.
├── CLAUDE.md / AGENTS.md   # Documentation
├── Makefile                # Contains Python setup commands
├── init/                   # Python asset generation scripts
├── python_common/          # Shared Pydantic config models & YAMLs
├── python_utils/           # LLM (DSPy) & Logging logic
├── src_python/             # Legacy source
├── src-tauri/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── tests/                  # Pytest tests (Config, Env, Healthchecks)
└── ...
```

### After (Proposed)
```text
.
├── CLAUDE.md / AGENTS.md   # Updated to specify BUN only
├── src-tauri/
│   ├── Cargo.toml          # Added: config, tracing, serde_yaml
│   ├── global_config.yaml  # Moved from python_common/
│   └── src/
│       ├── lib.rs
│       ├── config.rs       # New: Rust config structs (serde)
│       └── logging.rs      # New: Tracing setup
└── ...                     # Python directories removed
```

## 4. Implementation Phases

### Phase 1: Documentation & Standards
*Establish the "Bun only" rule first.*
- [ ] Update `CLAUDE.md` to specify `bun` commands and remove Python references.
- [ ] Verify `AGENTS.md` (symlink) updates automatically.
- [ ] Update `package.json` scripts to ensure they don't default to npm/node if specific runners are needed (though `bun run` usually handles this).

### Phase 2: Rust Foundation (Config & Logging)
*Establish Rust as the backend source of truth.*
- [ ] Add crates to `src-tauri/Cargo.toml`:
    - `config`
    - `tracing`, `tracing-subscriber`
    - `serde`, `serde_json`, `serde_yaml`
- [ ] Create `src-tauri/src/config.rs`:
    - Port `python_common/config_models.py` to Rust structs using `#[derive(Deserialize)]`.
- [ ] Implement Config Loader in Rust:
    - Logic to read `global_config.yaml` and environment variables.
- [ ] Create `src-tauri/src/logging.rs`:
    - Implement `tracing` subscriber to replace `loguru`.

### Phase 3: Test Migration
*Ensure reliability before deleting Python code.*
- [ ] Analyze `tests/` for critical logic (e.g., `test_env_var_loading.py`, `test_pydantic_type_coercion.py`).
- [ ] Create `src-tauri/tests/` or unit tests within `src-tauri/src/`.
- [ ] Write Rust tests (`#[test]`) that verify:
    - Config loads correctly from YAML.
    - Environment variables override config.
    - Type coercion works as expected (Serde handling).

### Phase 4: Cleanup & Removal
*Remove legacy Python code.*
- [ ] Move `python_common/global_config.yaml` to `src-tauri/`.
- [ ] Delete `src_python/`, `python_common/`, `python_utils/`.
- [ ] Delete `init/` (Asset generation scripts).
- [ ] Delete `tests/` (Python tests).
- [ ] Remove `pyproject.toml`, `uv.lock`, `pytest.ini`.
- [ ] Clean up `Makefile` (remove Python targets).

### Phase 5: Frontend Integration
*Connect React to Rust.*
- [ ] Create a Tauri Command `get_config` in `src-tauri/src/lib.rs`.
- [ ] Create a React Hook `useConfig` in `src/hooks/` to invoke the command.
- [ ] Verify application runs with `bun run tauri dev`.

### Phase 6: Future Rust Enhancements
- [ ] Rewrite `make banner` and `make logo` logic in Rust to eliminate the remaining `uv`/Python dependency for asset generation.

## 5. Success Criteria
- [ ] `bun run tauri dev` starts the application without errors.
- [ ] Rust tests (`cargo test`) pass and cover the logic previously held in `tests/`.
- [ ] No Python files (`.py`) remain in the repository (except maybe `scripts/` if strictly necessary, but ideally converted).
- [ ] `CLAUDE.md` accurately reflects the new architecture.
