# Brain Compiler Overview

In order to compile high level Brain code into brainfuck, the code will be translated through *many* intermediate language. In this file is a list of the intermediate languages in order from lowest level (brainfuck) to the highest level (brain). This list will continually change, expand and shrink as i work things out. Design and development will begin from the lowest level language and eventually work up to Brain.
See individual files for details about each intermediate language.

All languages support comments using a single `/` character. Everything after a `/` on a l;ine will be ignored.
All languages have subroutines(*) that can be called and have arguements passed. They are given in a series of parentheses `()` after the instrution, e.g. `A(x)(y)(z)` means execute the `A` function with `x`, `y`, `z` as the arguments. Where the arguements are used by the subroutine implementation, braces `{}` are used with 0-based indexing to indicate which arguement should be inserted there, so in this example, `{0}` would refer to `x`, `{1}` refers to y and so on.

# Intermediate Languages

1. Brainfuck
2. Brainsub
3. Brainsplit

...

Brain