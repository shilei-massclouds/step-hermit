#!/bin/sh

export XARGO_RUST_SRC=/home/cloud/gitRust/step-hermit/rust/library

cargo xbuild -Zbuild-std=core,alloc,std,panic_abort \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-monk.json

    #--target x86_64-unknown-hermit
