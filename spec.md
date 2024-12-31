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

expr -> literal
      | unary
      | binary
      | grouping ;

literal -> NUMBER | STRING | "true" | "false" | "nil";

unary -> ("!" | "-") expr ;

binary -> expr operator expr ;

grouping -> "(" expr ")" ;

operator -> "==" | "!=" | "<=" | ">="
          | "+" | "-" | "<" | ">" | "*" | "/"
          | "and" | "or" ;

```
let expr = 1 + 3
let binary = expr(1) operator(+) expr(3)

let expr = -1 / 3
let unary = -1
let binary = expr(unary(- 1)) operator(/) expr(3)
```

that's fucking magic
> go down
> reach the bottom
> return one
> go right
> maybe match some shit(s)
> repeat
> you're now up with you fking AST parsed

```
expr    -> eq

eq      -> cmp ( ( "==" | "!=" ) cmp )* ;

cmp     -> term ( (">" | "<" | ">=" | "<=" ) term )* ;

term    -> factor ( ("+" "-") factor )* ;

factor -> unary ( ("/" | "*" ) unary )* ;

unary   -> ( "!" | "-" ) unary
         | primary;

primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expr ")" ;
```