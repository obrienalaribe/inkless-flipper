#!/bin/sh
set -exo pipefail

cargo build --release

# rustc inserts two export: `__data_end` and `__heap_base`
# pallet-contracts rejects contracts containing those symbols
# this step is part of cargo-contract
strip-exports\
	./target/wasm32-unknown-unknown/release/inkless_flipper.wasm\
	./target/inkless_flipper.wasm
