# The Parlan Programming Language

Parlan is an open-source programming language designed to demonstrate how compilers work. 

##  Overview
Parlan was created as an educational tool to help beginners understand the inner workings of a compiler. It focuses on clarity and simplicity, making it easy to follow the journey from source code to an executable.

###  Internal Pipeline
The compiler follows a traditional three-stage architecture:
1. **Lexical Analysis:** Scans source text and converts it into a stream of tokens.
2. **Parsing:** Generates an **Abstract Syntax Tree (AST)** from the tokens.
3. **Codegen:** Generates **C code** from the AST.

> [!TIP]
> If you want to learn how a compiler is implemented, feel free to read the comments in every source file—they are designed to guide you through the logic!

##  Directory Tree
```text
root
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   README.md
│
├───src (Compiler Source Code)
│       ast.rs      - Definition of the Abstract Syntax Tree
│       backend.rs  - C code generation logic
│       lexer.rs    - Tokenizer / Lexical analysis
│       main.rs     - Entry point
│       parser.rs   - Syntax analysis
│
└───tests (Performance & Validation)
        main.par    - Sample Parlan source file
        out.c       - Generated C output
        perf_test.py - Performance benchmarking script
```
## Getting Started
To try out Parlan, follow these steps:

1. Clone the repository and navigate into it:

``` Bash
git clone https://github.com/Santiago-Lopez-25/parlan.git
cd parlan
```
2. Create a source file:
        Create a file (e.g., hello.par) and add some Parlan code.

3. Run the compiler:
        Execute the following command in your terminal:

```Bash
cargo run -- [file_name] --time --compile
```
*The --time flag will display the execution time and debug information to show you exactly how the compiler processes your code.*
*The --compile flag will compile the output file with Clang (if you use GCC use --gcc flag)*

## Code Example

Here is how a simple program looks in Parlan:

```
// simple 'hello world' example
func main(): int {
        c_code ""printf("hello world");""
        return 0;
}
```

## Parlan Language Reference

if you want to learn how the parlan language work and how to write on it, read the [Parlan Language Reference](./LANGUAGE_REFERENCE.md)

## Documentation
in every source file of this proyect i tried to add as many documentation comments as posible. i hope they help you to understand every step

>[!NOTE]
> my english is not the best, so the comments may have several typos. feel free to send a pull request fixing them!
