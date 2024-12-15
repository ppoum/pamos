ENTRY(_start)

MEMORY {
    LOWER_HALF : ORIGIN = 0x0, LENGTH = 0x800000000000
    HIGHER_HALF : ORIGIN = 0xFFFF800000000000, LENGTH = 0x800000000000 - 1
}

SECTIONS
{
    .bootstrap : {
        *(.bootstrap._start)
        *(.bootstrap)
    } > LOWER_HALF

    .bss ALIGN(0x1000) : {
        *(.bss*)
    } > LOWER_HALF

    .rodata ALIGN(0x1000) : {
        *(.rodata*)
    } > LOWER_HALF

    .ltext ALIGN(0x1000) : AT(ORIGIN(HIGHER_HALF)) {
        *(.ltext.kmain)
        *(.ltext*)
    } > HIGHER_HALF

    .lrodata ALIGN(0x1000) : {
        *(.lrodata*)
    } > HIGHER_HALF
}
