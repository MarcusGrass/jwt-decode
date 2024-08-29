#/bin/sh
set -e
cargo fmt --all
cargo clippy -p decode-lib
cargo clippy -p decode-lib-fuzz
cargo clippy -p jwt-decode
cargo clippy -p tester
