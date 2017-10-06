// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(specialization)]

//#![deny(missing_docs)]
//#![deny(warnings)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

// TODO: Remove this
#![allow(unused_macros, unused_imports)]


#[macro_use]
extern crate nom;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

// this is going to be usefull when looking at this crate
// https://www.lua.org/manual/5.3/manual.html#9


#[macro_use]
mod macros;
mod lexer;

pub use nom::IResult;

use lexer::{lex, Token};

// TODO: Implement our own Error type
pub fn lex_string<'a, T: Into<&'a [u8]>>(s: T) -> Option<Vec<Token<'a>>> {
    match lex(s.into()) {
        IResult::Done(_, a) => Some(a),
        _ => None
    }
}


#[cfg(test)]
mod tests {
}

/*
//named!(pub parse_chunk<ASTNode>, ws!(parse_block));
use exp::parse_exp;
named!(pub parse_chunk<ASTNode>, dbg_dmp!(ws!(parse_exp)));

// TODO: Implement our own Error type
pub fn parse_string<'a, T: Into<&'a [u8]>>(s: T) -> Option<ASTNode> {
    match parse_chunk(s.into()) {
        IResult::Done(_, a) => Some(a),
        _ => None
    }
}

pub fn parse<T: Read>(mut s: T) -> Option<ASTNode> {
    let mut buf = vec![];
    s.read_to_end(&mut buf);
    buf.pop(); //Remove EOF
    match parse_chunk(&buf) {
        IResult::Done(_, a) => Some(a),
        _ => None
    }
}
*/
