.global _start
.align 2
_start:


    mov X1, #1
    mov X2, #2
    mov X0, #0
    mov X16, #1
    svc #0x80

.data
new_line: .ascii "\n"
fn_end: .quad _start
