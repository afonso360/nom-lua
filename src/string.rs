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
use nom::hex_digit;
use std::str;
use std_unicode;

//named!(pub parse_string< ASTNode >, alt!(parse_string_bracket | parse_string_regular));
//
//named!(parse_string_bracket< ASTNode >, )
//named!(parse_string_regular< ASTNode >, )

named!(parse_unicode<char>,
       map_opt!(
           map_res!(
               map_res!(
                   delimited!(tag!("\\u{"), recognize!(hex_digit), tag!("}")),
                   str::from_utf8),
                   |h| u32::from_str_radix(h, 16)),
                   std_unicode::char::from_u32));

#[cfg(test)]
mod tests {
    ast_panic_test!(test_parse_unicode_1, parse_unicode, "\\u{}");
    ast_test!(test_parse_unicode_2, parse_unicode, "\\u{A}", std_unicode::char::from_u32(0xA).unwrap());
    ast_test!(test_parse_unicode_3, parse_unicode, "\\u{a2}", std_unicode::char::from_u32(0xa2).unwrap());
    ast_test!(test_parse_unicode_4, parse_unicode, "\\u{AFf9}", std_unicode::char::from_u32(0xAFf9).unwrap());
    ast_test!(test_parse_unicode_5, parse_unicode, "\\u{0000000000000FFFF}", std_unicode::char::from_u32(0xFFFF).unwrap());
    ast_test!(test_parse_unicode_6, parse_unicode, "\\u{10FFFF}", std_unicode::char::from_u32(0x10FFFF).unwrap());
    ast_panic_test!(test_parse_unicode_7, parse_unicode, "\\u{110000}");
}
