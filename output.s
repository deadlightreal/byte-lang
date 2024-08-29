.global _start
.align 2
_start:

    adr X3, ten
    ldr W1, [X3]

    mov W2, #10

    cmp W1, W2

    b.eq equal_0

    b.ne not_equal_0

    bl continue_0

continue_0:

    mov X0, #0
    mov X16, #1
    svc #0x80

equal_0:
    adr X3, five
    ldr W1, [X3]

    mov W2, #5

    cmp W1, W2

    b.eq equal_1

    b.ne not_equal_1

    bl continue_1

continue_1:



    bl continue_0
not_equal_0:
    mov X0, #1
    adr X1, print_string_0
    mov X2, 10
    mov X16, #4
    svc #0x80



    bl continue_0
equal_1:
    mov X12, #5
    mov X11, #1
    bl l_0

l_0:

    mov X0, #1
    adr X1, print_string_1
    mov X2, 9
    mov X16, #4
    svc #0x80

    cmp X12, X11
    b.eq l_0_end

    add X11, X11, #1
    bl l_0

l_0_end:



    bl continue_1
not_equal_1:
    mov X0, #1
    adr X1, print_string_2
    mov X2, 10
    mov X16, #4
    svc #0x80



    bl continue_1
new_line: .ascii "\n"
print_string_0: .ascii "ten != 10\n"
print_string_1: .ascii "five = 5\n"
print_string_2: .ascii "five != 5\n"
five: .word 5
printstring: .asciz "print"
printstring_end:
printstring_length: .word 5 
ten: .word 10
