#!/bin/bash
SCRIPT_DIR="$(dirname ${0})"
. "${SCRIPT_DIR}"/vars
cargo "${TOOLCHAIN}" test -- --show-output 2>/dev/null
