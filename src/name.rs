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

use ast::ASTNode;
use nom::{alpha, digit};
use std::str;
use std::str::FromStr;

named!(recognize_keyword, alt!(
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

named!(pub parse_name<String>, map_res!(map_res!(do_parse!(
            not!(recognize_keyword) >>
            a: recognize!(preceded!(
                  many1!(alt!(tag!("_") | alpha)),
                  many0!(alt!(tag!("_") | alpha | digit)))) >> (a)),
            str::from_utf8), FromStr::from_str));

named!(pub parse_label<ASTNode>, map!(delimited!(tag!("::"), ws!(parse_name), tag!("::")), ASTNode::Label));


#[cfg(test)]
mod tests {
    ast_test!(test_parse_name_1, parse_name, "il", String::from_str("il").unwrap());
    ast_test!(test_parse_name_2, parse_name, "_il3", String::from_str("_il3").unwrap());
    ast_panic_test!(test_parse_name_3, parse_name, "3lc_");
    ast_panic_test!(test_parse_name_4, parse_name, "not");

    ast_test!(test_parse_label_1, parse_label, "::il::", ASTNode::Label("il".into()));
    ast_test!(test_parse_label_2, parse_label, ":: z ::", ASTNode::Label("z".into()));
}
