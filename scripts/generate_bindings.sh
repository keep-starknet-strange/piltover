#!/bin/bash

scarb build
cargo +stable run --bin bindgen
bash scripts/rust_fmt.sh --fix
