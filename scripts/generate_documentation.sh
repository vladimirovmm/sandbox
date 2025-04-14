#!/bin/bash

for path in $(find ./ -name Cargo.toml -type f); do  
    dir=`dirname ${path}`
    cargo doc --manifest-path $path --open --target-dir $dir
done; 
