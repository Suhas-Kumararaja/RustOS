    .global _start

    .section .rodata

debug_string:
    .string "Hiiiii Qemuuuuu\n"

    .section .text

_start:
    li a7, 0x4442434e
    li a6, 0x0
    li a0, 16
    lla a1, debug_string
    li a2, 0
    ecall

loop:
    j loop
