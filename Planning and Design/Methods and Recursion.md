# Methods

Methods in Brain will all be substituted into the main program at compile time. This will be dealt with by one of the high level intermediate languagues. FOr most cases, this is a trivial substitution and renaming of variables so they dont overlap. FOr recursive methods, this cannot be done, see below.

Flattening of methods require that methods do not return anything. Hence as part of the substitution process, `return` statements will be converted to assignments to a `return_val` variable, and will use goto emulation (as explain in State tracking) to skip the rest of the subroutine after a return

# Recursion

Recursion will be implemented in Brain by one of the higher level intermediate languages converting any recursive code to non-recursive using an explicit stack.

Method substitution will be used until a method is reduced to only calling itself.
At this point, the compiler will convert the recursive function using the following algorithm:

- Declare a stack
- declare a 'stage' integer variable
- Loop until stack is empty
- Put code inside loop
- split code where any recursive calls occur, the recursive call is at the end of the block ,not the start of the new.
- encase each block with an `if stage <= X && !skip` statement, where X is the ordered index of the block (0 indexed)
- variable declarations made within the subroutine should be hoisted outside the loop
- replace the recursive call with:
    - Push all local vars to stack (not including stage and skip)
    - Push the length the stack pointer has been moved in this operation to the stack
    - Push the current value of stage to the stack
    - Set skip = true
    - Set stage to 0
- At the end of the subroutine (within final stage block), add the following code:
    - Stack pointer should be at the 'stage' from previous call
    - set stage = pop from stack + 1
    - use next value (length moved) to pop all variables from parent call off stack and into appropriate locations in variables
    - stack pointer should now be pointed at the parent parent call's stage
- At the end of the subroutine (outside final stage block, still inside loop), set skip to false

this algorithm may need adjusting so that the 'end of subroutine within final stage' is put before stage 0, in an `if returning` block, where returning is a variable set to true when a method returns (so at the end of the loop when skip is changed), so that on the next iteration, the variables can be pulled from the stack and loaded. It may be possible to base this on the value of the 'skip' variable

# State tracking

Flattening recurisve calls requires use of a `goto` statement. This is not possible, so 'stage' and 'skip' are used to emulate this.
stage indicates where in the subroutine to go to. All if statements use `<=` so that after the block jumped to is finished, the code continues. 'skip' is used for backward `goto` statements, and indicates that the stage has not yet been reached and the code must loop to get back to the destination. This prevents blocks from running after the emulated `goto` that wouldve otherwise run based on assuming the goto has already been jumped to and the code was just continuing.

When a recursive call is made, the current stage is pushed onto the stack so that when the call returns, it can jump to the correct place in the caller subroutine

# Memory
All variables from parent calls are stored on the stack, so that they can be restored when the child returns. The distance the stack pointer moved when putting all these variables on the stack is stored so that the parent knows how far to go down the stack to retrieve all variables.

# Example - not complete, uses an old uncomplete algorithm
The following pseduocode:
```
sub factorial(x) {
    if x == 0 {
        return 1
    }
    return x * factorial (x - 1)
}
```
is converted to the following (WIP):
```
sub factorial(x) {
    let callStack = new stack
    let stage = 0
    let skip = false
   
    while stack not empty { //this should be a post-condition loop
        if stage <= 0 && !skip {
            if x == 0 {
                run = false
            }
        }
        
        if run {
            let retVal = x
            stack.push(x)
            x = x - 1
            
        }
    }
}
```