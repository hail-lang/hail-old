<p align="center">
    <img src="logo.svg" width="50%">
    <p align="center"><i>A low-level programming language with high-level features (and a cheesy logo)!</i><p>
</p>

## Table of Contents
- [About](#about)
    - [Inspiration](#inspiration)
    - [Disclaimer](#disclaimer)
- [Examples](#examples)
- [Notable Features](#notable-features)

## About
Hail is a memory & type safe systems programming language.

### Inspiration
- C/C++: low-level & fast, syntax.
- JavaScript: import syntax.
- Odin & Jai: syntax.
- Rust: safety, enums.

### Disclaimer
> **note**: Hail as a language and compiler is VERY unfinished.  Everything is subject to *breaking change*.  Syntax, semantics, standard libraries, everything.  The features listed here and in the [Features](#features) section may not be in the final product of the language, and the [examples](https://github.com/hail-lang/hail/tree/main/examples) may not work in later versions of the language.

## Examples
```hail
import { println } from std_io

main :: routine() {
    println("Hello, world!")
}
```

If you want to see some more examples of Hail programs, check the [examples folder](https://github.com/hail-lang/hail/tree/main/examples).

## Notable Features
> Please see the above [disclaimer](#disclaimer).

- *Cool syntax*.
- Algebraic data types, like Rust enums.
- Generic types.
- Memory managed at compile time.
- Optional semicolons (finally, an actually decent compiled language with optional semicolons)!
- Values are immutable by default.