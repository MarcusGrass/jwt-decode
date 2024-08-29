#!/bin/sh
set -ex

cargo clippy -p decode-lib -- -D warnings
cargo clippy -p decode-lib-fuzz -- -D warnings
cargo clippy -p jwt-decode -- -D warnings
cargo clippy -p tester -- -D warnings