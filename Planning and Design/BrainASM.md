# BrainASM

The next level up from Brain3U

BrainASM is intended to expose a language similar to a simplified assembly instruction

It will segment the memory exposed by Brain3U into blocks of 3 cells: 'Data' (D), 'Working' (W), 'Move' (M). The exposed memory interface is only the D memory

BrainASM code follows a similar style to low-level invocation of system calls on a standard operating system.

First, values (arguments) are loaded (copied) into the first few W cells, in an order defined by the operation
Then the required operation is invoked, which operates on the arguments (which it knows the locations of as they are pre-determined), and leaves the result(s) in pre-determined locations at the start of the W cells.
Operations can also take arguements known at compile time, in which case this will be compiled irectly into the code of the instruction.

Note: unlike syscalls, it is not necessary to store a code to represent the required operation as the operation is part of the code.

In the following explanations, named arguments are hardcoded and known at compile time, arguments such as {0} indicate the value in W cell 0, which will not be known at compile time

The special instructions in BrainASM:
- HLOAD x y: Loads (copy) a value from a hardcoded D address x into hardcoded W cell y. As both of these numbers are hardcoded, this is much faster than moving the pointer an amount not known at compile time
- HMOVE x y: Moves a value from hardcoded W cell x to hardcoded W cell y. Useful as a faster option to relocate results from operations to be used as args for next ones
- HSTOR x y: Stores (no copy) a value from hardcoded W cell x to hardcoded D address y. USed for extracting results from function calls
- PLOAD x {0}: Follows the pointer {0}, and loads the value there into hardcoded W cell x
- PSTOR x {0}: Follows the pointer {0}, and stores the value from hardcoded W cell x into it


Note: H stands for Hardcoded, P stands for Pointer