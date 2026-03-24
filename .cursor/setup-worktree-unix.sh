#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ROOT_SOURCE="${ROOT_WORKTREE_PATH:-}"

if [[ -z "${ROOT_SOURCE}" ]]; then
  echo "ROOT_WORKTREE_PATH is not set. This script must run from a Cursor worktree."
  exit 1
fi

require_cmd() {
  local cmd="$1"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "Missing required command: ${cmd}"
    exit 1
  fi
}

copy_optional_file() {
  local rel_path="$1"
  local source_path="${ROOT_SOURCE}/${rel_path}"
  local dest_path="${PROJECT_ROOT}/${rel_path}"

  if [[ -f "${source_path}" ]]; then
    mkdir -p "$(dirname "${dest_path}")"
    cp "${source_path}" "${dest_path}"
    echo "Copied ${rel_path}"
  else
    echo "Skipped ${rel_path} (not present in root worktree)"
  fi
}

echo "=== Tauri Template Cursor worktree setup ==="
echo "Project root: ${PROJECT_ROOT}"
echo "Primary worktree: ${ROOT_SOURCE}"

require_cmd bun
require_cmd cargo
require_cmd make

echo "--- Copying env/config artifacts"
copy_optional_file ".env"

echo "--- Installing Node dependencies"
(cd "${PROJECT_ROOT}" && bun install)

echo "--- Installing docs dependencies"
if [[ -f "${PROJECT_ROOT}/docs/package.json" ]]; then
  (cd "${PROJECT_ROOT}/docs" && bun install)
fi

echo "--- Checking Rust toolchain"
(cd "${PROJECT_ROOT}/src-tauri" && cargo check 2>/dev/null) || echo "Warning: cargo check failed"

echo "Done. Worktree setup complete."
