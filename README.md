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
# ㅇㅣ게 된다
$ qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos

# 이건 화면이 거의  멈춰 있는데
qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos -nographic
```

