# New memory management

The memory model used will have many similarities to a usual stack and heap model.

Brain will be a garbage collected language

Variables will have to be declared, like in C#, but the programmer does not have to deal with allocation or freeing. Brain is statically typed

The following process happens in a high level intermediate language, after the code is flattened to one subroutine.

- Types are used to work out method and operation overloads. For example, `a + b` might go to `INT_ADD(a,b)` or `STRING_CONCAT(a,b)`.
- Variable declarations are converted to memory allocations, with size and memory type based on the data type.
- after the last use of each variable, a `DELETE_VAR(a)` statement is added to aid in memory allocation
- Refer to below this list for details on the types of memory: 'stack', 'heap'. Heap is further subdivided into 'lablled' and 'dynamic'
- sizing allocation for stack memory are known at compile time, so the addresses are decided by the compiler. all accesses to variables in known memory are translated into a certain amount of `>` to reach that address
- allocation for labelled memory is known at compile time, so each variable is assigned a number which is unique for that variables lifetime, but may be used by another variable later in the code. Accesses to these variables are replaced with hardcoded linear searches through 'flag' cells to find the correct memory
- nothing is known about variables in dynamic memory. Allocations and deletions are converted to declarations of pointer variables which are stored in whichever data structure is storing the data. Accesses to the dynamic data is replaced with `RETRIEVE_FROM_DYN(a)` method cals where `a` is the pointer.



# Memory segmentation

Memory on the brainfuck tape will be divided as follows:

Stack (S) | Heap (H) | Flag (F) | Working (W)

# Memory types

Stack memory is used for variables whiose allocation and size is fixed and known at compile time, such as integers or booleans. Variables on the stack are translated into addresses determined at compile time, and accesses are tranalted into hardcoded instructions to get to these addresses.

## Heap memory

Heap memory stores both labelled and dynamic memory. The F cell is used to distinguish them.

Knwon memory is memory where the allocation is known at compile time, but the size isnt, or the size changes. This includes lists, as well as arrays where the size is unkown at compile time. Each of these allocations is assigned an integer label at compile time. The label is stored in the F cell of the first block of memory for the allocation. The rest of the F cells are filled with a number indicating used labelled memory. Allocations of these are made by an allocator on the heap, and accesses are replaced with linear searches in F memory for the label. Because the label is known at compile time, this is very efficient as it can be coded into the brainfuck code.

Dynamic memory is where the allocation and size are not known at compile time. This includes things such as lists of strings. String sizes are not known at compile time, so this is converted to a list (stored in Known) of pointers to Dynamic, referring to the strings. Accesses Dynamic is really slow because in brainfuck, the algorithm to 'move x cells to the right' is O(n^2) if x is not known at compile time. D memory uses sequential addresses rather than labels like label memory, because a linear search for an unkown value is even slower.
For D memory, the F cell is set to a number indicating 'used D memory', and the length of the allocation is stored in the first H cell.

