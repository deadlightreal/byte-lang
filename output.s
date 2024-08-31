.global _start
.align 2
_start:

    mov X0, #0
    mov X16, #1
    svc #0x80

.data
new_line: .ascii "\n"
fn_end: .quad _start
test: .asciz ""
test_end:
test_length: .word 0 
num: .word 10
boolean: .byte 0
