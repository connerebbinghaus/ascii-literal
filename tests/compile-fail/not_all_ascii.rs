extern crate ascii_literal;
extern crate ascii;

const ASCII: &ascii::AsciiStr = ascii_literal::ascii_literal!("This is not ASCII! 😢");
//~^ error: String is not valid ascii!

fn main(){}