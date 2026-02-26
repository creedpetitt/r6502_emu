#!/bin/bash
set -e

if [ ! -f "6502_functional_test.bin" ]; then
    echo "Downloading Klaus Dormann 6502 Functional Test..."
    curl -sO https://raw.githubusercontent.com/Klaus2m5/6502_65C02_functional_tests/master/bin_files/6502_functional_test.bin
fi

echo "Running Klaus test..."
cargo run --release --quiet -- 6502_functional_test.bin
