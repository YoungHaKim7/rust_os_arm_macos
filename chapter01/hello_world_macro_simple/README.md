# Result


```bash
$ just qemu

cargo build --target aarch64-unknown-none

qemu-system-aarch64 -machine virt -cpu cortex-a57 -m 256M -nographic -kernel target/aarch64-unknown-none/debug/hello_world_macro_simple
Hello, world!
Rust AArch64 kernel started.
Decimal: 42, hexadecimal: 0x2a
This is a test.
QEMU: Terminated

```

