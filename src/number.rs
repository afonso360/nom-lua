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

use std::str;
use std::str::FromStr;
use ast::ASTNode;
use ast::ASTNode::*;
//use hexf_parse::parse_hexf64;

use super::nom::{digit, hex_digit};
//TODO: LOCALE dependent decimal point!
//TODO: Hex numbers

named!(parse_int_overflow<ASTNode>, map!(
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           ASTNode::Float));

named!(parse_int<ASTNode>, map!(
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           ASTNode::Integer));

named!(parse_hex_int<ASTNode>,
           do_parse!(
               preceded!(tag!("0"), alt!(tag!("x") | tag!("X"))) >>
               hex: map_res!(
                     map_res!(hex_digit, str::from_utf8),
                    |h| i64::from_str_radix(h, 16))
               >> (ast!(Integer, hex))));

named!(parse_float_exp, recognize!(do_parse!(
               alt!(tag!("e") | tag!("E"))
            >> opt!(alt!(tag!("+") | tag!("-")))
            >> digit
            >> ())));

named!(parse_float<ASTNode>,
       do_parse!(
              float: map_res!( map_res!( recognize!( do_parse!(
                         alt!(
                          delimited!(digit, tag!("."), opt!(complete!(digit))) |
                          delimited!(opt!(digit), tag!("."), digit) |
                          digit)
                      >> opt!(complete!(parse_float_exp))
                      >> ())),
                  str::from_utf8), FromStr::from_str)
           >> (ast!(Float, float)))
      );


//named!(parse_hex_float<ASTNode>, map_res!(apply!(parse_hexf64, false), |_| ASTNode::Float(0.0f)));


named!(pub parse_number<ASTNode>, dbg_dmp!(alt!(
            //complete!(parse_hex_float) |
            complete!(parse_hex_int) |
            complete!(parse_float) |
            parse_int |
            parse_int_overflow
)));

#[cfg(test)]
mod tests {
    //The tests panic because the macro calls unwrap, otherwise they should fail gracefully

    ast_test!(parse_int_1, parse_int, "20", ast!(Integer, 20));

    // Overflowing causes integers to be interperted as floats, thus this should fail
    ast_panic_test!(parse_int_3, parse_int, "5678987656789876520999999999999");

    // preceding +/- are separate ASTNodes
    ast_panic_test!(parse_int_4, parse_int, "-20");
    ast_panic_test!(parse_int_5, parse_int, "+20");


    ast_test!(parse_hex_1, parse_hex_int, "0X20", ast!(Integer, 0x20));
    ast_test!(parse_hex_2, parse_hex_int, "0x20", ast!(Integer, 0x20));
    ast_test!(parse_hex_3, parse_hex_int, "0x20a", ast!(Integer, 0x20A));
    ast_test!(parse_hex_4, parse_hex_int, "0x20aB", ast!(Integer, 0x20AB));
    ast_test!(parse_hex_5, parse_hex_int, "0X20F", ast!(Integer, 0x20F));
    // need 0x preceding to parse sucessfully
    ast_panic_test!(parse_hex_6, parse_hex_int, "20");
    // preceding +/- are separate ASTNodes
    ast_panic_test!(parse_hex_7, parse_hex_int, "-0x20");
    ast_panic_test!(parse_hex_8, parse_hex_int, "+0x20");


    ast_test!(parse_float_1, parse_float, "3.0", ast!(Float, 3.0));
    ast_test!(parse_float_2, parse_float, ".1", ast!(Float, 0.1));
    ast_test!(parse_float_3, parse_float, "1.", ast!(Float, 1.0));
    ast_test!(parse_float_4, parse_float, "3.1416", ast!(Float, 3.1416));
    ast_test!(parse_float_5, parse_float, "314.16e-2", ast!(Float, 314.16e-2));
    ast_test!(parse_float_6, parse_float, "0.31416E1", ast!(Float, 0.31416E1));
    ast_test!(parse_float_7, parse_float, "34e1", ast!(Float, 340.0));
    ast_test!(parse_float_8, parse_float, "34e+1", ast!(Float, 340.0));
    ast_test!(parse_float_9, parse_float, "34e-1", ast!(Float, 3.4));
    ast_test!(parse_float_10, parse_float, "34.e-1", ast!(Float, 3.4));
    ast_test!(parse_float_11, parse_float, ".2e1", ast!(Float, 2.0));
    ast_panic_test!(parse_float_12, parse_float, ".e1");

    // preceding +/- are separate ASTNodes
    ast_panic_test!(parse_float_13, parse_float, "-20.0");
    ast_panic_test!(parse_float_14, parse_float, "+20.0");

    ast_test!(parse_number_1, parse_number, "20", ast!(Integer, 20));
    ast_test!(parse_number_2, parse_number, "20.0", ast!(Float, 20.0));
    ast_test!(parse_number_3, parse_number, "0x20", ast!(Integer, 0x20));
    ast_test!(parse_number_4, parse_number, "1000000000000000000000000", ast!(Float, 1e+24));
    //ast_panic_test!(parse_number_5, parse_number, "10f");
}
