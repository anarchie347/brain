# Brain32

possible replacement for Brainsplit and Braincds

Brain32 segments memory as the following:

W D D D | W D D D D | W D D D D

W - Working
D - data

The D cells are grouped in 4s and act as a single, 32 bit cell.
The W cells are used for the calculations to link these cells together

Brain32 exposes to the programmer the following memory:

W D | W D | W D 

W - Working
D - Data (32 bit)

This is to allow more efficient memory usage for grouping cells get 32bit cells
THe following symbols replace `<>` from brainfuck in Brain32:
'=>' indicates what it is mapped to in brainfuck

`^` Data to working => `>>>`
`v` Working to data => `<<<`
`>` move right to next of same type `>>>>`
`<` move left to next of same type `<<<<`

`+-` instructions are converted to work with the full 4 cells of data

## HOW AM I GONNA DEAL WITH MOVING 32bit NUMS INTO WORKING