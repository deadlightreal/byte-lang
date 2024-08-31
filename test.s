.global _start
.align 3

_start:
    mov X16, #197
    mov X0, #0
    mov X1, #4096
    mov X2, #3
    mov X3, 0x1002
    mov X4, #-1
    mov X5, #0
    svc #0x80

    mov X9, X0

    adrp X10, string@PAGE
    add X10, X10, string@PAGEOFF
    mov X11, #6
    bl memcpy

    mov x15, x9            // x9 already points to the "World" string
    sub x9, x9, #6         // Move x9 back to the "Hello" string
    mov x1, x9
    mov x2, 6
    mov X16, #4
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80

memcpy:
    cbz X11, .memcpy_done
    ldrb W12, [X10], #1
    strb W12, [X9], #1
    sub X11, X11, #1
    b memcpy

.memcpy_done:
    ret

.data
string: .asciz "hello"
