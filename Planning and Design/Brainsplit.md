# BrainSplit

Brainsplit takes the single memory tape of brainfuck and splits it into 2 separate memory tapes, a data memory and a working memory
This is achieved by making every other cell a working memory cell, so to traverse to the next memory cell of the same type, the brainfuck instruction `>>` is used. to swap memories, the instruction `>` or `<` are used.

Brainsplit exposes these memories using the following rules:
`>`, `<` in brainsplit are translated to `>>` `<<` or [Brainsub](./Brainsub.md), to traverse to the adjacent cell of the same type
`^` translates to `>` to swap to working memory
`_` translates to `<` to swap to data memory

`^`, `_` can also be used to go the other way, but this would result in the pointer ending up one cell out of place.

Memory is split as shown below:

D | W | D | W | D | W

Brainsplit does not implement any use for the different memories, it simply probides them to be utilised by higher level languages.

Brainsplit allows the use of [Brainsub](./Brainsub.md) methods, and translates the aruements to work with the split memories. This affects the following methods:

- `L(x)` moves x places left within the same memory (in practice, this doubles the arguement)
- `R(x)` affected same as `L(x)`
- `M(x)` has the arguement adjested so it only affects the same type of memory (by insteing `0`s into the arguements). Brainsplit also adds the method overload `M(x)(y)(z)` whoich allows moving values across memory types. `x` is the pattern for data memory, `y` is the pattern for working memory, and `z` is a single character, either 'W' or 'D' indicating which memory the pointer is currently in.

It would be possible to design a M function that doest require an argument specifying the memory type, but this would be very inefficient on memory usage. It would be implemented by using a lower level language to add a register to store the state of which memory is pointed at. Because this register would have to be accessble from anywhere, this would require splitting memory in two again, as one memory would have to be used to traverse from an arbitray point on the tape to a given location where the state is stored. This would be very memory inefficient and slow, so a better solution is require the programmer to specify the current memory.