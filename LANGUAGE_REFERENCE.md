# Parlan Language Reference

Welcome to the official language reference for **Parlan**. Since Parlan is designed to teach how compilers work, its syntax is intentional, explicit, and easy to parse.

---

## 1. Program Structure
Every Parlan program is a collection of functions. The entry point of any executable must be the `main` function.

```
func main(): int {
    // Your code here
    return 0;
}
```

---

## 2. Variables and Data Types
Parlan is statically typed. at this moment it doesn't supports type infering (yet).
for now you need to specifie every single type, these are the basic types:

| Type    | Description                                    | Example            |
| :---    | :--- | :---                                    |
| `int`   | 64-bit Integer                                 | `42`               |
| `float` | Floating point number                          | `3.14`             |
| `string`| a string of characters (translated to `char*`) | `"hello world"`    |
| `bool`  | a boolean value (translated to `unsigned char`)| `true`             |
| `void`  | Used for functions that return nothing         | `func log(): void` |
| `vector`| a dynamic vector (uses a custom implementation)|

---

## 3. Functions
Functions are defined using the `func` keyword, followed by the name, parameters, and the return type after a colon `:`.

**Syntax:**
`func name(arg: type): return_type { ... }`

**Example:**
```
func add(a: int, b: int): int {
    return a + b
}
```

---

## 4. Control Flow
Parlan uses standard block-based logic to control the execution flow.

### If / Else If / Else
```
if x > 5 {
    // logic
} else if x > 2 {
    // logic
} else {
    // logic
}
```

---

## 5. Loops 
Parlan has also while loops 

```
while x > 0 {
    x = x - 1
}
```

## 6. Operators
Parlan have every common operator, like arithmetic operators, boolean operators, etc.

here are all of they:

| Operator | Description | Example  |
| :--- | :--- | :--- |
| `+` | adds two numeric values | `2 + 2` |
| `-` | substracts two numeric values or represents a negative number | `2 - 2` `-2` |
| `*` | multiplies two numeric values | `2 * 2` |
| `/` | divides two numeric values | `2 / 2` |
| `>` | compares two numeric values, returns true if the first one is bigger | `2 > 2` |
| `<` | compares two numeric values, returns true if the second one is bigger | `2 < 2` |
| `>=` | compares two numeric values, returns true if the first one is bigger or equal to the second one | `2 >= 2` |
| `<=` | compares two numeric values, returns true if the second one is bigger or equal to the first one | `2 <= 2>` |
| `==` | compares two numeric values, returs true if both are equal | `2 == 2` |
| `!=` | compares two numeric values, returns true if both are not equal | `2 != 2` |
| `and` | compares two boolean values, returns true if both are true | `true and true` |
| `or` | compares two boolean values, returns true if one or both are true | `false or true` |
| `not` | negates a boolean value | `not true` |

## 7. Dynamic Vectors
Parlan has vector instrinsics, that mean you can create, manipulate and delete vectors with these instrinsic functions:

| Functions | Description | Syntax |
| :--- | :--- | :--- |
| `__new_vector` | creates a new vector | `__new_vector([type])` |
| `__free_vector` | deletes a vector, always use it | `__free_vector([vector])` |
| `__push_vector` | pushes a new element of the same type with which the vector was initialized | `__push_vector([vector],[element])` |
| `__get_vector` | return the element at the provided index, panics if the index is out of range | `__get_vector([vector],[index])` |

>[!NOTE]
> these functions may disappear in future versions

## 8. Direct C Integration (`c_code`)
Since Parlan compiles to C, you can inject raw C code directly into your Parlan source. This is useful for calling standard library functions like `printf`.

```
c_code { printf("Value: %d\n", usr_x); }
```
> **How it works:** The Parlan compiler takes the string inside the double quotes `""` and places it directly into the generated `out.c` file during the Codegen phase.

>[!NOTE]
> you need to be carefull with identifiers, because the compiler adds `usr_` before every one of them. if you forgot to add the `usr_` sufix, your program will not compile! (well, clang/gcc will not compile the output)

---

## 9. Comments
Documentation is key in an educational language.
* **Single-line comments:** Use `//` to ignore text until the end of the line.

```
// This is a comment and will be ignored by the Lexer
```

* **Multi-line comments:** use `/*` to ignore text until `*/`

```
/*
    this 
    is
    a 
    long
    comment
*/
```

---

## 10. From Source to Executable (The Pipeline)
To understand how your code is processed, you can visualize the flow:

1.  **Lexer (`lexer.rs`):** Breaks your text into tokens like `FUNC`, `IDENT(main)`, `LBRACE`.
2.  **Parser (`parser.rs`):** Organizes tokens into an **AST** (Abstract Syntax Tree).
3.  **Backend (`backend.rs`):** Converts the AST into valid C code.
4.  **Clang/GCC:** The final step that turns the C code into a machine-readable binary.

## Compiler Flags
the parlan compiler has several flags, these are:

| Flag | Description |
| :--- | :--- |
| `--time` | shows compile time information from lexer to backend with a little debug information |
| `--debug` | shows the entire process, printing the token stream and the ast |
| `--compile` | calls Clang or GCC to compile the output file |
| `--gcc` | tells the compiler to use GCC instead of Clang to compile output |
| `-o` | tells the compiler that the next argument is the name of the output file (if not passed, uses 'out.c')