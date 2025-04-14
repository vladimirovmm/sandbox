#!/bin/bash

for path in $(find ./ -name Cargo.toml -type f); do  
    cargo fix --manifest-path $path  --allow-dirty --allow-staged || exit 1
	cargo clippy --fix --manifest-path $path  --no-deps --allow-dirty --allow-staged || exit 2
	cargo +nightly fmt --manifest-path $path || exit 3
done; 
