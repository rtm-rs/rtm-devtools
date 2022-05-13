#!/usr/bin/env bash
set -e

find . -type f -name '*.rs' -print0 | xargs --null redo-ifchange

cargo nextest run --no-fail-fast --lib &>"$3"
cargo test --doc &>>"$3"
