# ascii-literal
A proc-macro to make compile-time checked AsciiStrs.

## Example
```rust
use ascii_literal::ascii_literal;
const MESSAGE: &ascii::AsciiStr = ascii_literal!("Hello in ASCII!");
println!("{}", MESSAGE);
```
