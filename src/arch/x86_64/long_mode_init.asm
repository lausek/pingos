global long_mode_start

section .text
bits 64
long_mode_start:
    ; reset data registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; call kernel main function
    extern kmain
    call kmain

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt
