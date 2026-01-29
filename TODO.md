# TODO

## Integrations

- [ ] Integrate [Strix](https://github.com/usestrix/strix) - **Requires human supervision**
- [ ] Re-init Postgres API key due to potential leak

## Code Quality

### High Priority

- [x] Broad exception handling in `src/utils/logging_config.py:64` - catches all exceptions indiscriminately
- [x] Validator anti-pattern in `python_common/global_config.py:188-198` - validators ignore input parameter `v`
- [x] Circular import risk in `python_common/flags.py:6-7` - `setup_logging()` called at import time
- [x] Broad exceptions in `utils/llm/dspy_langfuse.py:280,321,438` - catches generic `Exception`
- [x] Unsafe exception re-instantiation in `src/utils/logging_config.py:61-71` - reconstructs exceptions unsafely

### Medium Priority

- [ ] Type ignore comments in `utils/llm/dspy_langfuse.py` - indicates type system gaps
- [ ] Limited test coverage for `init/`, `utils/llm/` directories
- [ ] Feature flag not checked in all fallback paths

## Future Improvements

- [ ] Rename `src-tauri/src/config.rs` to `global_config.rs`.
- [ ] Automate Knip: Run `.github/workflows/knip.yml` (create if needed) on PR and push.
- [ ] Automate Link Check: Run `.github/workflows/link_check.yml` (create if needed) weekly.
- [ ] Test Organization: Separate flaky, fast, and nondeterministic tests in the Rust test suite.
