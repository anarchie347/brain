# Braincds

Braincds build on [Brainsplit](./Brainsplit.md) to provide methods that utilise the working memory. Braincds abstracts away the working memory, so it is not accessible to higher level languages. This allows the implementation of braincds to use all the working memory without the possibility of issues of important data being overwritten.

Braincds exposes all methods that exist in [Brainsplit](./Brainsplit.md), expect the overload of `M` that takes 3 arguements and allows moving between memories.

Braincds main feature comes from compiler defined subroutines (hence the 'cds' in the name).
Compiler defined subrouties are a standard libary of subroutines defined within the compiler that use the working memory.
THe following are examples of compiler defined subroutines:

- If statements
- While loops
- For loops
- Hop (Move left or right a number of cells indicated by the current cell)
- Copy (takes an arguement in the same format as [Brainsub](./Brainsub.md) `M`)
