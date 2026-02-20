# Bonk Language Compiler

## Build & Run

```bash
make run                              # compile + assemble + link + run examples/basic.bonk
make run FILE=examples/basic.bonk    # explicit file
make clean                            # remove build artifacts
cargo build                           # build compiler only
cargo run -- input.bonk output.asm   # run compiler directly
```

Build output goes to `build/` (asm, object file, binary).

## Architecture

Pipeline: **source → lexer → parser → compiler → x86-64 NASM assembly**

| File | Role |
|------|------|
| `src/main.rs` | CLI entry point — reads source, runs pipeline, writes `.asm` |
| `src/lexer.rs` | Tokenizer — converts source text to `Vec<Token>` |
| `src/tokens.rs` | Token enum definition |
| `src/parser.rs` | Recursive descent parser — tokens to AST (`Vec<Statement>`) |
| `src/ast.rs` | AST types: `Statement`, `Expression`, `BinaryOperator` |
| `src/compiler.rs` | Code generator — walks AST, emits x86-64 NASM lines |
| `runtime/http.c` | C runtime — HTTP fetch via libcurl, linked into final binary |

## Language Syntax

```
run main()            # entry point (must exist)
  x = 10;            # variable assignment
  print x;           # print integer or string
  print "hello";     # string literal
  x = ~add(1, 2);   # function call with return value
  ~foo(1, 2);        # function call as statement (~ prefix)
end

run add(a, b)         # function with return value
  send a + b;        # return expression (like return)
end

run foo(a, b)         # function with parameters
  if a == b then      # if-else
    print "equal";
  else
    print "not equal";
  end

  while a > 0 do      # while loop
    a = a - 1;
  end
end
```

### HTTP Fetch

```
run main()
  result = fetch "http://example.com";        # GET request (1 arg = URL)
  print result;

  resp = fetch "POST" "http://api.com" "data"; # method + URL + body
  print resp;

  fetch "DELETE" "http://api.com/item";        # fire-and-forget (no body)
end
```

- 1 arg: `fetch URL` — implicit GET
- 2 args: `fetch METHOD URL` — custom method, no body
- 3 args: `fetch METHOD URL BODY` — custom method with request body
- Returns the response body as a string
- Works as both expression (`x = fetch ...;`) and statement (`fetch ...;`)

Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`

## Platform

- Target: macOS x86-64 (`nasm -f macho64`, symbols prefixed with `_`)
- Links against libc for `_printf` and libcurl for HTTP fetch
- On Apple Silicon: uses `gcc -arch x86_64` to cross-link; runs via Rosetta
- Requires: Rust, NASM, GCC, libcurl
