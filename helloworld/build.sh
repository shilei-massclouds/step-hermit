#!/bin/sh

cd ../libhermit-rs && ./build.sh && cd -

cargo build -Zbuild-std=core,alloc,std,panic_abort \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-monk.json

cd rusty-loader && ./build.sh && cd -
