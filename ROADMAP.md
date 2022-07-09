# Roadmap
This is a list of features we aim to implement.  This project uses the GitHub issue system as a todo list, and allows for discussion of features.

## Language design
### Declarations
- Routines (routines are functions in Hail)
- Structs
- Enums, with algebraic data types
- Traits
- Struct applications
```hail
apply MyTrait to MyStruct {
    // ...
}

apply MyStruct {
    new :: routine() -> MyStruct {
        return MyStruct::{
            // ...
        }
    }
}
```
- Import statements
```hail
import lib
import lib as lib2
import { MyStruct, my_routine } from lib
```

### Syntax
- Optional semicolons, using automatic semicolon insertion ([#3](https://github.com/hail-lang/hail/issues/3))
- `val` variables
```hail
val my_variable = 42
val my_variable <- uint32 = 42
val my_variable <- uint32
```

## Compiler
> TODO: list the compiler stuff.

## Diagnostics
- Pretty diagnostics using `codespan-reporting` ([testing #2](https://github.com/hail-lang/hail/issues/2))