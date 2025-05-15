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

This will compile the LavaScript source file `input.lava` and output JavaScript code.

## Example

```rust
// Example LavaScript code
class Animal {
	init() {}
	meth speak() -> Int {return 0; }
}

class Cat extends Animal {
	init(name: Str, my_num: Int) { super(); }
	meth speak() -> Int { return 1; }
}

fun greet(name: Str) -> Void {
	println(name);
}

let cat: Animal = new Cat();
cat.speak();

greet("Hello");
```

## Development Status

- [x] Lexer (100%)
- [x] Parser (100%)
- [ ] Type Checker (0%)
- [x] Code Generator (100%)

## ABNF

```abnf
string-char = ALPHA / DIGIT / SP / %x21 / %x23-26 / %x28-3F / %x40-5B 
            / %x5D-60 / %x7B-7E 
            / %x5C %x6E    ; \n
            / %x5C %x74    ; \t
            / %x5C %x72    ; \r
            / %x5C %x22    ; \"
            / %x5C %x27    ; \'
            / %x5C %x5C    ; \\

string-literal = %x22 *string-char %x22
; hex notation %x31-39 for digits 1-9
integer-literal = "0" / (%x31-39 *DIGIT) 
identifier = 1*ALPHA *(DIGIT / "_")

var = identifier
funcname = identifier
classname = identifier
methodname = identifier

str = string-literal
i = integer-literal
type = "Int" / "Str" / "Boolean" / "Void" / classname

comma-exp = [exp *("," exp)]

primary-exp = var
            / str
            / i
            / "(" exp ")"
            / "this"
            / "true"
            / "false"
            / "println" "(" exp ")"
            / "print" "(" exp ")"
            / funcname "(" comma-exp ")"
            / "new" classname "(" comma-exp ")"

call-exp = primary-exp *("." methodname "(" comma-exp ")")
unary-exp = "!" unary-exp
          / "-" unary-exp
          / "+" unary-exp
          / call-exp
mult-exp = unary-exp *(("*" / "/") unary-exp)
add-exp = mult-exp *(("+" / "-") mult-exp)
comparison-exp = add-exp *(("<" / ">" / "<=" / ">=" / "==" / "!=") add-exp)
and-exp = comparison-exp *(("&&") comparison-exp)
or-exp = and-exp *(("||") and-exp)
exp = or-exp

vardec = "let" var ":" type
paramdec = var ":" type

comma-vardec = [vardec *("," vardec)]
comma-paramdec = [paramdec *("," paramdec)]

stmt = exp ";"
     / vardec ";"
     / var "=" exp ";"
     / "let" var ":" type "=" exp ";"
     / "while" "(" exp ")" stmt
     / "break" ";"
     / "return" [exp] ";"
     / "if" "(" exp ")" stmt *("else" stmt)
     / "{" *stmt "}"

funcdef = "fun" funcname "(" comma-paramdec ")" "->" type "{" *stmt "}"
methoddef = "meth" methodname "(" comma-paramdec ")" "->" type "{" *stmt "}"
constructor = "init" "(" comma-paramdec ")" "{" *("super" "(" comma-exp ")" ";") *stmt "}"
classdef = "class" classname ["extends" classname] "{" *(vardec ";") constructor *methoddef "}"
program = *(classdef / funcdef) 1*stmt
```

## License

MIT
