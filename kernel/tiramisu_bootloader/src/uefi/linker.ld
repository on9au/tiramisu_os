ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFFFF80000000;

SECTIONS {
    . = 1M;                         /* Start at 1MB */

    /* Multiboot Header */
    .init ALIGN(4K) :
    {
        /* Link the Multiboot header. Multiboot header must always be on top. */
        KEEP(*(.multiboot))
        *(.init .init.*)
    }

    . += KERNEL_OFFSET;

    /* Text Section */
    .text ALIGN(4K) : AT (ADDR(.text) - KERNEL_OFFSET)
    {
        /* Link the kernel code */
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET)
	{
		*(.rodata .rodata.*)
	}

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET)
	{
		*(.bss .bss.*)
	}

    .page_table ALIGN(4K) : AT(ADDR(.page_table) - KERNEL_OFFSET)
	{
		*(.page_table)
	}

    . = ALIGN(2M);
	. = . + 2M; /* make space for a 2MiB stack guard page */
	.stack ALIGN(4K) : AT(ADDR(.stack) - KERNEL_OFFSET)
	{
		*(.stack)
	}
}
