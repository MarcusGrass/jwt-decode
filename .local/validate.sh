#!/bin/sh
set -e
cargo fmt --all --check
cargo clippy -p decode-lib -- -D warnings
cargo clippy -p decode-lib-fuzz -- -D warnings
cargo clippy -p jwt-decode -- -D warnings
cargo clippy -p tester -- -D warnings

./.local/test.sh

cargo deny check