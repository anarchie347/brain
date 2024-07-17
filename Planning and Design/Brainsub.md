# Brainsub

Brainsub is an extension of brainfuck that adds useful subroutines.

Brainsub has syntax the same as brainfuck and uses brainfuck instructions. The only difference is some subroutines require arguements. These are given in parentheses `()` after the instructions. If multiple arguments are given, they should be in separate parentheses.

## Subroutines:

`S(x)` Sets the current cell to the numerical value x

`A(x)` Sets the current cell to the ascii value of x

`L(x)` Move left x times

`R(x)` Move right x times