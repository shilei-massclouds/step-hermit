#!/bin/sh

WORK_DIR=/home/cloud/gitRust/step-hermit/libhermit-rs
BUILD_ARCHIVE=$WORK_DIR/x86_64-unknown-none/debug/libhermit.a
DIST_ARCHIVE=$WORK_DIR/x86_64/debug/libhermit.a
LLVM_AR=`rustc --print sysroot`/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-ar

cargo build --target=x86_64-unknown-none --target-dir $WORK_DIR \
    --no-default-features --features "acpi fsgsbase pci pci-ids smp" --profile dev

mkdir -p $WORK_DIR/x86_64/debug/
cp $BUILD_ARCHIVE $DIST_ARCHIVE

cp $WORK_DIR/libhermit.redefine-syms.template $WORK_DIR/x86_64/debug/libhermit.redefine-syms
nm --defined-only --print-file-name $DIST_ARCHIVE 2>/dev/null | \
    grep "^$DIST_ARCHIVE:hermit-" | cut -d ' ' -f 3 | \
    grep "^sys_" | xargs -Isymbol echo 'libhermit_symbol symbol' \
    >> $WORK_DIR/x86_64/debug/libhermit.redefine-syms

objcopy --prefix-symbols=libhermit_ $DIST_ARCHIVE
objcopy --redefine-syms=$WORK_DIR/x86_64/debug/libhermit.redefine-syms $DIST_ARCHIVE

cargo build --manifest-path=hermit-builtins/Cargo.toml \
    --target=../helloworld/x86_64-unknown-monk.json \
    -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem \
    --target-dir $WORK_DIR --profile dev

cat $WORK_DIR/hermit-builtins/exports | xargs -Isymbol echo 'libhermit_builtins_symbol symbol' \
    >> $WORK_DIR/x86_64-unknown-monk/debug/libhermit_builtins.redefine-syms

objcopy --prefix-symbols=libhermit_builtins_ $WORK_DIR/x86_64-unknown-monk/debug/libhermit_builtins.a
objcopy --redefine-syms=$WORK_DIR/x86_64-unknown-monk/debug/libhermit_builtins.redefine-syms $WORK_DIR/x86_64-unknown-monk/debug/libhermit_builtins.a

echo "AR: $LLVM_AR ..."
$LLVM_AR qL $DIST_ARCHIVE $WORK_DIR/x86_64-unknown-monk/debug/libhermit_builtins.a
echo "AR final builtins: OK!"
