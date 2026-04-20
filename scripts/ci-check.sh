#!/usr/bin/env bash
set -euo pipefail

echo "=== Rust: format check ==="
cargo fmt --check

echo "=== Rust: clippy ==="
cargo clippy --all-targets --all-features -- -D warnings

echo "=== Rust: tests ==="
cargo test

echo "=== Web: build ==="
if [ -d "web/node_modules" ]; then
  (cd web && npx next build)
else
  echo "SKIP: web/node_modules not found -- run 'cd web && npm install' first"
fi

echo ""
echo "=== All checks passed ==="
