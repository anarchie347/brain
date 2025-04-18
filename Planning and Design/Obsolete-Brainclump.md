# Brainclump

Brainclump adds the following additions:

- If statements
- Move Left/Right by a value stored in a cell (hop)
- Copy

## How

BrainClump segments memory into 5 types:

- D (Data)
- W (Working 1)
- X (Working 2)

Therefore a memory tape, insteead of being treated as 30,000 (or more) data cells, it instead looks like (pipe characters are not memory cells, they are just used to indicate the groupings):

D W X | D W X  | D W X 

These can be though of as 3 separate memories which can be moved between using the brainfuck instructions `<` or `>`

Brainclump  code cannot move between the different types, they are used by the compiler to implement the features of brainclump. Because of this, the pointer will be pointing at a D cell for the start of every Brainclump instruction

### If statements

Brainclump has if statements. An if statements follows the following syntax: `I(x)(y)`, where `x` is the Brainclump code to be executed if the current cell is 0, and `y` is the Brainclump code to execute if it is non-zero. 

If statements are implemented using the following Brainsub algorithm:
Since the algorithm will only be executed when pointing at a D cell, D is used to refer to the value being checked
```
>Z /set W = 0
>Z- /set X = 255
<< /go back to D
[ /enter if D != 0
    {1} /execute D != 0 code
    > /go to W which is 0, so loop exits
]
/if D == 0, pointer at D, else pointer at W which is 0
>+ /if D == 0, pointer at  W which is 1, else pointer at X which is 0
[ /enter if D == 0
    {0} /execite D == 0 code
    >> / go to X which is 0, so loop exits
]
/ pointer at X
<< /move pointer back to D

```

### Hop

Hop has the following syntax: `L` or `R` to either hop left or right respectively. The pointer will move left or right by the amount stored in the current cell.

Left Hop is implemented by the following Brainsub algorithm:
```
M(_1)> /move count into W, then move pointer to W
[
    M(100_) L(3) /move count into previous W, then move pointer to previous W
    - /decrease count
] /repeat until the count is 0
< /move back to D
```
The algorithm can be adapted by changing `M(100_) L(3)` to `M(_001) R(4)` if hopping right

### Copy

Copy acts very similar to [Brainsub](./Brainsub.md)'s `M` operation, except that it copies rather than moves. This is a feature of Brainclump rather than Brainsub because it requires extra memory. The algorithm is as follows:

Modify the argument to account for the fact that only every 3rd cell stores data, so `1_01` goes to `100 _ 000 001` (spaces for illustration purposes only)
Modify the arguemtn so the value is also copied to the W cell after the D cell, so `100 _ 000 001` from previous step goes to `100 _ 100 001`
Call `M` with this modified argument
Call `M(1_)` to move the value from the W to D
Now the value has been copied