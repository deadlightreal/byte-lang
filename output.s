.global _start
.align 2
_start:


    mov X1, #5
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #4
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #10
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #0
    str X1, [sp]
    sub sp, sp, #16

    bl f_fn_test

    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 12
    mov X16, #4
    svc #0x80

    mov W1, #5


    ldr W2, [sp, #48]

    cmp W1, W2

    b.eq equal_0

    bl continue_0
equal_0:
    bl f_fn_test

    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 13
    mov X16, #4
    svc #0x80



    bl continue_0


continue_0:

    mov X0, #0
    mov X16, #1
    svc #0x80

f_fn_test:
    str X30, [sp]
    sub sp, sp, #16


    mov X1, #0
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #1
    str X1, [sp]
    sub sp, sp, #16



    add sp, sp, #48

    ldr X30, [sp]
    
    add sp, sp, #16

    ret
.data
new_line: .ascii "\n"
fn_end: .quad _start
print_string_0: .ascii "after first\n"
print_string_1: .ascii "after second\n"
