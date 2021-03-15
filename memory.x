/* Linker script for the STM32F103C8T6 */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 64K
    RAM   : ORIGIN = 0x20000000, LENGTH = 20K
}

EXTERN(__RESET_VECTOR)

/* # Sections */
SECTIONS
{
    PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

    /* ## Sections in FLASH */
    /* ### Vector table */
    .vector_table ORIGIN(FLASH) :
    {
        /* Initial Stack Pointer (SP) value */
        LONG(_stack_start);

        /* Reset Vector */
        KEEP(*(.vector_table.reset_vector)); /* this is the `__RESET_VECTOR` symbol */

         /* Placeholder for other unused interrupts in the vector table. //TODO: change to bluepill ones
         * The nRF52 should roughly be:
         *   * 0x000..=0x008 - SP, reset vector (above)
         *   * 0x008..=0x040 - Exceptions
         *   * 0x040..=0x0dc - Interrupts
         *
         * This doesn't actually PUT anything here, which would be bad
         * if we ever used interrupts or hit a fault, which we don't,
         * in our example.
         */
         . = 0x080000dc;
    } > FLASH

    PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

    /* ### .text */
    .text _stext :
    {
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > FLASH

    /* ### .rodata */
    .rodata __etext : ALIGN(4)
    {
        *(.rodata .rodata.*);

        /* 4-byte align the end (VMA) of this section.
           This is required by LLD to ensure the LMA of the following .data
           section will have the correct alignment. */
        . = ALIGN(4);
        __erodata = .;
    } > FLASH

    /* ## Sections in RAM */
    /* ### .data */
    .data : AT(__erodata) ALIGN(4)
    {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4); /* 4-byte align the end (VMA) */
        __edata = .;
    } > RAM

    /* LMA of .data */
    __sidata = LOADADDR(.data);

    /* ### .bss */
    .bss : ALIGN(4)
    {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss.*);
        . = ALIGN(4); /* 4-byte align the end (VMA) */
        __ebss = .;
    } > RAM

    /* ### .uninit */
    .uninit (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.uninit .uninit.*);
        . = ALIGN(4);
    } > RAM

    /* Place the heap right after `.uninit` TODO: no need for heap lol*/
    . = ALIGN(4);
    __sheap = .;

    /* ## Discarded sections */
    /DISCARD/ :
    {
        /* Unused exception info */
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
