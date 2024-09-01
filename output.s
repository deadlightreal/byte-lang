.global _start
.align 2
_start:

    adr X10, f_func_end
    adrp X11, fn_end@PAGE
    add X11, X11, fn_end@PAGEOFF
    str X10, [X11]
    bl f_func

f_func_end:

    adrp X3, five@PAGE
    add X3, X3, five@PAGEOFF
    ldr W1, [X3]

    mov W2, #5

    cmp W1, W2

    b.eq equal_0

    b.ne not_equal_0

    bl continue_0
equal_0:
    mov x8, 1
    stp x8, xzr, [sp, -0x10]!
    mov x0, sp
    mov x1, 0
    bl _nanosleep
    add sp, sp, 0x10

    mov X0, #1
    adrp X1, printstring@PAGE
    add X1, X1, printstring@PAGEOFF
    adrp X3, printstring_end@PAGE
    add X3, X3, printstring_end@PAGEOFF
    sub X2, X3, X1
    mov X16, #4
    svc #0x80

    mov X0, #1
    adrp X1, new_line@PAGE
    add X1, X1, new_line@PAGEOFF
    mov X2, 1
    mov X16, #4
    svc #0x80

    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 9
    mov X16, #4
    svc #0x80



    bl continue_0
not_equal_0:
    mov X0, #1
    adrp X1, print_string_2@PAGE
    add X1, X1, print_string_2@PAGEOFF
    mov X2, 10
    mov X16, #4
    svc #0x80



    bl continue_0


continue_0:

    mov X0, #1
    adrp X1, printstring@PAGE
    add X1, X1, printstring@PAGEOFF
    adrp X3, printstring_end@PAGE
    add X3, X3, printstring_end@PAGEOFF
    sub X2, X3, X1
    mov X16, #4
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80

f_func:

    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 14
    mov X16, #4
    svc #0x80


    adrp X10, fn_end@PAGE
    add X10, X10, fn_end@PAGEOFF
    ldr X11, [X10]
    br X11

.data
new_line: .ascii "\n"
fn_end: .quad _start
print_string_0: .ascii "executed func\n"
print_string_1: .ascii "five = 5\n"
print_string_2: .ascii "five != 5\n"
printstring: .asciz "print"
printstring_end:
printstring_length: .word 5
five: .word 5
ten: .word 10
