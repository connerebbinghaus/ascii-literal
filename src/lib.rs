//! A proc-macro to make compile-time checked AsciiStrs.
//! 
//! Provides the macro [`ascii_literal`], which const-evaluates to an [`AsciiStr`](ascii::AsciiStr),
//! checking for valid ASCII at compile time.

#![no_std]

use proc_macro_hack::proc_macro_hack;

/// Creates an [`AsciiStr`](ascii::AsciiStr) from a string literal.
/// 
/// Produces a compile error if the string is not valid ASCII.
/// 
/// # Examples
/// ```rust
/// use ascii_literal::ascii_literal;
/// const MESSAGE: &ascii::AsciiStr = ascii_literal!("Hello in ASCII!");
/// assert_eq!(MESSAGE.as_str(), "Hello in ASCII!");
/// ```
/// 
/// ```compile_fail
/// // This doesn't compile!
/// const OOPS: &ascii::AsciiStr = ascii_literal::ascii_literal!("💥");
/// ```
#[proc_macro_hack]
pub use ascii_literal_impl::ascii_literal;

#[doc(hidden)]
pub union Transmute<T: Copy, U: Copy> {
    pub from: T,
    pub to: U,
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("compile-tests/fail/*.rs");
        t.pass("compile-tests/pass/*.rs");
    }
}