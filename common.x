ENTRY(reset_handler);

EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);
EXTERN(INTERRUPTS);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        // Stack pointer
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        // Reset vector
        KEEP(*(.vector_table.reset_vector));

        // Exception handlers
        KEEP(*(.vector_table.exceptions));

        // Non-core vectors
        KEEP(*(.vector_table.interrupts));
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

PROVIDE(nmi_handler = default_exception_handler);
PROVIDE(hard_fault_handler = default_exception_handler);
PROVIDE(memory_management_handler = default_exception_handler);
PROVIDE(bus_fault_handler = default_exception_handler);
PROVIDE(usage_fault_handler = default_exception_handler);
PROVIDE(sv_call_handler = default_exception_handler);
PROVIDE(pending_sv_handler = default_exception_handler);
PROVIDE(systick_handler = default_exception_handler);