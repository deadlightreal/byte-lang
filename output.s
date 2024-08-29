.global _start
.align 2
_start:

    mov W1, #20

    mov W2, #20

    cmp W1, W2

    b.eq equal_0

    b.ne not_equal_0

equal_0:
    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 8
    mov X16, #4
    svc #0x80

    mov W1, #10

    mov W2, #10

    cmp W1, W2

    b.eq equal_1

    b.ne not_equal_1

equal_1:
    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 8
    mov X16, #4
    svc #0x80



    bl continue_1
not_equal_1:
    mov X0, #1
    adrp X1, print_string_2@PAGE
    add X1, X1, print_string_2@PAGEOFF
    mov X2, 9
    mov X16, #4
    svc #0x80



    bl continue_1
    bl continue_1

continue_1:



    bl continue_0
not_equal_0:
    mov X0, #1
    adrp X1, print_string_3@PAGE
    add X1, X1, print_string_3@PAGEOFF
    mov X2, 9
    mov X16, #4
    svc #0x80



    bl continue_0
    bl continue_0

continue_0:

    mov X0, #0
    mov X16, #1
    svc #0x80

.data
new_line: .ascii "\n"
print_string_0: .ascii "20 = 20\n"
print_string_1: .ascii "10 = 10\n"
print_string_2: .ascii "10 != 10\n"
print_string_3: .ascii "20 != 20\n"
