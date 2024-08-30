.global _start
.align 2
_start:

    adr X10, f_test_end
    adrp X11, fn_end@PAGE
    add X11, X11, fn_end@PAGEOFF
    str X10, [X11]
    bl f_test

f_test_end:
    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 22
    mov X16, #4
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80

f_test:
        mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 17
    mov X16, #4
    svc #0x80


    
    adrp X10, fn_end@PAGE
    add X10, X10, fn_end@PAGEOFF
    ldr X11, [X10]
    br X11

.data
new_line: .ascii "\n"
fn_end: .quad _start
print_string_0: .ascii "in test function\n"
print_string_1: .ascii "outside test function\n"
