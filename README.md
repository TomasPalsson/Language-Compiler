# Compiler
Basic compiler written in Rust that compiles a simple language to x86 assembly.

# Example code 
```
run main()
    a = 1;
    b = 2;
    c = a + b;
    print c;
    if c == 3 then
        print a;
    else 
        print b;
    end
    ~func;

run func()
    print 1111;
end
```
Results in the following assembly code:
```assembly
extern _printf
section .rodata
fmt: db "%ld", 10, 0
global _main
section .text
_main:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    mov rax, 1
    mov [rbp - 8], rax
    mov rax, 2
    mov [rbp - 16], rax
    mov rax, [rbp - 8]
    push rax
    mov rax, [rbp - 16]
    pop rcx
    add rax, rcx
    mov [rbp - 24], rax
    mov rsi, [rbp - 24]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    mov rax, [rbp - 24]
    push rax
    mov rax, 3
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je else_1
    mov rsi, [rbp - 8]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    jmp endif_0
else_1:
    mov rsi, [rbp - 16]
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
endif_0:
    call _func
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
global _func
_func:
    push rbp
    mov rbp, rsp
    sub rsp, 24
   mov rsi, 1111
    lea rdi, [rel fmt]
    mov rax, 0
    call _printf
    mov rsp, rbp
    mov rax, 0
    pop rbp
    ret
```

and output
```
3
1
1111
```


# Todo
- [ ] Add loops
- [ ] Add strings 
