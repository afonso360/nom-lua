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

use super::nom::{digit, hex_digit, double};

named!(parse_hex_beginning, recognize!(preceded!(tag!("0"), alt!(tag!("x") | tag!("X")))));

named!(parse_int< ASTNode >,
       map!(
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           ASTNode::Integer));

named!(parse_hex_int< ASTNode >,
           do_parse!(
               parse_hex_beginning >>
               hex: map_res!(
                     map_res!(hex_digit, str::from_utf8),
                    |h| i64::from_str_radix(h, 16))
               >> (ASTNode::Integer(hex))));

named!(parse_float_exp, recognize!(do_parse!(
               alt!(tag!("e") | tag!("E"))
            >> opt!(alt!(tag!("+") | tag!("-")))
            >> digit
            >> ())));

named!(parse_float_exp_no_sign, recognize!(do_parse!(
               alt!(tag!("e") | tag!("E"))
            >> digit
            >> ())));


named!(parse_hex_float_exp, recognize!(do_parse!(
               alt!(tag!("p") | tag!("P"))
            >> opt!(alt!(tag!("+") | tag!("-")))
            >> hex_digit
            >> ())));


named!(parse_float< ASTNode >,
       do_parse!(
              float: map_res!(
                         map_res!(
                             recognize!(
                                 do_parse!(
                                         alt!(
                                             delimited!(digit, tag!("."), opt!(complete!(digit))) |
                                             delimited!(opt!(digit), tag!("."), digit) |
                                             digit)
                                         >> opt!(complete!(parse_float_exp))
                                         >> ())),
                              str::from_utf8),
                         FromStr::from_str)
           >> (ASTNode::Float(float)))
      );




named!(parse_hex_float< ASTNode >,
       do_parse!(
              float: map_res!(
                         map_res!(
                             recognize!(
                                 do_parse!(
                                         alt!(
                                             delimited!(hex_digit, tag!("."), opt!(complete!(hex_digit))) |
                                             delimited!(opt!(hex_digit), tag!("."), hex_digit) |
                                             hex_digit)
                                         >> opt!(complete!(parse_float_exp_no_sign))
                                         >> opt!(complete!(parse_hex_float_exp))
                                         >> ())),
                              str::from_utf8),
                         FromStr::from_str)
           >> (ASTNode::Float(float)))
       );




named!(parse_number< ASTNode >,
       alt!(parse_hex_float | parse_float | parse_hex_int | parse_int));

#[cfg(test)]
mod tests {
    //The tests panic because the macro calls unwrap, otherwise they should fail gracefully

    ast_test!(test_parse_int_1, parse_int, "20", ASTNode::Integer(20));
    // Overflowing causes integers to be interperted as floats, thus this should fail
    ast_panic_test!(test_parse_int_3, parse_int, "5678987656789876520", ASTNode::Integer(1));
    // preceding +/- are separate ASTNodes
    ast_panic_test!(test_parse_int_4, parse_int, "-20", ASTNode::Integer(-20));
    ast_panic_test!(test_parse_int_5, parse_int, "+20", ASTNode::Integer(20));


    ast_test!(test_parse_hex_1, parse_hex_int, "0X20", ASTNode::Integer(0x20));
    ast_test!(test_parse_hex_2, parse_hex_int, "0x20", ASTNode::Integer(0x20));
    ast_test!(test_parse_hex_3, parse_hex_int, "0x20a", ASTNode::Integer(0x20A));
    ast_test!(test_parse_hex_4, parse_hex_int, "0x20aB", ASTNode::Integer(0x20AB));
    ast_test!(test_parse_hex_5, parse_hex_int, "0X20F", ASTNode::Integer(0x20F));
    // need 0x preceding to parse sucessfully
    ast_panic_test!(test_parse_hex_6, parse_hex_int, "20", ASTNode::Integer(32));
    // preceding +/- are separate ASTNodes
    ast_panic_test!(test_parse_hex_7, parse_hex_int, "-0x20", ASTNode::Integer(-32));
    ast_panic_test!(test_parse_hex_8, parse_hex_int, "+0x20", ASTNode::Integer(32));


    ast_test!(test_parse_float_1, parse_float, "3.0", ASTNode::Float(3.0));
    ast_test!(test_parse_float_2, parse_float, ".1", ASTNode::Float(0.1));
    ast_test!(test_parse_float_3, parse_float, "1.", ASTNode::Float(1.0));
    ast_test!(test_parse_float_4, parse_float, "3.1416", ASTNode::Float(3.1416));
    ast_test!(test_parse_float_5, parse_float, "314.16e-2", ASTNode::Float(314.16e-2));
    ast_test!(test_parse_float_6, parse_float, "0.31416E1", ASTNode::Float(0.31416E1));
    ast_test!(test_parse_float_7, parse_float, "34e1", ASTNode::Float(340.0));
    ast_test!(test_parse_float_8, parse_float, "34e+1", ASTNode::Float(340.0));
    ast_test!(test_parse_float_9, parse_float, "34e-1", ASTNode::Float(3.4));
    ast_test!(test_parse_float_10, parse_float, "34.e-1", ASTNode::Float(3.4));
    ast_test!(test_parse_float_11, parse_float, ".2e1", ASTNode::Float(2.0));
    ast_panic_test!(test_parse_float_12, parse_float, ".e1", ASTNode::Float(0.0));

    // preceding +/- are separate ASTNodes
    ast_panic_test!(test_parse_float_13, parse_float, "-20.0", ASTNode::Float(-20.0));
    ast_panic_test!(test_parse_float_14, parse_float, "+20.0", ASTNode::Float(20.0));



    // More complete tests are needed for this
    ast_valid!(test_parse_hex_float_1, parse_hex_float, "0x0.0");
    ast_valid!(test_parse_hex_float_2, parse_hex_float, "0x3.0");
    ast_valid!(test_parse_hex_float_3, parse_hex_float, "0x.1");
    ast_valid!(test_parse_hex_float_4, parse_hex_float, "0x1.");
    ast_valid!(test_parse_hex_float_5, parse_hex_float, "0x3.1416");
    ast_valid!(test_parse_hex_float_6, parse_hex_float, "0x314.16e-2");
    ast_valid!(test_parse_hex_float_7, parse_hex_float, "0x0.31416E1");
    ast_valid!(test_parse_hex_float_8, parse_hex_float, "0x34e1");
    ast_valid!(test_parse_hex_float_9, parse_hex_float, "0x34e+1");
    ast_valid!(test_parse_hex_float_10, parse_hex_float, "0x34e-1");
    ast_valid!(test_parse_hex_float_11, parse_hex_float, "0x34.e-1");
    ast_valid!(test_parse_hex_float_12, parse_hex_float, "0x.0e0");
    ast_valid!(test_parse_hex_float_13, parse_hex_float, "0x.e1");
    ast_valid!(test_parse_hex_float_14, parse_hex_float, "0x.e1p1");
    ast_valid!(test_parse_hex_float_15, parse_hex_float, "0x3.0p4");
    ast_valid!(test_parse_hex_float_16, parse_hex_float, "0x0.1E");
    ast_valid!(test_parse_hex_float_17, parse_hex_float, "0xA23p-4");
    ast_valid!(test_parse_hex_float_18, parse_hex_float, "0X1.921FB54442D18P+1");

    // preceding +/- are separate ASTNodes
    ast_invalid!(test_parse_hex_float_19, parse_hex_float, "-20.0");
    ast_invalid!(test_parse_hex_float_20, parse_hex_float, "+20.0");


    //ast_test!(test_parse_number_1, parse_number, "20", ASTNode::Integer(20));
    //ast_test!(test_parse_number_2, parse_number, "20.0", ASTNode::Float(20.0));
    //ast_test!(test_parse_number_3, parse_number, "1000000000000000000000000", ASTNode::Float(1e+24));
}
