# Methods

Methods in Brain will all be substituted into the main program at compile time. This will be dealt with by one of the high level intermediate languagues. FOr most cases, this is a trivial substitution and renaming of variables so they dont overlap. FOr recursive methods, this cannot be done, see below

# Recursion

Recursion will be implemented in Brain by one of the higher level intermediate languages converting any recursive code to non-recursive using an explicit stack.

Method substitution will be used until a method is reduced to only calling itself.
At this point, the compiler will convert the recursive function using the following algorithm:

- Declare a stack
- Loop until stack is empty
- Put code inside loop
- At any recursive calls, push all local variables to stack, push return address to stack and retart the methods code (see state tracking)

This algorithm is not rigorous and needs improving

# State tracking

at the start of the subroutine, a state variable (bool) is declared.
The code of the subroutine (including state declaration) is wrapped in a loop
As there is no equivalent of a `goto` statement to restart the subroutine, everywhere a recurisve call appears, all code after it in the subroutine is wrapped in an if statement, so only runs if state is true. THis is so that a 'go to start of methods' statement can be emulated by skipping all remaining code by setting state to false

# Example
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
   
    while stack not empty {
        let run = true
        if x == 0 {
            run = false
        }
        if run {
            let retVal = x
            stack.push(x)
            x = x - 1
            
        }
    }
}
```