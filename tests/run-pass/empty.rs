extern crate ascii_literal;
extern crate ascii;

fn main(){
    const EMPTY: &ascii::AsciiStr = ascii_literal::ascii_literal!("");
    assert!(EMPTY.is_empty());
}