#!/bin/sh

WORK_DIR=/home/cloud/gitRust/step-hermit/libhermit-rs
BUILD_ARCHIVE=$WORK_DIR/x86_64-unknown-none/debug/libhermit.a
DIST_ARCHIVE=$WORK_DIR/x86_64/debug/libhermit.a

cargo build --target=x86_64-unknown-none --target-dir /home/cloud/gitRust/step-hermit/libhermit-rs \
    --no-default-features --features "acpi fsgsbase pci pci-ids smp" --profile dev

mkdir -p $WORK_DIR/x86_64/debug/
cp $BUILD_ARCHIVE $DIST_ARCHIVE

