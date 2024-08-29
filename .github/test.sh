#!/bin/sh
set -ex

RUSTFLAGS='-C panic=abort -C link-arg=-nostartfiles -C target-feature=+crt-static -C relocation-model=pie' cargo b -p jwt-decode --target x86_64-unknown-linux-gnu --profile lto
cargo r -r -p tester -- ./target/x86_64-unknown-linux-gnu/lto/jwt-decode