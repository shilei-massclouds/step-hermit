#!/bin/sh

cargo build -Zbuild-std=core,alloc,std,panic_abort \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-monk.json

cd rusty-loader && ./build.sh && cd -
