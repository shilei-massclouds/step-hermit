#!/bin/sh

cd ../libhermit-rs && ./build.sh && cd -

cargo rustc -Zbuild-std=core,alloc,std,panic_abort \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-monk.json \
    -- -L native=/home/cloud/gitRust/step-hermit/libhermit-rs/x86_64/debug -l static=hermit

cd rusty-loader && ./build.sh && cd -
