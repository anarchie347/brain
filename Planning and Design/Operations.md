# Operations

For logical and arithmetic operations, at a high level, complex expressions will be expanded into a long series of single calculation variable assignments, which will then all be transated individually Any conditional statements or arguments will then just accept a single variable rather than an expression, hich can just be translated as moving the pointer to that cell. For example:

`if (a + b * c / (d + e)) {}`

will be translated to:

```
let v0 = d + e
let v1 = c / v0
let v2 = b * c1
let v3 = a + v2
if (v3) {}
```
