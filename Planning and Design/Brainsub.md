# Brainsub

Brainsub is an extension of brainfuck that adds useful subroutines.

## Subroutines:

`Z` Sets the current cell to 0

`S(x)` Sets the current cell to the numerical value x

`A(x)` Sets the current cell to the ascii value of x

`L(x)` Move left x times

`R(x)` Move right x times

`M(x)` Move the value in the current cell to the cells given by the patttern `x`. Deletes the original value. `x` must be a string consisting of 1s, 0s, and 1 occurence of _. The space indicates the current cell in the pattern, a zero represents 'dont move to here', a 1 represents 'move to here`. For example, the following string '100_1101' means that (taking the current cell as index [0]) the value should be moved into cells [-3], [1], [2], [4]. The table below may help:

| -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4
|-|-|-|-|-|-|-|-|
|1|0|0|_|1|1|0|1|