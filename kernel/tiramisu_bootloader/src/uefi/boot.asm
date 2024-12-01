section .text
global _start

_start:
    ; UEFI entry point
    ; Set up stack, call rust entry point
    extern rust_uefi_entry
    mov rsp, rust_uefi_entry
    call rust_uefi_entry
    hlt

section .bss
align 16
stack_bottom:
    resb 16384
stack_top: