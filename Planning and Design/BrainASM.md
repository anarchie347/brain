# BrainASM

The next level up from Brain3U

BrainASM is intended to expose a language similar to a simplified assembly instruction

It will segment the memory exposed by Brain3U into blocks of 3 cells: 'Data' (D), 'Working' (W), 'Move' (M). The exposed memory interface is only the D memory

BrainASM code follows a similar style to low-level invocation of system calls on a standard operating system.

First, values (arguments) are loaded (copied) into the first few W cells, in an order defined by the operation. 
Then the required operation is invoked, which operates on the arguments (which it knows the locations of as they are pre-determined), and leaves the result(s) in pre-determined locations at the start of the W cells.
Operations can also take arguements known at compile time, in which case this will be compiled irectly into the code of the instruction.

Note: unlike syscalls, it is not necessary to store a code to represent the required operation as the operation is part of the code.

BrainASM also includes 2 types. These are booleans and integers. booleans are either a 0 or a 1. Treating a different value as a boolean could result in undefined behaviour. Integers are 32-bit unsigned integers. As booleans are just 0 or 1 they can be used as integers without issue. There is no kind of type checking or polymorphism. The function signature just states what the function will try to parse the data as. Ignoring the required type could lead to undefined behaviour

In the following explanations, named arguments are hardcoded and known at compile time, arguments such as {0} indicate the value in W cell 0, which will not be known at compile time. Things after the '->' in the definition are return values

The special instructions related to arg loading in BrainASM:
- HLOAD x y: Loads (copy) a value from a hardcoded D address x into hardcoded W cell y. As both of these numbers are hardcoded, this is much faster than moving the pointer an amount not known at compile time
- HMOVE x y: Moves a value from hardcoded W cell x to hardcoded W cell y. Useful as a faster option to relocate results from operations to be used as args for next ones
- HSTOR x y: Stores (no copy) a value from hardcoded W cell x to hardcoded D address y. USed for extracting results from function calls
- PLOAD x {0}: Follows the pointer {0}, and loads the value there into hardcoded W cell x
- PSTOR x {0}: Follows the pointer {0}, and stores the value from hardcoded W cell x into it


Note: H stands for Hardcoded, P stands for Pointer


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