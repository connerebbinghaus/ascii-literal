extern crate ascii_literal;
extern crate ascii;

const ASCII: &ascii::AsciiStr = ascii_literal::ascii_literal!("Ø");
//~^ error: String is not valid ascii!
fn main(){}