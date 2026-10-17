```bash
$ just qemu

 cargo build --target aarch64-unknown-none
 qemu-system-aarch64 -machine virt -cpu cortex-a57 -m 256M -nographic -kernel target/aarch64-unknown-none/debug/nographic_final_hello

Hello World!
QEMU: Terminated
```


