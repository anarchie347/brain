# Brainsub

Brainsub is an extension of brainfuck that adds useful subroutines.

Brainsub has syntax the same as brainfuck and uses brainfuck instructions. The only difference is some subroutines require arguements. These are given in parentheses `()` after the instructions. If multiple arguments are given, they should be in separate parentheses.

## Subroutines:

`Z` Sets the current cell to 0

`S(x)` Sets the current cell to the numerical value x

`A(x)` Sets the current cell to the ascii value of x

`L(x)` Move left x times

`R(x)` Move right x times

`M(x)` Move the value in the current cell to the cells given by the patttern `x`. Deletes the original value. `x` must be a string consisting of 1s, 0s, and 1 occurence of a space. The space indicates the current cell in the pattern, a zero represents 'dont move to here', a 1 represents 'move to here`. For example, the following string '100 1101' means that (taking the current cell as index [0]) the value should be moved into cells [-3], [1], [2], [4]. The table below may help:

| -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4
|-|-|-|-|-|-|-|-|
|1|0|0|_|1|1|0|1|