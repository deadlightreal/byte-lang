.global _start
.align 2
_start:


    sub sp, sp, #16
    mov X1, #0
    str X1, [sp, #0]


    sub sp, sp, #16
    mov X1, #11
    str X1, [sp, #16]

    mov W1, #10


    ldr W2, [sp, #16]

    cmp W1, W2

    b.eq equal_0

    b.ne not_equal_0

    bl continue_0
equal_0:
    mov X0, #1
    adrp X1, print_string_0@PAGE
    add X1, X1, print_string_0@PAGEOFF
    mov X2, 6
    mov X16, #4
    svc #0x80



    bl continue_0
not_equal_0:
    mov X0, #1
    adrp X1, print_string_1@PAGE
    add X1, X1, print_string_1@PAGEOFF
    mov X2, 10
    mov X16, #4
    svc #0x80



    bl continue_0


continue_0:

    mov X0, #0
    mov X16, #1
    svc #0x80

.data
new_line: .ascii "\n"
fn_end: .quad _start
print_string_0: .ascii "equal\n"
print_string_1: .ascii "not equal\n"
