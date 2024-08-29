.global _start
.align 2
_start:

    mov W1, #30

    mov W2, #20

    cmp W1, W2

    b.ge greater_equal_0

    b.lt less_than_0

greater_equal_0:
    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 14
    mov X16, #4
    svc #0x80



    bl continue_0
less_than_0:
    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 10
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
print_string_0: .ascii "greater equal\n"
print_string_1: .ascii "less than\n"
