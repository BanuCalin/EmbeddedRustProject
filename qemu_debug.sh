#!/usr/bin/bash

cargo build

qemu-system-gnuarmeclipse \
  -cpu cortex-m4 \
  -machine STM32F4-Discovery \
  -nographic \
  -gdb tcp::3333 \
  -S \
  -verbose \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7em-none-eabi/debug/embedded-rust-project