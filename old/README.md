<p align="center">
    <img src="logo.svg" width="75%">
    <p align="center" style="font-style: italic;">A type-safe, high speed programming language for scalable systems!  (featuring a cheesy logo!)</p>
</p>

> OLD IMPLEMENTATION

> _**note:**_ the compiler is unfinished and probably buggy.  this means that until the language reaches a more stable state (i.e. the beta stage), the language may introduce many breaking changes between versions.  in the future, it will be our goal to introduce as few breaking features as possible, to ensure backwards compatibility.

# Table of Contents
- [Table of Contents](#table-of-contents)
- [The Language](#the-language)
- [Goals](#goals)

# The Language
Hail is a low-level, type-safe programming language.  By default, Hail doesn't manage memory.

It's designed to compete with other lower-level languages, such as C/C++, Jai, Odin, Rust and Zig.

```hail
import { println } from io

main :: routine() {
    println("Hello, world!")
}
```

# Goals
These are the goals which I intend to reach at one point with Hail.

- Compile applications "blazingly fast," without sacrificing stability or efficiency of the application.
- Generate small, portable executables.
- Be simple enough that a single person can know the entire language syntax & semantics by memory (excluding the standard libraries, as documentation will always be readily available for those).