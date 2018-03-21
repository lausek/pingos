ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* set multiboot header at the start */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }

}
