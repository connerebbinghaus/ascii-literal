const ASCII: &ascii::AsciiStr = ascii_literal::ascii_literal!("Hello!");

fn main(){
    assert_eq!(ASCII.as_str(), "Hello!");
}