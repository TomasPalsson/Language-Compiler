extern _printf
section .rodata
fmt: db "%ld", 10, 0
global _main
section .text
_main:
;   ; prologue
    push rbp
    mov rbp, rsp
; Reserving space
    sub rsp, 24
;   ; BODY
    mov rax, 2
    push rax
    mov rax, 2
    pop rcx
    add rax, rcx
    push rax
    mov rax, 4
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 8
    pop rcx
    mov rbx, rax
    mov rax, rcx
    mov rdx, 0
    idiv rbx
    mov [rbp - 8], rax
    mov rsi, [rbp - 8]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    call _newfunc
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
global _newfunc
_newfunc:
;   ; prologue
    push rbp
    mov rbp, rsp
; Reserving space
    sub rsp, 24
;   ; BODY
    mov rax, 12
    mov [rbp - 16], rax
    mov rsi, [rbp - 16]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    call _spicy
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
global _spicy
_spicy:
;   ; prologue
    push rbp
    mov rbp, rsp
; Reserving space
    sub rsp, 24
;   ; BODY
   mov rsi, 69
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
