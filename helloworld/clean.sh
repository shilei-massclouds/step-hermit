cargo clean -Zbuild-std=core,alloc,std,panic_abort \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-monk.json

rm -rf ./rusty-loader/target

rm -rf ../libhermit-rs/x86_64* ../libhermit-rs/debug
