global start

section .text
bits 32
start:
    
    ; set stack pointer
    mov esp, stack_top

    ; more bootloader stuff goes here
    
    call check_multiboot
    call check_cpuid
    call check_long_mode
    
    ; set up paging
    call init_page_table
    call enable_paging

    hlt

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

check_multiboot:
    ; TODO: implement multiboot check
    ret

check_cpuid:
    ; TODO: implement CPUID check
    ret

check_long_mode:
    ; TODO: implement multiboot check
    ret

init_page_table:
    ; TODO: enable paging
    ret

enable_paging:
    ; TODO: enable paging
    ret

; data segment 
section .bss
; required by paging
; resb = reserve X bytes uninitialized memory
align 4096
p4:
    resb 4096
p3:
    resb 4096
p2:
    resb 4096
stack_bottom:
    resb 64
stack_top:
