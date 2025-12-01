#!/usr/bin/env sh

FILE="../target/debug/Year2025"
cargo build --bin Year2025
$FILE --day 1 --part 1