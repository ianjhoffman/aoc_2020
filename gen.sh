#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Day number required! (Usage: ./gen.sh <1-25>)"
    exit 1
fi

if [ "$1" -lt 1 ] || [ "$1" -gt 25 ]; then
    echo "Day number invalid! (Usage: ./gen.sh <1-25>)"
    exit 1
fi

DIRNAME="aoc_$1"
cargo new "$DIRNAME" --bin --vcs none

# Add common dependencies to Cargo.toml
cat ./templates/cargo.txt >> "$DIRNAME/Cargo.toml"

# Make input directory for input file
mkdir "$DIRNAME/input"

# Overwrite main.rs with template
cat ./templates/main.rs > "$DIRNAME/src/main.rs"
