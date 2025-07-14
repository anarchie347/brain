# Brain32R

Brain32R is an adpation of Brain32, where opertions use relative adresses.

The idea behind Brain32R is that moving operands to the start of the memory cell is expensive and pointless when the loction of one of the operands could be used as a 'base' for the instruction.

This adds some additional complexity, mainly that of pointer arithmetic to calculate relative addresses as opposed to absolute ones. This should be relatively easy to overcome. If either adress is known at ompile time this becomes trivial. If both adresses are unknown, then a subtraction is needded but this shouldnt be too difficult