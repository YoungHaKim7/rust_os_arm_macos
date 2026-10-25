.section __TEXT,__text

.global _start

.extern _main

_start:
    bl _main

    // exit(main_return_value)
    // After `bl _main` the return value already sits in x0,
    // which is arg0 of the macOS arm64 syscall ABI.
    mov x16, #0x2000000       // SYSCALL_CLASS_UNIX (2 << 24)
    orr x16, x16, #1          // SYS_exit
    svc 0x80

    // Should never be reached.
    udf #0


.global _macos_syscall

// macOS arm64 (AAPCS64) C ABI inputs:
// x0 = syscall number (C ABI input)
// x1 = arg1
// x2 = arg2
// x3 = arg3
//
// macOS arm64 syscall ABI expects:
// x16 = syscall number (0x2000000 | unix syscall number)
// x0 = arg1
// x1 = arg2
// x2 = arg3
// Return value and negative errno both come back in x0.

_macos_syscall:
    mov x16, x0
    mov x0, x1
    mov x1, x2
    mov x2, x3
    svc 0x80
    ret


.global _my_write

// my_write(fd, buf, len) -> isize
// The C ABI already passes fd/x0, buf/x1, len/x2,
// which matches the write(2) argument registers.
// x16 = 0x2000004 (SYS_write | SYSCALL_CLASS_UNIX).

_my_write:
    mov x16, #0x2000000       // SYSCALL_CLASS_UNIX (2 << 24)
    orr x16, x16, #4          // SYS_write
    svc 0x80
    ret
