#!/bin/bash

scarb build
cargo build --bin bindgen
bash scripts/rust_fmt.sh --fix
