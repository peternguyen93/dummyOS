#!/bin/bash

cargo build --release
rust-objcopy --strip-all -O binary ./target/aarch64-unknown-none-softfloat/release/dummyOS kernel.bin
qemu-system-aarch64 -M raspi3b -serial stdio -display none -kernel kernel.bin
