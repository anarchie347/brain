# Memory V3

Brain will use a memory model similar to a normal stack and heap.
Because all code is converted to be one subroutine, the stack memory does not need to behave like a stack, and instead acts like a normal block of memory.

Variables stored on the stack are a fixed size that is known at compile time. All memory allocation on the stack is determined at compile time.
All other data is stored on the heap, with a pointer from the stack. The first cell in a heap allocation contains the length of the allocation.

The brainfuck tape is divided as follows:

> Stack (S) | Heap (H) | Flag (F) | Working (W)

What is shown above is one 'block'. One cell is any of the individual S,H,F,W

Flag will be used to store metadata about a block. Mostly relating to its H cell.

The F will represent one of the following values

- First block of tape
- H First block of allocation
- H Part of allocated memory
- H free

Because the 'first block' state is stored in F, the first block cannot store data about the heap, so H will be unused for the first block. The 'first of tape' is required so that the pointer can be moved to the start of the tape from any unknown position

Memory adresses for S are computed at compile time, so accesses to these are substituted with traversing the pointer to the start of the tape, then a hardcoded amount of `>` to get to the cell.

Allocations to the heap return a pointer that is stored on the stack at a known location. Accesses to heap data are made by traversing to the S cell for the pointer, then following the pointer.

Pointers can exist to either type of memory, but the typ eof memory for the pointer does not need to be stored because types are checked and computed at compile time.