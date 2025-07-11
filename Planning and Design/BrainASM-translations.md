This file is primarily for psuedocode style breakdowns of BrainASM methods:

ALl methods assume they start at index 0, stack memory
Memory blocks are split S H F W M



# Operations

## IADD [0] [1]

stores [0] + [1] in [0], stores true in [1] if overflow.
```
>>> move to [0].W
[
    
]