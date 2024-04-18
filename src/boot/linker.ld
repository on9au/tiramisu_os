ENTRY(_start)

SECTIONS {
    . = 1M;                         /* Start at 1MB */

    /* Multiboot Header */
    .multiboot :
    {
        /* Link the Multiboot header */
        *(.multiboot)
    }

    /* Text Section */
    .text :
    {
        /* Link the kernel code */
        *(.text)
    }

    /* BSS Section */
    .bss :
    {
        /* Link the BSS section */
        *(.bss)
    }

    /* End of kernel */
    _end = .;                       /* Define _end symbol */
}
