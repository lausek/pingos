global start
extern long_mode_start

section .text
bits 32
start:
    
    ; set stack pointer
    mov esp, stack_top
    ; set multiboot info
    mov edi, ebx

    ; more bootloader stuff goes here
    call check_multiboot
    call check_cpuid
    call check_long_mode
    
    ; set up paging
    call init_page_table
    call enable_paging
    
    ; load GDT
    lgdt [gdt64.pointer]

    jmp gdt64.code:long_mode_start

    hlt

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

check_multiboot:
    ; TODO: implement multiboot check
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error 
    ret

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
    mov al, "1"
    jmp error

check_long_mode:
    mov eax, 0x80000000
    cpuid
    mov eax, 0x80000001
    jb .no_long_mode

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jb .no_long_mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

init_page_table:
    mov eax, p3
    ; present + writeable
    or eax, 0b11 
    ; write address in eax to p4 table
    mov [p4], eax

    mov eax, p2
    or eax, 0b11 
    ; write address in eax to p4 table
    mov [p3], eax

    mov ecx, 0

; set each p2 entry to a huge 2MiB page
.map_p2:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011 
    mov [p2 + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2

    ret

enable_paging:
    ; 1. set p4 table address to cr3 register
    mov eax, p4
    mov cr3, eax

    ; 2. set PAE-flag (physical address extension) 
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; 3. set the long mode flag
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; 4. enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

; read only data
section .rodata
gdt64:
    dq 0
.code : equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

; data segment 
section .bss
; required by paging
; resb = reserve X bytes uninitialized memory
align 4096
HEAP_BOTTOM:
p4:
    resb 4096
p3:
    resb 4096
p2:
    resb 4096
HEAP_TOP:

; initialize stack
stack_bottom:
    resb 4096 * 4
stack_top:
