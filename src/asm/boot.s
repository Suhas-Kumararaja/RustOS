    .extern _STACK_PTR
    .global _start

    .section .text.boot

_start:
    la sp, _STACK_PTR
    jal main

loop:
    j loop
