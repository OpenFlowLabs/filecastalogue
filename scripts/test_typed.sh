#!/bin/bash
export RUST_BACKTRACE=1
cargo build && ./target/debug/mess_typed_json
