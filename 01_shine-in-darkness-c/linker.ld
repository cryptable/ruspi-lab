ENTRY(_start)


SECTIONS
{
    . = 0x80000;
    .text :
    {
        KEEP(*(.text.start))
        *(.text)
    }
    . = ALIGN(32); /* align to page size */
 
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(32); /* align to page size */
 
    .data :
    {
        *(.data)
    }
    . = ALIGN(32); /* align to page size */
 
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(32); /* align to page size */
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
}
