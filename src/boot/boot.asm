section .text
global _start
_start:
    ; Set up the stack
    mov rsp, stack_end

    ; Call your initialization code
    call bootloader_init

    ; Jump to the Rust kernel entry point
    jmp higher_half_kernel_entry

bootloader_init:
    ; Your initialization code here
    ; This may involve setting up the environment,
    ; configuring hardware, etc.
    ret

section .bss
align 8
_end: