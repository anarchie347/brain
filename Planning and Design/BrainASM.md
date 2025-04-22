# BrainASM

The next level up from Brain3U

BrainASM is intended to expose a language similar to a simplified assembly instruction

It will segment the memory exposed by Brain3U into blocks of 5 cells: 'Stack' (S), 'Heap' (H), 'Flag' (F), 'Working' (W), 'Move' (M). The exposed memory interface is only the S,H memory. W memory is used for the operations, and M memory is used for manipulating the pointer to move it around memory (moving to known locations, following pointers etc)

The stack/heap division is not exacty the same as in a standard programming language. In BrainASM the divide is:
- Stack: Memory management is done during compilation, addresses arer worked out during compilation (in a higher level language) and are hardcoded by the time the compiler reaches BrainASM. The stack is only accessible by hardcoded accesses known at compile time.
- Heap: Memory management is done during runtime. BrainASM is not memory safe and assumes all heap references are valid. BrainASM exposes 'malloc', 'gccoll' and 'free'. malloc to allocate heap, gccoll to free all unused memory, free to free a specific block. These operations are implemented by the BrainASM compiler as implementing higher up would cause (even more) astronomical performance impact. The heap is only accessible by pointers stored in memory.

The F memory is used to for flags, which are useful for memory traversal. The 0th block will have a certain F value to indicate it is the first block, so the pointer can find the start from anywhere. When loading a value from a pointer, the implementation will: follow the pointer, then change the F value to something (e.g. 5). Now to access this location (which is repeatedly needed for a move/copy), a simple search for 5 in F cells will get there a lot faster than following the pointer. When not in use for quick move flags, F primarily stored metadata about the H cell in that block (free, first allocated, last allocated)

BrainASM code follows a similar style to low-level invocation of system calls on a standard operating system.

First, values (arguments) are loaded (copied) into the first few W cells, in an order defined by the operation. 
Then the required operation is invoked, which operates on the arguments (which it knows the locations of as they are pre-determined), and leaves the result(s) in pre-determined locations at the start of the W cells.
Operations can also take arguements known at compile time, in which case this will be compiled directly into the code of the instruction.

Because arguement positions and count are constant and are defined in the language defintion, this allows the operation translations to freely use all W memory after the last argument cell

Note: unlike syscalls, it is not necessary to store a code to represent the required operation as the operation is part of the code.

BrainASM also includes 2 types. These are booleans and integers. booleans are either a 0 or a 1. Treating a different value as a boolean could result in undefined behaviour. Integers are 32-bit unsigned integers. As booleans are just 0 or 1 they can be used as integers without issue. There is no kind of type checking or polymorphism. The function signature just states what the function will try to parse the data as. Ignoring the required type could lead to undefined behaviour

In the following explanations, named arguments are hardcoded and known at compile time, arguments such as {0} indicate the value in W cell 0, which will not be known at compile time. Things after the '->' in the definition are return values

The special instructions related to arg loading in BrainASM:
- SLOAD x y: Loads (copy) a value from a hardcoded S address x into hardcoded W cell y. Used for copying stack values into arguements
- SMOVE x y: Moves a value from hardcoded W cell x to hardcoded W cell y. Useful as a faster option to relocate results from operations to be used as args for next ones
- SSTOR x y: Stores (no copy) a value from hardcoded W cell x to hardcoded S address y. Used for extracting results from function calls to the stack
- PLOAD x {0}: Follows the pointer {0} (H memory), and loads the value there into hardcoded W cell x
- PSTOR x {0}: Follows the pointer {0} (H memory), and stores the value from hardcoded W cell x into it


Note: S stands for Stack, P stands for Pointer

The memory management instructions:
- MALLOC {0} -> {0}: allocates a block in heap size {0}, stores pointer to it in {0}
- GCCOLL: Garbage collects all of memory
- MFREEP {0}: Frees the block of memory at {0}
- FALLOC x : Fixed malloc, takes a hardcoded size, more efficient. Useful for when initial size is known, but the data cannot be on the stack as the size may change

Conditional instructions:
- IF {0} {...}: takes a boolean in {0} and executes the block if it is true.
- WHILE {0} {...}: takes a boolean in {0} and executes the block while it is true. (Pre-condition loop)


BrainASM then supports the following (may be extended) operations:
- IADD {0} {1} -> {0} {1}: stores ({0} + {1}) to {0}. Stores True to {1} if overflows
- ISUB {0} {1} -> {0} {1}: stores ({0} - {1}) to {0}. Stores True to {1} if underflows
- IMLT {0} {1} -> {0} {1}: stores ({0} * {1}) to {0}. Stores True to {1} if overflows
- IDIV {0} {1} -> {0} {1}: stores (Floor({0} / {1})) to {0}. Stores remainder ({0} % {1}) to {1}
- IEQU {0} {1} -> {0}: stores boolean of ({0} == {1}) to {0}
- ILES {0} {1} -> {0}: stores boolean of ({0} < {1}) to {0}
- IGRE {0} {1} -> {0}: stores boolean of ({0} > {1}) to {0}
- BAND {0} {1} -> {0}: stores boolean of {0} AND {1} to {0}. Requires {0}, {1} to be boolean
- BORR {0} {1} -> {0}: stores boolean of {0} OR {1} to {0}. Requires {0}, {1} to be boolean
- BXOR {0} {1} -> {0}: stores boolean of {0} XOR {1} to {0}. Requires {0}, {1} to be boolean
- BNOT {0} -> {0}: stores boolean of NOT {0} to {0}. Requires {0}
- BNAN {0} {1} -> {0}: stores boolean of {0} NAND {1} to {0}. Requires {0}, {1} to be boolean
- BNOR {0} {1} -> {0}: stores boolean of {0} NOR {1} to {0}. Requires {0}, {1} to be boolean
- BXNR {0} {1} -> {0}: stores boolean of {0} XNOR {1} to {0}. Requires {0}, {1} to be boolean

Note: Operations beginning with I take two integers, operations beginning with B take two booleans