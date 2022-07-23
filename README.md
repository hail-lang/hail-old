<p align="center"><img src="logo.svg" align="center"/></p>
<p align="center"><i>hail is a cool, fast programming language that doesn't suck</i></p>

# table of contents
- [goals](#goals)
- [comparisons](#comparisons)
    - [rust](#rust)
    - [c](#c)
    - [c++](#c-1)
    - [javascript](#javascript)
- [examples](#examples)
    - [hello, world!](#hello-world)
    - [vecs](#vecs)
- [building](#building)
    - [on windows](#on-windows)
    - [on others](#on-others)

# goals
- *Be all in one*: hail should be a compiler, build tool and package manager, all in one.
- *Be fast*: hail should take a source module and compile it quickly.
- *Be safe (but not too safe!)*: hail code should be safe enough to ensure the code works (in the majority of cases), but not so safe that it makes the language harder.
- *Be good looking*: it should make you feel like a pro hacker without being hard to learn or just generally bad.
- *Be consistent*: there should only be one way to do something, and that one way should be fast and reliable.

# comparisons

## rust
Rust and hail are inherently similar, in that they are both systems programming languages, but by design, they are quite different.

Rust focuses on assuming the user doesn't know what they're doing; that isn't inherently bad, BUT it makes a lot of programs that should be relatively simple *more* complex, just because it doesn't trust the user.

hail trusts the user enough that those simple programs stay simple; but it's still safe enough to prevent common mistakes from being shipped, without any runtime overhead.

This has to do with Rust's *compile time memory management*.  This makes sure you almost never have to touch memory when working with Rust, BUT that also means that working with memory and pointers/references is more difficult; as everything is memory managed at compile time.  In hail, memory is completely manually managed by the user.

Rust also has generic types, whereas hail does not.  hail also has algebraic data types like Rust's `enum` declarations:

```rust
enum MyEnum {
    MyMember(u32),
}

fn main() {
    let my_member = MyEnum::MyMember(42);
    match my_member {
        MyEnum::MyMember(value) => {
            println!("{}", value);
        }
    }
    // => 42
}
```

```hail
import { println } from io

val MyEnum = enum {
    MyMember <- uint32,
}

val my_member <- MyEnum = MyEnum::MyMember::(42)
match my_member {
    value <- MyEnum::MyMember => {
        println(uint32::as_str(&value))
    }
}
// => 42
```

## c
C and hail are also inherently similar, in different ways than Rust.

They are both systems languages that assumes the user knows what they're doing; but C assumes the user is right almost 100% of the time.  Otherwise, Hail is very similar to C, and actually aims to replace it at some point.

For example; consider the following C code:

```c
#include <stdio.h>

int get_number() {
    printf("Hello, world!");

    // never return an `int`
    // compiles just fine
}
```

And in hail:

```hail
import { println } from io

val get_number = routine() -> int {
    printf("Hello, world!")

    // never return an `int`
    // COMPILER ERROR HERE, this program doesn't compile.
}
```

## c++
hail and C++ are a little different; as C++ is object oriented, while hail is not.

C++ also has `template`s, which are like Rust's generic types; which hail has no equivalent of.

## javascript
JavaScript and hail are VERY different, JavaScript is garbage collected, dynamically typed and interpreted (high level), and hail has no memory management, it is statically typed and compiled (low level).

# examples

## hello, world!
```hail
import { println } from io

println("Hello, world!")
```

## vecs
> *`Vec`s/vectors are dynamically sized arrays.*

```hail
import { println } from io
import { UintVec } from uint

val my_vec = fluid UintVec::new()
UintVec::push(&fluid my_vec, 42)
UintVec::push(&fluid my_vec, 64)
UintVec::push(&fluid my_vec, 86)

val i = fluid 0
while i < UintVec::len(&my_vec) {
    println(UintVec::get(&my_vec, i))

    i += 1
}
// => 42
// => 64
// => 86

UintVec::drop(my_vec) // free the memory used by the Vec.
```

# building

## on windows
Building on Windows requires Mingw or an equivalent.  A build script for Windows is included, it can be ran with the following command:

```batch
build
```

## on others
There's no build scripts for other platforms yet; as I only use Windows.  In the future I will make build scripts for Linux/MacOS.

For now, just link everything in `src` together with GCC/Clang with a command like this:

```sh
gcc -Iinclude src/main.c ... -o hail
```