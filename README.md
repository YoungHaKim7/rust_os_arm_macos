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

# quem run & exit

- To exit QEMU, press `Ctrl-A`, then `X`.

```bash
$ just qemu

qemu-system-aarch64 \
          -machine virt \
          -cpu cortex-a57 \
          -m 256M \
          -nographic \
          -kernel target/aarch64-unknown-none/debug/a02_nographic_hello_wolrd_os

Hello World!
QEMU: Terminated
```


# qemu

```bash
# ㅇㅣ게 된다
$ qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos

# 이건 화면이 거의  멈춰 있는데
qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos -nographic
```


- nographic

```bash
$ cargo build

$ qemu-system-aarch64 \
          -machine virt \
          -cpu cortex-a57 \
          -m 256M \
          -nographic \
          -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos
```
