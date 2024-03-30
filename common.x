ENTRY(reset_handler);

EXTERN(RESET_VECTOR);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        // Stack pointer
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        // Reset vector
        KEEP(*(.vector_table.reset_vector));
    } > FLASH

    .text :
    {
        *(.text .text.*);
    } > FLASH

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*);
    } > FLASH

    .bss : ALIGN(4)
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = ALIGN(4);
    } > RAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata)) ALIGN(4)
    {
        _sdata = .;
        *(.data .data.*);
        _edata = ALIGN(4);
    } > RAM

    _sidata = LOADADDR(.data);

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
    }
}
