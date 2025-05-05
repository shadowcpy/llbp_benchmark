#!/usr/bin/env bash

set -euxo pipefail

mkdir output || true

BINARY_SEARCH=$(cargo bench 2>&1 >/dev/null | grep -E -o "(target/release/deps/binary_search[^)]*)")
cp "$BINARY_SEARCH" output/binary_search
