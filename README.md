# ascii-literal
A proc-macro to make compile-time checked AsciiStrs.

## Examples
This compiles just fine, and works as you would expect.
```rust
use ascii_literal::ascii_literal;
const MESSAGE: &ascii::AsciiStr = ascii_literal!("Hello in ASCII!");
println!("{}", MESSAGE); // Prints "Hello in ASCII!"
```

This, however, will give a compile time error.
```rust,compile_fail
// This doesn't compile!
const NOT_ASCII: &ascii::AsciiStr = ascii_literal::ascii_literal!("Boom! 💥");
```
