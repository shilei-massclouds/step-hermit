#
# Make system for helloworld
#

all:
	printf "in helloworld\n"
	cargo rustc -Zbuild-std=core,alloc,std,panic_abort \
		-Zbuild-std-features=compiler-builtins-mem \
		--target x86_64-unknown-monk.json \
		-- -L native=/home/cloud/gitRust/step-hermit/libhermit-rs/x86_64/debug -l static=hermit
	make -C rusty-loader

justrun:
	qemu-system-x86_64 -smp 1 -display none -m 1G -serial stdio \
		-cpu qemu64,apic,fsgsbase,rdtscp,xsave,xsaveopt,fxsr \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-kernel rusty-loader/target/x86_64/debug/rusty-loader \
		-initrd target/x86_64-unknown-monk/debug/helloworld \
		-smp 1

clean:
	rm -rf ./target
	rm -rf ./rusty-loader/target
	rm -rf ../libhermit-rs/x86_64* ../libhermit-rs/debug

.PHONY: all build clean run justrun FORCE
