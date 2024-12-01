global _start
section .init.text32
bits 32
_start:
    ; We are now in i386 (32-bit) protected mode.
    ; Interrupts are disabled.
    ; Paging is disabled.
    ; Kernel has full control over the CPU.

    ; Set up the stack
    mov esp, stack_top - KERNEL_OFFSET

    ; Make pointer accessible to rust entry point. Do not change edi before calling entry.
    mov edi, ebx

    ; Checks
    call check_multiboot2   ; Check multiboot2
    call check_cpuid        ; Check CPUID
    call check_long_mode    ; Check long mode

    ; Call your initialization code
    call set_up_page_tables     ; Sets up paging.
    call enable_paging          ; Enables paging.

    ; load the 64-bit GDT
    lgdt [gdt64.pointer_low - KERNEL_OFFSET]

    ; Jump to the address of long_mode_start
    jmp gdt64.code:long_mode_start

check_multiboot2:
	cmp eax, 0x36d76289 ; Check eax if magic exists. GRUB will put magic on eax.
	jne .no_multiboot2
	ret
.no_multiboot2:
	mov al, '0'
	jmp _error_occurred

check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    ; in the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, '1'
	jmp _error_occurred

check_long_mode:
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, '2'
    jmp _error_occurred

global _error_occurred
_error_occurred:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20  ; Print "ERR" at 0xb8000 ; Prints ERROR at 0xb8000
	mov byte  [0xb800a], al ; Display error code (at 'al') as ASCII character
	hlt

set_up_page_tables:
	; Set up recursive paging at the second to last entry
	mov eax, p4_table - KERNEL_OFFSET
	or eax, 11b ; present + writable
	mov [(p4_table - KERNEL_OFFSET) + (510 * 8)], eax

	; map the first P4 entry to the first P3 table
	;
	; This will be changed to the page containing
	; only the first megabyte before rust starts
	mov eax, low_p3_table - KERNEL_OFFSET
	or eax, 11b ; present + writable
	mov [p4_table - KERNEL_OFFSET], eax

	; map the last P4 entry to last P3 table
	mov eax, high_p3_table - KERNEL_OFFSET
	or eax, 11b ; present + writable
	mov [p4_table - KERNEL_OFFSET + (511 * 8)], eax

	; map first entry of the low P3 table to the kernel table
	mov eax, kernel_data_table - KERNEL_OFFSET
	or eax, 11b ; present + writable
	mov [low_p3_table - KERNEL_OFFSET], eax
	; now to the second to highest entry of the high P3 table
	mov [high_p3_table - KERNEL_OFFSET + (510 * 8)], eax

	; map each P2 entry to a huge 2MiB page
	mov ecx, 0x0       ; counter variable

.map_kernel_data_table:
	mov eax, 0x200000  ; 2MiB
	mul ecx            ; eax now holds the start address of the ecx-th page
	or eax, 10000011b  ; present + writable + huge
	mov [(kernel_data_table - KERNEL_OFFSET) + (ecx * 8)], eax ; map ecx-th entry

	inc ecx            ; increase counter
	cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
	jne .map_kernel_data_table  ; else map the next entry

	; map the first P2 entry to the megabyte table
	mov eax, memory_chunk_table - KERNEL_OFFSET
	or eax, 11b
	mov [low_p2_table - KERNEL_OFFSET], eax

	; identity map the first megabyte
	mov ecx, 0x0

.map_memory_chunk_table:
	mov eax, 4096      ; 4Kb
	mul ecx            ; start address of ecx-th page
	or eax, 11b        ; present + writable
	mov [(memory_chunk_table - KERNEL_OFFSET) + (ecx * 8)], eax ; map ecx-th entry

	inc ecx            ; increase counter
	cmp ecx, 256       ; if counter = 256, the whole megabyte is mapped
	jne .map_memory_chunk_table ; else map the next entry

	ret


unmap_guard_page:
	; put the address of the stack guard huge pages into ecx
	mov ecx, (bsp_stack_guard_page - 0x200000 - KERNEL_OFFSET)
	shr ecx, 18      ; calculate P2 index
	and ecx, 0x1FF  ; get P2 index by itself
	; ecx now holds the index into the P2 page table of the entry we want to unmap
	mov eax, 0x0  ; set huge page flag, clear all others
	mov [(kernel_data_table - KERNEL_OFFSET) + ecx], eax ; unmap (clear) ecx-th entry
	ret


enable_paging:
	mov eax, cr4
	or eax, (1 << 7) | (1 << 5) | (1 << 1)
	mov cr4, eax

	; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov eax, p4_table - KERNEL_OFFSET
	mov cr3, eax

	; set the no execute (11), long mode (8), and SYSCALL Enable (0)
	; bits in the EFER
	mov ecx, 0xC0000080
	rdmsr
	or eax, (1 <<11) | (1 << 8) | (1 << 0) ; NXE, LME, SCE
	wrmsr

	; enable paging and write protection in the cr0 register
	mov eax, cr0
	or eax, (1 << 31) | (1 << 16) ; PG | WP
	mov cr0, eax

	ret

;
; --- LONG MODE ---
;

global long_mode_start
section .init.text.high
bits 64
long_mode_start:
    lgdt [rel gdt64.pointer]
    mov rax, start_high
	jmp rax

global start_high
extern rust_bios_entry
section .text
bits 64
start_high:

    add rsp, KERNEL_OFFSET

    ; load gdt64.data into all data segment registers
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; call the rust entry
    extern rust_bios_entry
    call rust_bios_entry
    hlt

section .page_table nobits alloc noexec write
align 4096
p4_table:
    resb 4096

section .bss
low_p3_table:
	resb 4096
high_p3_table:
	resb 4096
low_p2_table:
	resb 4096
memory_chunk_table:
	resb 4096
kernel_data_table:
	resb 4096

; 2MB space here
section .guard_huge_page nobits noalloc noexec nowrite

section .stack nobits alloc noexec write
align 4096 
global bsp_stack_guard_page
bsp_stack_guard_page:
	resb 4096
global bsp_stack_bottom
stack_bottom:
	resb 16384
global bsp_stack_top
stack_top:
	resb 4096
double_fault_stack_top:

KERNEL_OFFSET equ 0xFFFFFFFF80000000 ; This is the kernel offset.

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) ; code segment
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) ; data segment
.pointer_low:
    dw $ - gdt64 - 1
    dd gdt64 - KERNEL_OFFSET
.pointer:
    dw $ - gdt64 - 1
    dq gdt64