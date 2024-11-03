.global _start
.align 2
.text
_start:

mov x0, #10
str x0, [sp]
sub sp, sp, #16
mov x0, #1
adrp x1, static_0@PAGE
add x1, x1, static_0@PAGEOFF
mov x2, #12
mov x16, #4
svc 0x80
mov x0, #1
adrp x1, static_1@PAGE
add x1, x1, static_1@PAGEOFF
mov x2, #12
mov x16, #4
svc 0x80
mov x0, #1
adrp x1, static_2@PAGE
add x1, x1, static_2@PAGEOFF
mov x2, #12
mov x16, #4
svc 0x80
mov x0, #1
adrp x1, static_3@PAGE
add x1, x1, static_3@PAGEOFF
mov x2, #12
mov x16, #4
svc 0x80
mov x0, #1
adrp x1, static_4@PAGE
add x1, x1, static_4@PAGEOFF
mov x2, #13
mov x16, #4
svc 0x80
mov x0, #0
mov x16, #1
svc #0X80
.data
static_0: .ascii "hello world\n"
static_1: .ascii "hello world\n"
static_2: .ascii "hello world\n"
static_3: .ascii "hello world\n"
static_4: .ascii "hello world\n\n"
