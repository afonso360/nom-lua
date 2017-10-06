// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Plus,
    Mod,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Band,
    Pow,
    Bor,
    Semicolon,
    Comma,
    Hash,
    Idiv,
    Div,
    Mul,
    Rsh,
    Gte,
    Gt,
    Lsh,
    Lte,
    Lt,
    Label,
    Colon,
    Neq,
    Bnot,
    Eq,
    Set,
    DotDotDot,
    DotDot,
    Dot,
    Minus,
    Comment(&'a [u8]),
}

named!(reserved_keyword<&[u8]>, alt!(
    tag!("and") |
    tag!("break") |
    tag!("do") |
    tag!("else") |
    tag!("elseif") |
    tag!("end") |
    tag!("false") |
    tag!("for") |
    tag!("function") |
    tag!("goto") |
    tag!("if") |
    tag!("in") |
    tag!("local") |
    tag!("nil") |
    tag!("not") |
    tag!("or") |
    tag!("repeat") |
    tag!("return") |
    tag!("then") |
    tag!("true") |
    tag!("until") |
    tag!("while")
));


// TODO: Split this into multi level comment
named!(parse_mlc<Token>, do_parse!(
       tag!("--")
    >> level: many1!(tag!("["))
    >> t: take_until!(&( "]".repeat(level.len()) )[..])
    >> (Token::Comment(t))
));

named!(parse_minus<Token>, alt!(
    do_parse!(tag!("--") >> t: take_until!("\n") >> (Token::Comment(t))) |
    do_parse!(tag!("-") >> (Token::Minus))
));


//named!(pub lex<Token>, do_parse!(t: tag!("+") >> (Token::Plus)));
named!(pub lex<Vec<Token>>, many0!(alt_complete!(
    do_parse!(tag!("+") >> (Token::Plus)) |
    do_parse!(tag!("%") >> (Token::Mod)) |
    do_parse!(tag!("{") >> (Token::OpenBrace)) |
    do_parse!(tag!("}") >> (Token::CloseBrace)) |
    do_parse!(tag!("(") >> (Token::OpenParen)) |
    do_parse!(tag!(")") >> (Token::CloseParen)) |
    do_parse!(tag!("&") >> (Token::Band)) |
    do_parse!(tag!("^") >> (Token::Pow)) |
    do_parse!(tag!("|") >> (Token::Bor)) |
    do_parse!(tag!(";") >> (Token::Semicolon)) |
    do_parse!(tag!(",") >> (Token::Comma)) |
    do_parse!(tag!("#") >> (Token::Hash)) |
    do_parse!(tag!("//") >> (Token::Idiv)) |
    do_parse!(tag!("/") >> (Token::Div)) |
    do_parse!(tag!("*") >> (Token::Mul)) |
    do_parse!(tag!(">>") >> (Token::Rsh)) |
    do_parse!(tag!(">=") >> (Token::Gte)) |
    do_parse!(tag!(">") >> (Token::Gt)) |
    do_parse!(tag!("<<") >> (Token::Lsh)) |
    do_parse!(tag!("<=") >> (Token::Lte)) |
    do_parse!(tag!("<") >> (Token::Lt)) |
    do_parse!(tag!("::") >> (Token::Label)) |
    do_parse!(tag!(":") >> (Token::Colon)) |
    do_parse!(tag!("~=") >> (Token::Neq)) |
    do_parse!(tag!("~") >> (Token::Bnot)) |
    do_parse!(tag!("==") >> (Token::Eq)) |
    do_parse!(tag!("=") >> (Token::Set)) |
    do_parse!(tag!("...") >> (Token::DotDotDot)) |
    do_parse!(tag!("..") >> (Token::DotDot)) |
    do_parse!(tag!(".") >> (Token::Dot)) |
    parse_mlc |
    parse_minus
)));

#[cfg(test)]
mod tests {
    ast_test!(parse_mlc, parse_mlc, "--[[[ --[[[ ]-]]-- ]]]", Token::Comment(" --[[[ ]-]]-- ".as_bytes()));
    ast_test!(parse_single_chars, lex,
              "+-%",
              vec![
                 Token::Plus,
                 Token::Minus,
                 Token::Mod,
              ]);
}
