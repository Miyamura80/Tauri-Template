# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tauri template for desktop application development with React and TypeScript. Migrating from a Python-based template.

## Common Commands

```bash
# Tauri / Frontend
bun install          # Install dependencies
bun run tauri dev    # Run the app in development mode
bun run build        # Build the frontend
bun run tauri build  # Build the Tauri application

# Legacy Python Commands
make setup           # Create/update .venv and sync dependencies
make test            # Run pytest on tests/
```

## Architecture

- **src/** - Tauri frontend (React + TypeScript + Vite)
- **src-tauri/** - Tauri backend (Rust)
- **src_python/** - Legacy Python source code
- **python_python_common/** - Legacy Python global configuration
- **python_utils/** - Legacy Python utilities
- **tests/** - pytest tests (legacy)
- **init/** - Initialization scripts (legacy)
- **docs/** - Documentation (Next.js app)

## Code Style

- snake_case for functions/files/directories
- CamelCase for classes
- UPPERCASE for constants
- 4-space indentation, double quotes
- Use built-in types (list, dict, tuple) not typing.List/Dict/Tuple

## Configuration Pattern

```python
from python_common import global_config

# Access config values
global_config.example_parent.example_child
global_config.llm_config.default_model

# Access secrets from .env
global_config.OPENAI_API_KEY
```

## LLM Inference Pattern

```python
from python_utils.llm.dspy_inference import DSPYInference
import dspy

class MySignature(dspy.Signature):
    input_field: str = dspy.InputField()
    output_field: str = dspy.OutputField()

inf_module = DSPYInference(pred_signature=MySignature, observe=True)
result = await inf_module.run(input_field="value")
```

## Testing Pattern

```python
from tests.test_template import TestTemplate
from tests.conftest import slow_test, nondeterministic_test

class TestMyFeature(TestTemplate):
    def test_something(self):
        assert self.config is not None

    @slow_test
    def test_slow_operation(self):
        pass
```

## Logging

```python
from loguru import logger as log
from src_python.utils.logging_config import setup_logging

setup_logging()
log.debug("detailed diagnostic information")
log.info("general informational message")
log.warning("warning message for potentially harmful situations")
log.error("error message for error events")
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
- **Protected Branch**: `main` is protected. Do not push directly to `main`. Use PRs.
- **Merge Strategy**: Squash and merge.

## Deprecated

- Don't use `datetime.utcnow()` - use `datetime.now(timezone.utc)`
