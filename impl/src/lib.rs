//! Implementation detail of `ascii-literal`.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use ascii::AsAsciiStr;

const ALL: [&'static str; 128] = [
    "Null", "SOH", "SOX", "ETX", "EOT", "ENQ", "ACK", "Bell",
    "BackSpace", "Tab", "LineFeed", "VT", "FF", "CarriageReturn", "SI", "SO",
    "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB",
    "CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US",
    "Space", "Exclamation", "Quotation", "Hash", "Dollar", "Percent", "Ampersand", "Apostrophe",
    "ParenOpen", "ParenClose", "Asterisk", "Plus", "Comma", "Minus", "Dot", "Slash",
    "_0", "_1", "_2", "_3", "_4", "_5", "_6", "_7",
    "_8", "_9", "Colon", "Semicolon", "LessThan", "Equal", "GreaterThan", "Question",
    "At", "A", "B", "C", "D", "E", "F", "G",
    "H", "I", "J", "K", "L", "M", "N", "O",
    "P", "Q", "R", "S", "T", "U", "V", "W",
    "X", "Y", "Z", "BracketOpen", "BackSlash", "BracketClose", "Caret", "UnderScore",
    "Grave", "a", "b", "c", "d", "e", "f", "g",
    "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w",
    "x", "y", "z", "CurlyBraceOpen", "VerticalBar", "CurlyBraceClose", "Tilde", "DEL",
];

#[proc_macro_hack]
pub fn ascii_literal(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as LitStr);
    let s = expr.value();

    let a = if let Ok(a) = (s[..]).as_ascii_str() {
        a
    } else {
        return syn::Error::new_spanned(expr, "String is not valid ascii!").to_compile_error().into();
    };

    let chars = a.as_bytes().iter().map(|&ch| {
        quote::format_ident!("{}", ALL[ch as usize])
    });

    let t = quote!{
        {
            const __ASCII_LITERAL_CHARS: &'static [::ascii::AsciiChar] = &[#(::ascii::AsciiChar::#chars),*];
            const __ASCII_LITERAL_STR: &'static ::ascii::AsciiStr = unsafe { 
                ::ascii_literal::Transmute {
                    from: __ASCII_LITERAL_CHARS
                }.to
            };
            __ASCII_LITERAL_STR
        }
    };
    t.into()
}