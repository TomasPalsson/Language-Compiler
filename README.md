# Bonk

A compiled programming language written in Rust. Bonk compiles to x86-64 NASM assembly, targeting macOS.

```
run fibonacci(n)
  if n <= 0 then
    send 0;
  end
  if n == 1 then
    send 1;
  else
    send ~fibonacci(n - 1) + ~fibonacci(n - 2);
  end
end

run main()
  x = ~fibonacci(10);
  print x;
end
```

## Quick Start

```bash
make run                           # compile + assemble + link + run examples/basic.bonk
make run FILE=examples/basic.bonk  # explicit file
```

### Requirements

- [Rust](https://rustup.rs/)
- [NASM](https://www.nasm.us/)
- GCC (for linking against libc)
- macOS x86-64 (runs via Rosetta on Apple Silicon)

## Language Reference

### Functions

Every Bonk program needs a `main` function. Functions are defined with `run` and closed with `end`:

```
run main()
  print "hello";
end
```

Functions can take parameters:

```
run greet(name)
  print name;
end
```

### Variables

Variables are assigned with `=`. No declaration needed — assignment creates the variable:

```
x = 10;
y = x + 5;
```

### Return Values (`send`)

Functions return values using `send`. Without `send`, a function returns `0` by default:

```
run add(a, b)
  send a + b;
end

run main()
  x = ~add(3, 5);
  print x;
end
```

`send` exits the function immediately, like `return` in other languages.

### Function Calls

Function calls use the `~` prefix. They can appear as statements or inside expressions:

```
~greet("world");           # statement (discard return value)
x = ~add(1, 2);           # expression (capture return value)
y = ~add(~add(1, 2), 3);  # nested calls
```

### Printing

`print` outputs integers or strings to stdout:

```
print 42;
print "hello";
print x;
print a + b;
```

### Operators

| Operator | Description |
|----------|-------------|
| `+`      | Addition |
| `-`      | Subtraction |
| `*`      | Multiplication |
| `/`      | Division |
| `==`     | Equal |
| `!=`     | Not equal |
| `<`      | Less than |
| `<=`     | Less than or equal |
| `>`      | Greater than |

### Control Flow

**If-else:**

```
if x == 1 then
  print "one";
else
  print "not one";
end
```

The `else` branch is optional:

```
if x > 0 then
  print "positive";
end
```

**While loops:**

```
while x > 0 do
  print x;
  x = x - 1;
end
```

## Build Commands

| Command | Description |
|---------|-------------|
| `make run` | Full pipeline: compile, assemble, link, execute |
| `make run FILE=path.bonk` | Run a specific source file |
| `make compile` | Compile `.bonk` source to assembly |
| `make assemble` | Assemble to object file |
| `make link` | Link into executable |
| `make clean` | Remove build artifacts |
| `cargo build` | Build the compiler only |
| `cargo run -- input.bonk output.asm` | Run compiler directly |

## Architecture

```
source.bonk → Lexer → Parser → Compiler → output.asm → NASM → GCC → binary
```

| File | Role |
|------|------|
| `src/main.rs` | CLI entry point |
| `src/lexer.rs` | Tokenizer — source text to tokens |
| `src/tokens.rs` | Token enum definition |
| `src/parser.rs` | Recursive descent parser — tokens to AST |
| `src/ast.rs` | AST types: `Statement`, `Expression`, `BinaryOperator` |
| `src/compiler.rs` | Code generator — AST to x86-64 NASM assembly |
