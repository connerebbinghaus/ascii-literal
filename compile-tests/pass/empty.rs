const EMPTY: &ascii::AsciiStr = ascii_literal::ascii_literal!("");

fn main(){
    assert!(EMPTY.is_empty());
}