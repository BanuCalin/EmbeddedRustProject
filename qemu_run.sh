#!/usr/bin/bash

cargo build --release

qemu-system-gnuarmeclipse \
  -cpu cortex-m4 \
  -machine STM32F4-Discovery \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7em-none-eabi/release/embedded-rust-project