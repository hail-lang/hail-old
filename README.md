<p align="center">
    <img src="logo.svg" width="75%">
    <p align="center" style="font-style: italic;">A type-safe, multi-paradigm, high-speed general purpose programming language.</p>
</p>

> _**note:**_ i'm currently still working on the language design and semantics, which is why i restarted the project.  i didn't make much progress at first and i implemented it with a parser generator, which doesn't give me as much control as i like.  this is the last time i restart the project without warning.

# Table of Contents
- [Table of Contents](#table-of-contents)
- [The Language](#the-language)
- [Goals](#goals)

# The Language
Hail is a low-level, type safe programming language.  By default, Hail doesn't manage memory.

It's designed to compete with other lower-level languages, such as C/C++, Jai, Odin, Rust and Zig.

# Goals
These are the goals which I intend to reach at one point with Hail.

- Compile applications "blazingly fast," without sacrificing stability or efficiency of the application.
- Generate small, portable executables.
- Be simple enough that a single person can know the entire language syntax & semantics by memory (excluding the standard libraries, as documentation will always be readily available for those).