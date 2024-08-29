#/bin/sh
set -e
cargo +nightly fuzz run fuzz_valid -- -max_total_time=15
cargo +nightly fuzz run fuzz_invalid -- -max_total_time=15
