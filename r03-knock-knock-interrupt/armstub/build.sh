#!/bin/bash

mkdir -p build
aarch64-none-elf-as -o build/armstub.o src/armstub.S
aarch64-none-elf-ld --section-start=.text=0 -o build/armstub.elf build/armstub.o
aarch64-none-elf-objcopy -O binary build/armstub.elf build/armstub.bin