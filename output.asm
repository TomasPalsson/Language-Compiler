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
    push rax
    mov rax, 4
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    mov [rbp - 8], rax
    mov rax, [rbp - 8]
    push rax
    mov rax, 5
    pop rcx
    add rax, rcx
    mov [rbp - 16], rax
    mov rax, [rbp - 16]
    push rax
    mov rax, 15
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je endif_0
    call _iffunc
endif_0:
    mov rsi, [rbp - 16]
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
    mov [rbp - 24], rax
    mov rsi, [rbp - 24]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    call _spicy
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
global _iffunc
_iffunc:
;   ; prologue
    push rbp
    mov rbp, rsp
; Reserving space
    sub rsp, 24
;   ; BODY
   mov rsi, 112
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
   mov rsi, 112
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
   mov rsi, 112
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
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
