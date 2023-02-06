#!/bin/bash
SCRIPT_DIR="$(dirname ${0})"
. "${SCRIPT_DIR}"/vars
cargo "${TOOLCHAIN}" test --features os_string_paths -- --show-output 2>/dev/null
