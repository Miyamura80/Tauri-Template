# ANSI color codes
GREEN=\033[0;32m
YELLOW=\033[0;33m
RED=\033[0;31m
BLUE=\033[0;34m
RESET=\033[0m

PYTHON=uv run
TEST=uv run pytest
PROJECT_ROOT=.

.DEFAULT_GOAL := help

########################################################
# Help
########################################################

### Help
.PHONY: help docs
help: ## Show this help message
	@echo "$(BLUE)Available Make Targets$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "; category=""} \
		/^### / {category = substr($$0, 5); next} \
		/^[a-zA-Z_-]+:.*?## / { \
			if (category != last_category) { \
				if (last_category != "") print ""; \
				print "$(GREEN)" category ":$(RESET)"; \
				last_category = category; \
			} \
			printf "  $(YELLOW)%-23s$(RESET) %s\n", $$1, $$2 \
		}' $(MAKEFILE_LIST)

########################################################
# Tauri / Frontend
########################################################

### Tauri
.PHONY: dev build tauri-dev tauri-build

dev: ## Run the frontend in development mode
	bun run dev

build: ## Build the frontend
	bun run build

tauri-dev: ## Run the app in Tauri development mode
	bun run tauri dev

tauri-build: ## Build the Tauri application
	bun run tauri build

########################################################
# Initialization
########################################################

### Initialization
.PHONY: init banner logo
init: ## Initialize project (usage: make init name=my-project description="my description")
	@if [ -z "$(name)" ] || [ -z "$(description)" ]; then \
		echo "$(RED)Error: Both 'name' and 'description' parameters are required$(RESET)"; \
		echo "Usage: make init name=<project_name> description=<project_description>"; \
		exit 1; \
	fi
	@echo "$(YELLOW)🚀 Initializing project $(name)...$(RESET)"
	@sed -i.bak "s/name = \"python-template\"/name = \"$(name)\"/" pyproject.toml && rm pyproject.toml.bak
	@sed -i.bak "s/description = \"Add your description here\"/description = \"$(description)\"/" pyproject.toml && rm pyproject.toml.bak
	@sed -i.bak "s/# Python-Template/# $(name)/" README.md && rm README.md.bak
	@sed -i.bak "s/<b>Opinionated Python project stack. 🔋 Batteries included. <\/b>/<b>$(description)<\/b>/" README.md && rm README.md.bak
	@echo "$(GREEN)✅ Updated project name and description.$(RESET)"

banner: check_uv ## Generate project banner image
	@echo "$(YELLOW)🔍Generating banner...$(RESET)"
	@uv run python -m init.generate_banner
	@echo "$(GREEN)✅Banner generated.$(RESET)"

logo: check_uv ## Generate logo and favicon for docs
	@echo "$(YELLOW)🔍Generating logo and favicon...$(RESET)"
	@uv run python -m init.generate_logo
	@echo "$(GREEN)✅Logo and favicon generated in docs/public/$(RESET)"


########################################################
# Check dependencies
########################################################

check_uv:
	@echo "$(YELLOW)🔍Checking uv version...$(RESET)"
	@if ! command -v uv > /dev/null 2>&1; then \
		echo "$(RED)uv is not installed. Please install uv before proceeding.$(RESET)"; \
		exit 1; \
	else \
		uv --version; \
	fi

check_jq:
	@echo "$(YELLOW)🔍Checking jq version...$(RESET)"
	@if ! command -v jq > /dev/null 2>&1; then \
		echo "$(RED)jq is not installed. Please install jq before proceeding.$(RESET)"; \
		echo "$(RED)brew install jq$(RESET)"; \
		exit 1; \
	else \
		jq --version; \
	fi

########################################################
# Setup githooks for linting
########################################################
setup_githooks:
	@echo "$(YELLOW)🔨Setting up githooks on post-commit...$(RESET)"
	chmod +x .githooks/post-commit
	git config core.hooksPath .githooks


########################################################
# Python dependency-related
########################################################

### Setup & Dependencies
setup: check_uv ## Create venv and sync dependencies
	@echo "$(YELLOW)🔎Looking for .venv...$(RESET)"
	@if [ ! -d ".venv" ]; then \
		echo "$(YELLOW)VS Code is not detected. Creating a new one...$(RESET)"; \
		uv venv; \
	else \
		echo "$(GREEN)✅.venv is detected.$(RESET)"; \
	fi
	@echo "$(YELLOW)🔄Updating python dependencies...$(RESET)"
	@uv sync

view_python_venv_size:
	@echo "$(YELLOW)🔍Checking python venv size...$(RESET)"
	@PYTHON_VERSION=$$(cat .python-version | cut -d. -f1,2) && \
	cd .venv/lib/python$$PYTHON_VERSION/site-packages && du -sh . && cd ../../../
	@echo "$(GREEN)Python venv size check completed.$(RESET)"

view_python_venv_size_by_libraries:
	@echo "$(YELLOW)🔍Checking python venv size by libraries...$(RESET)"
	@PYTHON_VERSION=$$(cat .python-version | cut -d. -f1,2) && \
	cd .venv/lib/python$$PYTHON_VERSION/site-packages && du -sh * | sort -h && cd ../../../
	@echo "$(GREEN)Python venv size by libraries check completed.$(RESET)"

########################################################
# Run Main Application
########################################################

### Running
all: setup setup_githooks ## Setup and run main application
	@echo "$(GREEN)🏁Running main application...$(RESET)"
	@$(PYTHON) main.py
	@echo "$(GREEN)✅ Main application run completed.$(RESET)"

docs: ## Run docs with bun
	@echo "$(GREEN)📚Running docs...$(RESET)"
	@cd docs && bun run dev
	@echo "$(GREEN)✅ Docs run completed.$(RESET)"

ralph: check_jq ## Run Ralph agent loop
	@echo "$(RED)⚠️  WARNING: Ralph is an autonomous agent that can modify your codebase.$(RESET)"
	@echo "$(RED)⚠️  It is HIGHLY RECOMMENDED to run Ralph in a sandboxed environment.$(RESET)"
	@printf "$(YELLOW)Are you sure you want to continue? [y/N] $(RESET)" && read ans && [ "$$ans" = "y" ] || (echo "$(RED)Aborted.$(RESET)"; exit 1)
	@echo "$(GREEN)🤖 Starting Ralph Agent...$(RESET)"
	@chmod +x scripts/ralph.sh
	@./scripts/ralph.sh $(ARGS)
	@echo "$(GREEN)✅ Ralph Agent finished.$(RESET)"


########################################################
# Run Tests
########################################################

TEST_TARGETS = tests/

### Testing
test: check_uv ## Run all pytest tests
	@echo "$(GREEN)🧪Running Target Tests...$(RESET)"
	$(TEST) $(TEST_TARGETS)
	@echo "$(GREEN)✅Target Tests Passed.$(RESET)"

test_fast: check_uv ## Run fast tests (exclude slow/nondeterministic)
	@echo "$(GREEN)🧪Running Fast Tests...$(RESET)"
	$(TEST) -m "not slow and not nondeterministic" $(TEST_TARGETS)
	@echo "$(GREEN)✅Fast Tests Passed.$(RESET)"

test_slow: check_uv ## Run slow tests only
	@echo "$(GREEN)🧪Running Slow Tests...$(RESET)"
	@$(TEST) -m "slow" $(TEST_TARGETS); \
	status=$$?; \
	if [ $$status -eq 5 ]; then \
		echo "$(YELLOW)⚠️ No slow tests collected.$(RESET)"; \
		exit 0; \
	fi; \
	exit $$status

test_nondeterministic: check_uv ## Run nondeterministic tests only
	@echo "$(GREEN)🧪Running Nondeterministic Tests...$(RESET)"
	@$(TEST) -m "nondeterministic" $(TEST_TARGETS); \
	status=$$?; \
	if [ $$status -eq 5 ]; then \
		echo "$(YELLOW)⚠️ No nondeterministic tests collected.$(RESET)"; \
		exit 0; \
	fi; \
	exit $$status

test_flaky: check_uv ## Repeat fast tests to detect flaky tests
	@echo "$(GREEN)🧪Running Flaky Test Detection...$(RESET)"
	$(TEST) --count 2 -m "not slow and not nondeterministic" $(TEST_TARGETS)
	@echo "$(GREEN)✅Flaky Test Detection Passed.$(RESET)"


########################################################
# Code Quality
########################################################

### Code Quality
.PHONY: fmt lint knip audit link-check ci

fmt: ## Format code with Biome and rustfmt
	@echo "$(YELLOW)✨ Formatting and linting with Biome...$(RESET)"
	bunx @biomejs/biome check --write --unsafe .
	@echo "$(YELLOW)✨ Formatting Rust code...$(RESET)"
	cd src-tauri && cargo fmt
	@echo "$(GREEN)✅ Formatting completed.$(RESET)"

lint: ## Lint code with Biome and Clippy
	@echo "$(YELLOW)🔍 Checking with Biome...$(RESET)"
	bunx @biomejs/biome check .
	@echo "$(YELLOW)🔍 Linting Rust code with Clippy...$(RESET)"
	cd src-tauri && cargo clippy -- -D warnings
	@echo "$(GREEN)✅ Linting completed.$(RESET)"

knip: ## Find unused files, dependencies, and exports
	@echo "$(YELLOW)🔍 Running Knip...$(RESET)"
	bunx knip
	@echo "$(GREEN)✅ Knip completed.$(RESET)"

audit: ## Audit dependencies for vulnerabilities
	@echo "$(YELLOW)🔍 Auditing frontend dependencies...$(RESET)"
	bun audit
	@echo "$(YELLOW)🔍 Auditing Rust dependencies...$(RESET)"
	@if command -v cargo-deny > /dev/null 2>&1; then \
		cd src-tauri && cargo deny check; \
	else \
		echo "$(YELLOW)⚠️ cargo-deny not installed. Skipping Rust audit.$(RESET)"; \
	fi
	@echo "$(GREEN)✅ Audit completed.$(RESET)"

link-check: ## Check for broken links in markdown files
	@echo "$(YELLOW)🔍 Checking links...$(RESET)"
	@if command -v lychee > /dev/null 2>&1; then \
		lychee .; \
	else \
		echo "$(YELLOW)⚠️ lychee not installed. Falling back to markdown-link-check...$(RESET)"; \
		bunx markdown-link-check README.md; \
	fi
	@echo "$(GREEN)✅ Link check completed.$(RESET)"

ci: fmt lint knip audit link-check ## Run all CI checks
	@echo "$(GREEN)✅ CI checks completed.$(RESET)"

########################################################
# Dependencies
########################################################

requirements:
	@echo "$(YELLOW)🔍Checking requirements...$(RESET)"
	@cp requirements-dev.lock requirements.txt
	@echo "$(GREEN)✅Requirements checked.$(RESET)"
