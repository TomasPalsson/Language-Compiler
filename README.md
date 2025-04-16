# Compiler
Basic (Turing Complete) compiler written in Rust that compiles a simple language to x86 assembly.

# Example code 
```
run main() 
  a = 1;
  while a > 5 do 
      print a;
      print "Hello ";
      a = a + 1;
    if a == 5 then
      print "Its a 5";
    end
  end
end
```
Results in the following assembly code:
```assembly
extern _printf
section .rodata
fmt: db "%ld", 10, 0
fmt_str: db "%s", 10, 0
str_0: db "Hello ", 0
str_1: db "Its a 5", 0
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
    lea rsi, [rel str_0]
    lea rdi, [rel fmt_str]
    mov rax, 0
    call _printf
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
    lea rsi, [rel str_1]
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
```

and output
```
1
Hello 
2
Hello 
3
Hello 
4
Hello 
Its a 5
```
