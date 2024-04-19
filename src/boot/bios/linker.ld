ENTRY(_start)

SECTIONS {
    . = 1M;                         /* Start at 1MB */

    /* Multiboot Header */
    .boot :
    {
        /* Link the Multiboot header. Multiboot header must always be on top. */
        KEEP(*(.multiboot))
    }

    /* Text Section */
    .text :
    {
        /* Link the kernel code */
        *(.text)
    }
}
