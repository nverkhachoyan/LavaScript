# LavaScript Compiler

A compiler for the LavaScript programming language that transpiles to JavaScript, written in Rust.

## Language Features

- Object-oriented programming with class-based inheritance
- Static type checking
- Variable initialization checking
- Return checking for non-void functions
- Method overloading
- Traditional non-S-expression syntax

## Project Structure

```
src/
├── main.rs           # Entry point
├── lexer/           # Lexical analysis
├── parser/          # Syntax analysis
├── ast/             # Abstract Syntax Tree definitions
├── typechecker/     # Type checking and semantic analysis
├── codegen/         # JavaScript code generation
└── error/           # Error handling and reporting
```

## Building

1. Install Rust and Cargo using [rustup](https://rustup.rs/)
2. Clone this repository
3. Run `cargo build` to build the project
4. Run `cargo test` to run the test suite

## Usage

```bash
cargo run -- input.ls
```

This will compile the LavaScript source file `input.ls` and output JavaScript code.

## Example

```rust
// Example LavaScript code
class Animal {
  init() {}
  method speak() Void { return println(0); }
}

class Cat extends Animal {
  init() { super(); }
  method speak() Void { return println(1); }
}

Animal cat;
cat = new Cat();
cat.speak();
```

## Development Status

- [ ] Lexer (10%)
- [ ] Parser (20%)
- [ ] Type Checker (40%)
- [ ] Code Generator (30%)

## License

MIT
