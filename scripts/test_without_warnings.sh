#!/bin/bash
cargo test --features os_string_paths 2>/dev/null "${@}"
