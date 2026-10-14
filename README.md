#  Install


```bash
$ brew install arm-none-eabi-gcc qemu
```

# build


```bash
$ cargo rustc -- -C link-arg=--script=linker.ld

# 이렇게 해도 된다.
$ cargo build
```

# qemu

```bash
qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos -nographic
```

