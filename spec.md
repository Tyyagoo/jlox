```ocaml
1 + 2 * 3 - 4
1 + 6 - 4
7 - 4
3


1 + 2 * 3 - 4         // source code
(- (+ 1 (* (2 3))) 4) // kind of a tree <- should look like this in the AST
(- (+ 1 6) 4)         // traverse the AST in post-order
(- 7 4)
3
```
