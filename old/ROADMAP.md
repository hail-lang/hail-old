# Roadmap
A list of features that have yet to be implemented for Hail.

## Compiler
- [x] Diagnostics
    - [x] make 'em pretty! ([#2](https://github.com/hail-lang/hail/issues/2))  thank you `codespan-reporting`
- [x] Lexer
    - [x] Whitespace and Line breaks
    - [x] Integer constants (as well as `0x0` and `0b0` constants)
    - [x] Float constants
    - [x] Identifier literals
    - [x] String literals
    - [x] Punctuation
    - [x] Groups (`()`, `[]` and `{}`)
    - [x] Comments
    - [x] Automatic semicolon insertion ([#3](https://github.com/hail-lang/hail/issues/3))
- [ ] Parser
    - [ ] Literals & constants
    - [ ] Paths & field accessing (`path1::path2`, `my_struct.my_field`)
    - [ ] Function calls
    - [ ] Basic operators
    - [ ] `as` expressions
    - [ ] Type annotations/declarations
    - [ ] `val` variable declarations
    - [ ] `if` statements
    - [ ] `while` statements
    - [ ] `test`/`match` statements (name undetermined at this time).
    - [ ] `return` statements
    - [ ] `continue` statements
    - [ ] `break` statements
    - [ ] `routine` declarations
    - [ ] `struct` declarations
    - [ ] `enum` declarations
    - [ ] `trait` declarations (??? undetermined whether or not this will be implemented).
    - [ ] `import` declarations.
- [ ] Everything else (**todo**: fill in the rest when we are later on in the compiler process)
