# Memory management

Lots of this is very similar to a standard implementation of memory management using a heap and a stack

Brain will be a garbage-collected language.
Variables will have to decalred, but the programmer does not have to deal with memory allocation or freeing.

The following process happens in a high level intermediate language, but after the code has all been flattened to one subroutine.

- All variables will be converted to using 32bit integers
- Variable declarations will be converted to allocate statements specifying an amount of memory, and deallocate statements inserted after the last use of the variable
- All allocation with sizes known at compile time are then assigned addresses in 'stack' memory
- All allocations with unkown sizes are assigned a single 32bit allocation in 'stack' memory to store a pointer to 'heap' memory
- read/write instructions for 'stack' allocations are substituted with the memory address
- read/write instructions for 'heap' allocations are substituted for a method to get the pointer from 'stack' memory and use this to access 'heap' memory
- Allocate and de-allocate statements are removed as they are only necessary for firguring out memory mappings


'heap' memory contains data whose size is not known at compile time

'stack' memory contains data whose size is knwon at compile time. 'stack' memory will not need to function as a stack because all code is converted to one method, so no call stack is required. The name is purely from how memory is managed by most compilers

Memory will be segmented as follows:

Stack | Heap | Traverse | Working

Traverse will be used to align the pointer to the start of the tape to then naviagate to addresses on the stack or heap.
It will consist of all '1's, expect the first celll which will be a 0
To traverse to the beginning of the tap, the pointer should be moves into any traverse cell, then the following brainfuck code should be run: `-<[<]`. This sets the current T cell to 0, then keeps going left until another 0 is found (the start). To get back to the previous location, run the following brainfuck code from the first T cell: `>[>]+` which moves right until it finds another 0 (the one set when beggining the move to cell 0), and sets it back to one so the algorithm can be used again.

The segmenting of memory like this and the use of the traverse cells will be translated in a lower level language than the memory allocations.
The memory allocation algorithm will convert variables into code that looks like `READ_STACK(523)`, `READ_HEAP(READ_STACK(24))`, `WRITE_STACK(12)(63)`.