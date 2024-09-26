# Brain32

possible replacement for Brainsplit and Braincds

Brain32 segments memory as the following:

W D D D D | W D D D D | W D D D D

W - Working
D - data

The D cells are grouped in 4s and act as a single, 32 bit cell.

The W cells are used for the calculations to link these cells together

Brain32 exposes two memories: D (32bit), W(8bit)

This is to allow more efficient memory usage for grouping cells get 32bit cells

`+-` instructions are converted to work with the full 4 cells of data

32 bit nums cannot be moved into memory directly, they must be moved 8 bits at a time and delat with individually

The next language up will implement functions that use working and will not expose W to higher languages, so higher languages do not have to deal with W cells not being 32bit



There are 3 modes to write brain32 code:
- D mode
- W mode
- R mode

This determines which type of memory is being accessed

Brain32 code is not meant to be human-written, so its compiler will not contain a complete parser, and will expect an array, where each element is some brain32 code, and a value to state the mode. So there is no written way to set the mode

The pointer default position within a memory block is the rightmost D cell, so the entire source code is wrapped with `>>>>` and `<<<<` to get to this location

In terms of compilation to a lower language, this means:

### D mode:
`+` `-` `[` `]` all work based on the 32bit number stored

`,` `.` only operate on the first 8bits

`<` `>` are mapped to `<<<<<` `>>>>>`

`M(x)` and `C(x)` are operatiojns that move or copy respectively the cell in position x (0, 1, 2, 3) into the W cell for that block

Except mid-operation, the pointer is at the rightmost, least significant cell of the 4 D cells

### W mode:
`<` `>` are mapped to `<<<<<` `>>>>>`

All other operations work as they usually do in brainfuck, on the 8bit cell

At the start and end of the block, `<<<<` and `>>>>` must be added respectively to move in and out of W memory

`M(x)` and `C(x)` are operatiojns that move or copy respectively the W cell into the cell in position x (0, 1, 2, 3)

The pointer is pointing at a W cell

### R mode
R mode is raw mode, so R code is substituted directly into the translated brainfuck code.
The pointer will start at the least significant D cell, and should end in this cell. If the pointer does not end in this cell, the behaviour of the rest of the code is undefined.
Brain32 is intended to always be compiler generated from a previous translation, so there should not be errors here. FOr debugging purposes during development, the compiler will have an option to insert a debug statement into the resulting brainfuck stating the pointer position before and after the raw code to ensure it always points to a least significant D cell at the end of the block


# Translation

The following translations are not entirely accurate to the actual implementation as issues were realised with these algorithms during implementation. Refer to source code for the algorithms

## cell descriptions:

Wx refers to a working cell, where x is the index. The current cell block is index 0

Dx refers to a data cell in the current block as shown below:

W0 | D3 | D2 | D1 | D0
## shorthand

The following defines subroutines used cin these defintions that will be substituted in when writing the compiler:

- Ia(x)(y):

I is the If 0 function

If `a` is present, it is hard coded and refers to the starting cell. If `a` is not present, refer to I(x)(y). In the code below, `W` and `D` refer to 'move pointer to W' or 'move pointer to D', which can be hardcoded from what 'a' is

`x` is the 'Do if the cell is 0` code that is exdecuted at the cell that I is called at

`y` is 'if not 0' code

```
[
    W+
    D-
] move to W cell
I(DxW)(DyW) run regualar if but execute args at data cell
```

- I(x)(y)

refer to 'Ia(x)(y)' for explanation of args. This code executes on W0
```
>>>>>[-] /set next W1 to 0
>>>>>[-]-  /set W2 to 255
<<<<< <<<<< /move to W0
[
    y /run else code at W0
    >>>>> /move to W1 which is 0
]
/if val = 0, pointer at W0, else pointer at W1
>>>>> /if val = 0, pointer at W1 (0), else pointer at W2 (255)
+ /if val = 0, pointer at W1 (1), else pointer at W2 (0)
[
    <<<<<
    x / execute code at W0
    >>>>> >>>>> + resets W2 from 255 to 0 and exits on W2
] /exits on W2
<<<<< <<<<< /move back to W0 to exit
```

## D mode
THis involves dealing with 32bit and handling operations accordingly

All translations assume the pointer is at the rightmost, least significant cell

`+`
```
+
I0(
    <+
    I1(
        <+
        I2(
            <+>
        )()
        >
    )()
    >
)()
```
This effectively translates to add 1 to least significant, if it 0 then add one to next cell (carry) and keep going to the most significant cell

`-`
```
I0(
    <
    I1(
        <
        I2(
            <->
        )()
        ->
    )()
    ->
)()
-
```
THis works in a similar vein to addition, but the zero check has to be done before the subtraction

`[`
```
Copy Dx to W0
Go to Dx
[
    Go to W1
    +
    Go to Dx
    [-] // set Dx to 0
]
Move W0 to Dx
Repeat for x=0,1,2,3. This sets W1 to a counter for amount of 0s, so if W1 is 0, D is 0
Go to W1
[< //open a bracket, go back to D0
```

`]`
```
same process as '[' to set W1 to 0 if D is 0, else W0 != 0
position pointer at W1 cell
]< //close bracket on W1, move back to D0
```

all other mappins for D mode and all for W mode are either trivial, or no change is required