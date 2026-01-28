# ANSI color codes
GREEN=\033[0;32m
YELLOW=\033[0;33m
RED=\033[0;31m
BLUE=\033[0;34m
RESET=\033[0m

PROJECT_ROOT=.

.DEFAULT_GOAL := help

########################################################
# Help
########################################################

### Help
.PHONY: help
help: ## Show this help message
	@echo "$(BLUE)Available Make Targets$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "; category=""} \
		/^### / {category = substr($0, 5); next} \
		/^[a-zA-Z_-]+:.*?## / { \
			if (category != last_category) { \
				if (last_category != "") print ""; \
				print "$(GREEN)" category ":$(RESET)"; \
				last_category = category; \
			} \
			printf "  $(YELLOW)%-23s$(RESET) %s\n", $1, $2 \
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
# Run Tests
########################################################

### Testing
test: ## Run Rust tests
	@echo "$(GREEN)🧪Running Rust Tests...$(RESET)"
	cd src-tauri && cargo test
	@echo "$(GREEN)✅Rust Tests Passed.$(RESET)"

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
		echo "$(YELLOW)⚠️ lychee not installed. Falling back to docs lint script...$(RESET)"; \
		cd docs && bun run lint:links; \
	fi
	@echo "$(GREEN)✅ Link check completed.$(RESET)"

ci: fmt lint knip audit link-check test ## Run all CI checks
	@echo "$(GREEN)✅ CI checks completed.$(RESET)"
