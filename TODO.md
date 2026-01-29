# TODO

## Future Improvements

- [x] Rename `src-tauri/src/config.rs` to `global_config.rs`.
- [x] Automate Knip: Run `.github/workflows/knip.yml` (create if needed) on PR and push.
- [x] Automate Link Check: Run `.github/workflows/link_check.yml` (create if needed) weekly.
- [ ] Test Organization: Separate flaky, fast, and nondeterministic tests in the Rust test suite.

## Technical Debt / Cleanup

- [ ] Remove legacy Python scripts in `init/` once Rust/Node equivalents are implemented (Phase 6).
