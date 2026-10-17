# install `cargo nm`

```bash
cargo install cargo-binutils

rustup component add llvm-tools

# 다른 tools
cargo install wasm-nm
```

# Debugging
```bash
rust-nm target/aarch64-unknown-none/debug/b01_arm_qemu_system_arm \
          | grep -E '(_start|rust_main|stack_top)'
0000000040084000 B _stack_top
```
