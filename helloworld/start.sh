#!/bin/sh

qemu-system-x86_64 -smp 1 -display none -m 1G -serial stdio \
    -cpu qemu64,apic,fsgsbase,rdtscp,xsave,xsaveopt,fxsr \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -kernel rusty-loader/target/x86_64/debug/rusty-loader \
    -initrd target/x86_64-unknown-hermit/debug/helloworld \
    -smp 1
