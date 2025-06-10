#!/bin/bash

scarb build
cargo +nightly-2024-08-28 run --bin bindgen
bash scripts/rust_fmt.sh --fix
