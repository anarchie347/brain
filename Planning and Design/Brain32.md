# Brain32

Brain32 is brainfuck but it is based on 32 bit cells rather than 8 bit cells.

The instruction set is identical to brainfuck with some additions to improve performance.

In the following, x represents a single number (0 to 3) and X represents a string of up to 4 numbers (0 to 3)

- `+x` increments the current cell. x indicates incrementing by 256^x, defaults to 0. This gives an efficiency improvement because the 32bit numbers are stored as 4 8bit numbers
- `-x` decrements current cell. x behaves the same as with +
- `.x` outputs the 8bit cell value of one of the sub-cells. x=0 gives least significant and x=3 gives most significant. Defaults to 0
- `,x` inputs and stores to cell x with behaviour same as that of .
- `>` goes to the next cell
- `<` goes to the previous cell
- `[X` start of pre-condition 'is non-zero' loop. X represents which sub-cells to include (AND is used). Defaults to 0123
- `]X` end of pre-condition 'is non-zero' loop. X works same as with `[`

# Internally

Internally memory is segmented into blocks of 5 brainfuck cells as follows:

W | D D D D W | D D D D W | D D D D W |...

D represents 'data' cells which are used to store the 32bit numbers
W represents 'working' cells which are used for calculations required to apply a brainfuck operation correctly to 4 cells

each 4 D cells is followed by a W cell to create a block of 5.

The first cell of the tape is also a W cell for efficiency reasons (allows all blocks to use the W behind them). Although this could be viewed as starting
with a W then 4 D, algorithms primarily use the W after because it makes the algorithm slightly easier to understand, so it is best to view the W cell to be after the 4 D cells.

The next level up from Brain32 is Brain3U, which is a simple substitution of shortcuts for Brain32 code, so is not documented in its own file