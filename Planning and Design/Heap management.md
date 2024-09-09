# Heap management

The heap will be managed by an allocator and a garbage collector

# Allocator

The allocator is a set of subroutines to allocate heap space. It contains the following methods:

- `alloc(x)` - allocates a memory of size 'x'
- `realloc(x)(y)` - allocates a memory of size 'x' and moves all the data from pointer 'y' into it (intended for when a data struct grows). Tries to just extend 'y's existing allocation, moves to new area if not possible
- `free(x)` - frees an allocation aet pointer 'x'

Both alloc and realloc return a pointer to the newly allocated space

# Garbage collector

The garbage collector will operate based on tracing garbage collection
When it runs, the GC will go through all current stack locations storing pointers (this is known at compile time, so the stack locations to go to are hardcoded at compile time), and mark them (change F cell). If the heap allocation contains pointers to the heap, then these are also followed (can be worked out from type checking at compile time). The pointer will not be followed if it points to already marked memory, as this means there is a circular reference. After all pointers have been followed, the GC runs through all the heap, and whereever it finds unmarked allocaitons, it clears them (using its own algorithm rather than 'free' from the allocator for efficiency)

The GC may also implement mark and compact to reduce memory fragmentation, this is yet undecided. IT is also currently undecided what will trigger the GC running