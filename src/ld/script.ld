OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY {
    kram (wxa) : ORIGIN = 0x80200000, LENGTH = 128 * 1024 * 1024
}

SECTIONS {
    .text.boot : {
        *(.text.boot)
    } > kram
    .text : {
        *(.text)
        *(.text.*)
    } > kram
    .data : {
        *(.data)
    } > kram
    .rodata : {
        *(.rodata)
        *(.rodata.*)
    } > kram
    .bss : {
        *(.bss)
    } > kram
    . = ALIGN(8);
    . = . + 4 * 1024;
    _STACK_PTR = .;
}
