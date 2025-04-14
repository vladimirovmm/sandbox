#!/bin/bash

for path in $(find ./ -name Cargo.toml -type f); do  
	cargo clippy --manifest-path $path --no-deps --all-targets -- -Dwarnings  || exit 1
	cargo +nightly fmt --manifest-path $path --check  || exit 2
	cargo test  --manifest-path $path  || exit 3
done; 
