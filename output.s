.global _start
.align 2
_start:


    mov X1, #10
    str X1, [sp]
    sub sp, sp, #16


    mov X1, #5
    str X1, [sp]
    sub sp, sp, #16

.data
new_line: .ascii "\n"
fn_end: .quad _start
