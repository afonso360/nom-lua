/*
 * Copyright (c) the nom-lua contributors. All rights reserved.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation. This file is also subject
 * to the Linking exception provided in the LICENSE file that
 * accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//#![deny(missing_docs)]
//#![deny(warnings)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate hexf_parse;

#[cfg(feature="graphviz")]
extern crate dot;

// this is going to be usefull when looking at this crate
// https://www.lua.org/manual/5.3/manual.html#9

macro_rules! ast_panic_test {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        #[should_panic]
        fn $name () {
            use super::*;
            $func($input.as_bytes()).unwrap().1;
        }
    }
}

macro_rules! ast_test {
    ($name: ident, $func: ident, $input: expr, $output: expr) => {
        #[test]
        fn $name () {
            use super::*;
            assert_eq!($func($input.as_bytes()).unwrap().1, $output);
        }
    }
}

macro_rules! ast_valid {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        fn $name () {
            use super::*;
            assert!(match $func($input.as_bytes()).unwrap().1 {
                _ => true,
            });
        }
    }
}

macro_rules! ast_invalid {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        #[should_panic]
        fn $name () {
            use super::*;
            $func($input.as_bytes()).unwrap().1;
        }
    }
}

macro_rules! astb {
    ($name: ident, $($a: expr),*) => {
        $name($(Box::new($a)),*)
    };
}

macro_rules! ast {
    ($name: ident) => {
        $name
    };
    ($name: ident, $($a: expr),*) => {
        $name($($a),*)
    };
}



use function::parse_block;
pub use ast::ASTNode;
use std::io::Read;

pub mod ast;
pub mod op;
pub mod number;
pub mod exp;
pub mod string;
pub mod name;
pub mod var;
pub mod field;
pub mod statement;
pub mod function;

pub use nom::IResult;

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
