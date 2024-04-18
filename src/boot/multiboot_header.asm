; This file contains the header for the Multiboot2 standard. It allows GRUB to identify that this is a bootable kernel image.
; Must be in the first section in the kernel image, done by the linker script.
section .multiboot
align 4                                                                                 ; Align to 4 byte boundry
multiboot_header_start:
    dd 0xe85250d6                                                                       ; Magic value identifying the header
    dd 0                                                                                ; Architecture, runs in i386 32-bit protected
    dd multiboot_header_end - multiboot_header_start                                    ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (multiboot_header_end - multiboot_header_start)) ; Checksum

multiboot_header_end: