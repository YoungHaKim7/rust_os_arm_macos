# 나만의 Syscall만들기

### 261015)C without lib C - nostdlib-C (X86_64) - Part 1
- Swindlers Inc
```txt
0:00 G'day
1:02 Simple C project
4:12 No stdlib
6:05 Our own _start
8:22 Our own write function
12:11 Syscall vs C function call
13:03 Our own syscall
15:22 compiler_flags.txt
```
- https://codeberg.org/Swindlers-Inc/hodgepodge
- https://youtu.be/U9nHEA6aOmI?si=15O5-posc6k_ijU3

# x86 Assembly/Interfacing with Linux
- https://en.wikibooks.org/wiki/X86_Assembly/Interfacing_with_Linux

# Linux Kernel ABI docs
- https://docs.kernel.org/admin-guide/abi-stable.html

# x86-64 VS ARM64 macOS
|  | x86-64 Linux (original) | ARM64 macOS (new) |
| --- | --- | --- |
| trap instruction | syscall | svc 0x80 |
| syscall number register | rax | x16 |
| args / return | rdi, rsi, rdx / rax | x0, x1, x2 / x0 |
| write number | 1 | 0x2000004 (4 + UNIX class 0x2000000) |
| exit number | 60 | 0x2000001 |
