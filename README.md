# Compiler

A simple, Turing-complete compiler written in Rust. It compiles a minimal high-level language to x86-64 NASM assembly.

---

## Features

- Full function definitions and calls
- Local variable declarations and assignments
- Integer and string literals
- While loops and if-else conditionals
- Printing to stdout (`print`)
- Expression evaluation with support for:
  - Arithmetic: `+`, `-`, `*`, `/`
  - Comparisons: `==`, `!=`, `<`, `>`

---

## ⚙️ Example Code

```plaintext
run main() 
  a = 1;
  while a > 5 do 
      print a;
      ~otherwhile;
      a = a + 1;
    if a == 5 then
      print "Its a 5";
    end
  end

end


run otherwhile()
  b = 1;
  while b > 5 do 
    print "B is less than 5";
    b = b + 1;
  end
end
```

---

## Assembly Output

```nasm
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
...
```
[See rest](output.asm)

---

## Output

```plaintext
1
B is less than 5
B is less than 5
B is less than 5
B is less than 5
2
B is less than 5
B is less than 5
B is less than 5
B is less than 5
3
B is less than 5
B is less than 5
B is less than 5
B is less than 5
4
B is less than 5
B is less than 5
B is less than 5
B is less than 5
Its a 5
```

---

## Internals

The compiler builds an intermediate AST and walks it to emit assembly. It currently uses:

- Stack-based variable allocation
- Custom parsing for function bodies, expressions, control flow
- A simple string pool to deduplicate literals

Argument parsing and argument passing for functions is **in progress**.

---

## Build Requirements

- **Rust** (to compile the compiler)
- **NASM** (to assemble output)
- **GCC** or compatible linker (to link with libc/printf)

---

## Usage

```bash
cargo run path/to/source.lang > out.asm
nasm -f elf64 out.asm -o out.o
gcc out.o -o prog
./prog
```

---

## 📌 TODO

- [ ] Argument parsing + register/stack-based argument passing
- [ ] Return values
- [ ] More control structures (`for`, `break`, etc.)
