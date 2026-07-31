#!/usr/bin/env bash
set -euo pipefail
cargo fmt --check
cargo clippy -- -D warnings
arch audit --rs -r main/config/arch/architecture.policy.toml
