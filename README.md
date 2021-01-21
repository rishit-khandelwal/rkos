# R.O.S: Rust Operating System

An OS written in rust for fun.

This OS is based on https://os.phil-opp.com/

### Setup

- Install qemu and Rust
- `rustup override set nightly`
  (Just incase if a nightly feature is used).
- `cargo install bootimage` to install bootimage, we use it create a bootable image.
- `cargo r` to run the OS in qemu.
- You might also need to install:
- - `llvm-tools-preview`
- - Install this by `rustup component add llvm-tools-preview`
