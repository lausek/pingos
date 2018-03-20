section .multiboot_header

header_start:
    dd 0xe85250d6                   ; magic number
    dd 0                            ; arch
    dd header_end - header_start    ; header size
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; more multiboot tags here

    dw 0                            ; type
    dw 0                            ; flags
    dd 0                            ; size

header_end:
