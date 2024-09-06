.global _start
.align 2
_start:


    mov X1, #0
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #10
    str X1, [sp]
    sub sp, sp, #16

    bl l_0_start


    ldr W1, [sp, #32]

    mov W2, #0

    cmp W1, W2

    b.eq equal_0

    bl continue_0
equal_0:
    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 7
    mov X16, #4
    svc #0x80



    bl continue_0


continue_0:

    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 6
    mov X16, #4
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80

f_printstring:
    str X30, [sp]
    sub sp, sp, #16


    mov X1, #10
    str X1, [sp]
    sub sp, sp, #16

    mov X0, #1
    adrp X1, print_string_2@PAGE
    add X1, X1, print_string_2@PAGEOFF
    mov X2, 9
    mov X16, #4
    svc #0x80



    add sp, sp, #32

    ldr X30, [sp]

    ret

l_1_start:
    str X30, [sp]
    sub sp, sp, #16

    mov W1, #0
    str W1, [sp]
    sub sp, sp, #16

    b l_1

l_1:

    bl f_printstring



    add sp, sp, #0

    ldr W11, [sp, #16]
    add W11, W11, #1
    str W11, [sp, #16]

    mov W12, #3
    cmp W12, W11
    b.ne l_1
 
    ldr X30, [sp, #32]

    add sp, sp, #32

    ret


l_0_start:
    str X30, [sp]
    sub sp, sp, #16

    mov W1, #0
    str W1, [sp]
    sub sp, sp, #16

    b l_0

l_0:

    bl l_1_start



    add sp, sp, #0

    ldr W11, [sp, #16]
    add W11, W11, #1
    str W11, [sp, #16]

    mov W12, #2
    cmp W12, W11
    b.ne l_0
 
    ldr X30, [sp, #32]

    add sp, sp, #32

    ret

.data
new_line: .ascii "\n"
print_string_0: .ascii "equals\n"
print_string_1: .ascii "hello\n"
print_string_2: .ascii "printing\n"
