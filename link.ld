
ENTRY(_start)

/* Program Headers */
PHDRS {
    /* stack segment readable and writable b110 */
    segment_boot_core_stack PT_LOAD FLAGS(6);
    /* code segment readable and executable b101 */
    segment_code PT_LOAD FLAGS(5);
    /* data segment readable and writable b110 */
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS {
    . = 0;

    /* Stack */
    .boot_core_stack (NOLOAD) : {
        . += 0x80000;
        __boot_core_stack_end = .;
    } :segment_boot_core_stack

    /* Code */
    .text : {
        KEEP(*(.text._start))
        *(.text.__start_argments)
        *(.text.__start_rust)
        *(text*)
    } :segment_code

    /* Data */
    .data : {
        *(.data*)
    } :segment_data

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end = .;
    } :segment_data

    /* Misc */
    .got : {
        *(.got*)
    }

    /DISCARD/ : { *(.comment*) }
}
