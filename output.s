.global _start
.align 2
_start:

    bl l_0

l_0:

    bl f_test


    adrp X13, l_0_index@PAGE   
    add X13, X13, l_0_index@PAGEOFF
    ldr W11, [X13]

    adrp X14, l_0_limit@PAGE
    add X14, X14, l_0_limit@PAGEOFF
    ldr W12, [X14]

    cmp W12, W11
    b.eq l_0_end

    add W11, W11, #1
    str W11, [X13]
    bl l_0

l_0_end:
    mov W11, #1
    str W11, [X13]

    mov X0, #0
    mov X16, #1
    svc #0x80

f_test:
        mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 12
    mov X16, #4
    svc #0x80


    
    ret

.data
new_line: .ascii "\n"
l_0_limit: .word 5
l_0_index: .word 1
print_string_0: .ascii "hello world\n"
