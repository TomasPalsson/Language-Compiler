extern _printf
section .rodata
fmt: db "%ld", 10, 0
fmt_str: db "%s", 10, 0
str_0: db "Its a 5", 0
str_1: db "B is less than 5", 0
global _main
section .text
_main:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    mov rax, 1
    mov [rbp - 8], rax
while_start_0:
    mov rax, [rbp - 8]
    push rax
    mov rax, 5
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je while_end_1
    mov rsi, [rbp - 8]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    call _otherwhile
    mov rax, [rbp - 8]
    push rax
    mov rax, 1
    pop rcx
    add rax, rcx
    mov [rbp - 8], rax
    mov rax, [rbp - 8]
    push rax
    mov rax, 5
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je endif_2
    lea rsi, [rel str_0]
    lea rdi, [rel fmt_str]
    mov rax, 0
    call _printf
endif_2:
    jmp while_start_0
while_end_1:
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
global _otherwhile
_otherwhile:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    mov rax, 1
    mov [rbp - 16], rax
while_start_3:
    mov rax, [rbp - 16]
    push rax
    mov rax, 5
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je while_end_4
    lea rsi, [rel str_1]
    lea rdi, [rel fmt_str]
    mov rax, 0
    call _printf
    mov rax, [rbp - 16]
    push rax
    mov rax, 1
    pop rcx
    add rax, rcx
    mov [rbp - 16], rax
    jmp while_start_3
while_end_4:
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
