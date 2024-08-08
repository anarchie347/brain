# Brain32

possible replacement for Brainsplit and Braincds

Brain32 segments memory as the following:

W D D D | W D D D D | W D D D D

W - Working
D - data

The D cells are grouped in 4s and act as a single, 32 bit cell.

The W cells are used for the calculations to link these cells together

Brain32 exposes two memories: D (32bit), W(8bit)

This is to allow more efficient memory usage for grouping cells get 32bit cells

`+-` instructions are converted to work with the full 4 cells of data

32 bit nums cannot be moved into memory directly, they must be moved 8 bits at a time and delat with individually

The next language up will implement functions that use working and will not expose W to higher languages, so higher languages do not have to deal with W cells not being 32bit



There are 2 modes to write brain32 code:
- D mode
- W mode

This determines which type of memory is being accessed

By default, all code is in D mode

Any code wrapped in `{ }` is W mode

In terms of compilation to a lower language, this means:

### D mode:
`+` `-` `[` `]` all work based on the 32bit number stored

`,` `.` only operate on the first 8bits

`<` `>` are mapped to `<<<<` `>>>>`

`M(x)` and `C(x)` are operatiojns that move or copy respectively the cell in position x (0, 1, 2, 3) into the W cell for that block

Except mid-operation, the pointer is at the rightmost, least significant cell of the 4 D cells

### W mode:
`<` `>` are mapped to `<<<<` `>>>>`

All other operations work as they usually do in brainfuck, on the 8bit cell

The encasing braces `{` `}` corresspond to `<<<` `>>>` respectively to move in and out of working memory.

`M(x)` and `C(x)` are operatiojns that move or copy respectively the W cell into the cell in position x (0, 1, 2, 3)

The pointer is pointing at a W cell