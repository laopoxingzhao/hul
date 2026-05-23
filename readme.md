# hul

A lightweight scripting language interpreter implemented in Rust, with zero external dependencies.

```
hu> let sum = 0;
hu> for (let i = 1; i <= 100; i = i + 1) { sum = sum + i; }
hu> print sum;
5050
```

## Features

- Variables, arithmetic, comparisons, logical operators
- String literals with escape sequences and concatenation (`"age=" + 25`)
- Control flow: `if` / `else if` / `else`, `while`, `for`
- Loop control: `break`, `continue`
- Functions with recursion and closures
- Lexical scoping with block scope
- Single-line (`//`) and multi-line (`/* */`) comments
- REPL and file execution modes
- Error recovery: reports multiple parse errors in one pass

## Getting Started

```bash
# Build
cargo build

# Run REPL
cargo run

# Run a script
cargo run -- examples/test.hu

# Run tests
cargo test
```

## Documentation

| Document | Description |
|----------|-------------|
| [docs/PRINCIPLES.md](docs/PRINCIPLES.md) | How the interpreter works — lexing, parsing, AST, execution |
| [docs/SYNTAX.md](docs/SYNTAX.md) | Language syntax reference with examples |

## Project Structure

```
src/
├── main.rs          CLI entry (REPL + file mode)
├── lib.rs           Public API
├── lexer.rs         Character stream → Token stream
├── parser.rs        Token stream → AST
├── ast.rs           AST node definitions
├── interpreter.rs   AST traversal and execution
└── value.rs         Runtime values + scope chain
```

## License

MIT
