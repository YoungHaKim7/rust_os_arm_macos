# a01 — My Own Syscall (Rust, native ARM64 macOS)

A `#![no_std]` / `#![no_main]` Rust binary with **no libc**: our own `_start`,
our own `write`, and our own raw syscall wrappers in ARM64 assembly
([asm/start.s](asm/start.s)), running natively on Apple Silicon with plain
`cargo run`.

Originally written for x86-64 Linux (Swindlers Inc, "C without libc"), ported
to macOS following the Darwin syscall conventions.

## Run

```bash
cargo run
```

## Result

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/a01_own_syscall_arm_macos`
Hello world!
$ echo $?
0
```

`main` returning 42 comes back as exit status 42, proving the raw `exit`
syscall (`x0` = return value) works end to end.

## x86-64 Linux → ARM64 macOS conversion

|                         | x86-64 Linux (original)   | ARM64 macOS (this port)              |
| ----------------------- | ------------------------- | ------------------------------------ |
| trap instruction        | `syscall`                 | `svc 0x80`                           |
| syscall number register | `rax`                     | `x16`                                |
| arg1 / arg2 / arg3      | `rdi` / `rsi` / `rdx`     | `x0` / `x1` / `x2`                   |
| return value            | `rax`                     | `x0` (errors: negative errno in `x0`)|
| `write` syscall number  | 1                         | 4 + 0x2000000 = `0x2000004`          |
| `exit` syscall number   | 60                        | 1 + 0x2000000 = `0x2000001`          |

Notes:

- `0x2000000` is `SYSCALL_CLASS_UNIX << 24`; every BSD/unix syscall number on
  64-bit macOS is OR-ed with it (`syscalls.master` in XNU).
- AArch64 `mov` cannot load arbitrary immediates, so `0x2000001` is built as
  `mov x16, #0x2000000` + `orr x16, x16, #1`.
- Darwin assembly has no ELF `.type` directive, and C symbols carry a leading
  underscore: the asm defines `_start`, `_main`, `_my_write`, `_macos_syscall`.

## Linking notes (`.cargo/config.toml`)

- `-nostartfiles` + `-Wl,-e,_start`: skip crt (which would call libc
  `main`/`exit`); our `_start` in asm/start.s is the real entry point.
- `-lSystem`: `ld` refuses to link a dynamic executable without libSystem, so
  it is linked and mapped by dyld — but the program never calls into it; every
  syscall goes through our own `svc 0x80` wrappers. (A truly `-static` build
  links fine but the kernel kills it on Apple Silicon: non-PIE/static-PIE
  executables get SIGKILL.)
- `panic = "abort"` in `Cargo.toml`: a no_std binary cannot use the host
  `core`'s unwinding.
- `_macos_syscall` (the generic 4-arg syscall wrapper, equivalent of the
  tutorial's `linux_syscall`) is dead-stripped by the linker until you
  uncomment the example at the bottom of `src/main.rs`.
