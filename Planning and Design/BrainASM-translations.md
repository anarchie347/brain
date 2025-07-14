This file is primarily for psuedocode style breakdowns of BrainASM methods:

ALl methods assume they start at index 0, stack memory
Memory blocks are split S H F W M



# Operations

## IADD [0] [1]

stores [0] + [1] in [0], stores true in [1] if overflow.
```
add units column, with overflow checking
add 256^1 column, with overflow checking (and adding carry if needed)
etc...
for 256^3 column, if overflow mark this as overflow operation
```
For this, can use the raw operation exposed by Brain32. The full `+` operator that deals with carrying is not needed as duign the adding of a particular column, at most 1 carry will be done so it will be much more efficient to manualy implement this.

## ISUB [0] [1]

similar to IADD but with subtraction




## BAND

if [0] truthy, copy/move [1] to [2], else [2] = 0

## BORR

if [0] truthy, add 1 to [2], if [1] truthy, add 1 to [2], check if [2] is truthy

## BXOR

set [2] = -1, if [0] truthy add 1 to [2], if [1] truthy add 1 to [2], check if [2] = 0
