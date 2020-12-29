# rust-arm-template

## Introduction

This is a quick template to perform cross compîlation with ARM using AArch64 ISA and Rust.

## Pre-requisite

1) It uses the 'nightly' version of Rust to cross compile to Aarch64

2) This template needs to have 'cargo xbuild' installed for easy cross compilation
To install:
```
cargo install xbuild
```

## Usage

To build your project for Aarch64:

```
cargo xbuild --target=aarch64-unknown-none.json --features raspi3
```

#Run in QEMU

```
qemu-system-aarch64 -M raspi3 -kernel target/aarch64-unknown-none/release/r03_knock-knock-interrupt -serial null -serial stdio
```

