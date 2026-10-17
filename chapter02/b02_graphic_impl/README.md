# Result


```bash
just qemu
cargo build --target aarch64-unknown-none

qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 256M -device ramfb -kernel target/aarch64-unknown-none/debug/b02_graphic_impl -display cocoa -serial stdio -monitor none

Rust AArch64 kernel started
Drawing framebuffer pixels...
Framebuffer pixels drawn
[ramfb] Searching fw_cfg directory...
[fw_cfg] Selecting signature...
[fw_cfg] Reading signature...
[fw_cfg] Signature: QEMU
[fw_cfg] Selecting file directory...
[fw_cfg] Directory entries: 0x0000000A
[fw_cfg] Entry 00000000
[fw_cfg] Entry 00000001
[fw_cfg] Entry 00000002
[fw_cfg] Entry 00000003
[fw_cfg] Entry 00000004
[fw_cfg] Entry 00000005
[fw_cfg] Found etc/ramfb
[ramfb] Found etc/ramfb selector: 0x00000025
[ramfb] Framebuffer address: 0x0000000040086000
[ramfb] Preparing configuration...
[ramfb] Starting DMA write...
[ramfb] DMA status: 0x00000001
[ramfb] ERROR: DMA reported an error

```

