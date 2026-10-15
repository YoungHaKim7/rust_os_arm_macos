qemu:
    cargo build --target aarch64-unknown-none
    qemu-system-aarch64 -machine virt -cpu cortex-a53 -kernel target/aarch64-unknown-none/debug/rust_os_arm_macos

    
