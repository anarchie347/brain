# Methods

Methods in Brain will all be substituted into the main program at compile time. This will be dealt with by one of the high level intermediate languagues. FOr most cases, this is a trivial substitution and renaming of variables so they dont overlap. FOr recursive methods, this cannot be done, see below

# Recursion

Recursion will be implemented in Brain by one of the higher level intermediate languages converting any recursive code to non-recursive using an explicit stack.

Method substitution will be used until a method is reduced to only calling itself.
At this point, the compiler will convert the recursive function using the following algorithm:

Declare a stack
Loop until stack is empty
Put code inside loop
At any recursive calls, push all local variables to stack, push return address to stack and start over at the loop again

Tis algorithm is not rigorous and needs improving