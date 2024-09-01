.global _start
.align 2
_start:

    bl l_0_start

    mov X0, #1
    adrp X1, print_string_2@PAGE
    add X1, X1, print_string_2@PAGEOFF
    mov X2, 13
    mov X16, #4
    svc #0x80

    mov X0, #0
    mov X16, #1
    svc #0x80


l_1_start:
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    adrp X19, l_1_return@PAGE
    add X19, X19, l_1_return@PAGEOFF

    str X30, [X19]

    b l_1

l_1:

    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 10
    mov X16, #4
    svc #0x80



    adrp X13, l_1_index@PAGE   
    add X13, X13, l_1_index@PAGEOFF
    ldr W11, [X13]
    add W11, W11, #1
    str W11, [X13]

    adrp X14, l_1_limit@PAGE
    add X14, X14, l_1_limit@PAGEOFF
    ldr W12, [X14]

    cmp W12, W11
    b.ne l_1

    mov W15, #0
    str W15, [X13]

    adrp X19, l_1_return@PAGE
    add X19, X19, l_1_return@PAGEOFF

    ldr X30, [X19]

    ret


l_0_start:
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    adrp X19, l_0_return@PAGE
    add X19, X19, l_0_return@PAGEOFF

    str X30, [X19]

    b l_0

l_0:

    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 10
    mov X16, #4
    svc #0x80

    bl l_1_start



    adrp X13, l_0_index@PAGE   
    add X13, X13, l_0_index@PAGEOFF
    ldr W11, [X13]
    add W11, W11, #1
    str W11, [X13]

    adrp X14, l_0_limit@PAGE
    add X14, X14, l_0_limit@PAGEOFF
    ldr W12, [X14]

    cmp W12, W11
    b.ne l_0

    mov W15, #0
    str W15, [X13]

    adrp X19, l_0_return@PAGE
    add X19, X19, l_0_return@PAGEOFF

    ldr X30, [X19]

    ret

.data
new_line: .ascii "\n"
fn_end: .quad _start
l_0_limit: .word 5
l_0_index: .word 0
l_0_return: .quad 0
l_1_limit: .word 5
l_1_index: .word 0
l_1_return: .quad 0
print_string_0: .ascii "in loop 1\n"
print_string_1: .ascii "in loop 2\n"
print_string_2: .ascii "outside loop\n"
