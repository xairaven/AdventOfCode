#!/usr/bin/env sh

FILE="../target/debug/Year2025"
cargo build --bin Year2025
$FILE --day 10 --part 1