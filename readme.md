
# rlisp
### A super tiny lisp in Rust


IDEA:

```c

//( get the real basics working first. )

(+ 1 2) //( returns 3 )

```



DEFINING FOREIGN FUNCTIONS:
try do it this way:::
```rust

use rlisp;


fn times(f: &mut FunctionEnv) {
    let x = get_number(f, 1);
    let y = get_number(f, 2);
    let ret = Value::Number(x * y);
    push_return(f, ret);
}

```
