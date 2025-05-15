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
## Design Rationale
We chose Rust as the implementation language for our compiler for educational purposes. None of us had extensive experience with Rust beforehand, and we saw this project as an opportunity to explore its unique features, though it did cause a lot more headaches than using other languages. Rust being a mildly popular language for memory safety gave it a nice idea for building something complex and low-level like a compiler. We realized that writing a compiler in Rust would not be the past of least resistance, but the challenge aligned with our goals of developing a deeper understanding of both the language and low-level systems programming.

We selected JavaScript as the source (input) language for our compiler because of its dynamically typed structure, which posed interesting challenges for static analysis and code transformation, more on that later. JavaScript is a language we're all familiar with, which allowed us to focus on the core compiler logic without needing to constantly reference obscure language behaviors like what Rust has. Its flexible syntax helped show that we were in for a bumpy ride, but an educational one filled with lots of dreadful lessons learned.

## Limitations
As you can see, when you compile the language, it requires the function type to be specified as such: -> DataType. We unfortunately didn't have enough time to implement all aspects of the compiler, specifically the type checker, so the language has to be typed since type inference is not supported, and developers must explicitly annotate all function return types. Not the end of the world.

## Lessons Learned
-Rust. Definitely not using Rust again if we were to create another compiler.
-Testing while developing helped fix mistakes and avoid huge scope creep.
-Having our targeted language be a statically-typed one may have been easier, but overall JavaScript wasn't bad at all.
-Communication was key, as figuring out who developed what had to be well defined, as misunderstanding from miscommuncation did come up.

## License

MIT
