# Lists and arrays

Array: fixed size, known at compile time, allocated in known memory

Vec: fixed size, not known at compile time, allocated in labelled memory

List: variable size, allocated in labelled memory


All collections elements must be the same size. So an array of integers or list of booleans is fine. Lists of strings are not directly possible because strings can change size. Even if all the string sizes are known at compile time, this still cannot be done because they all must be the same size. Lists, vecs and arrays of these are accomplished using pointers to labelled or dyn memory, as pointers have a fixed size
All elements being fixed size means that to access element i, the pointer needs to go to the sart of the collection and traverse (i * elementSize) + 1 (the first cell stores length)

All accesses to collections are bounds checked at runtime just before the access is made

For lists, the process is slightly more complicated. To reduce unnessecary moves when resized, their memory allocation is extended. If they run out of continuous memory space, they will allocate more space further along the tape, so the memory is not all in one block. The length of each block will be stored, as well as the total length. This will make the access algorithm slightly more complex.

If a certain access index is known at compile time, this will be hardcoded into the resulting code, resulting in much faster allocations.