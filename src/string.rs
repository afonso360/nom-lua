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
use nom::{hex_digit, digit};
use std::str;
use std_unicode;

//named!(pub parse_string< ASTNode >, alt!(parse_string_bracket | parse_string_regular));
//
//named!(parse_string_bracket< ASTNode >, )
//named!(parse_string_regular< ASTNode >, )

named!(parse_byte<char>, alt!(parse_byte_x | parse_byte_d));

named!(parse_byte_x<char>, map!(map_res!(map_res!(
                preceded!(tag!("\\x"), hex_digit),
                str::from_utf8),
            |s| u8::from_str_radix(s, 16)), |i: u8| i as char));



// TODO: if a decimal escape sequence is to be followed by a digit, it must be expressed using exactly three digits
// Notice, the fold_many_m_n is not actually enforcing bounds here, because digit recognizes
// more than one character, but I think this implementation might be usefull for the future
named!(parse_byte_d<char>, map!(map_res!(
            preceded!(tag!("\\"), fold_many_m_n!(1, 3, digit, String::new(), |mut acc: String, item: &[u8]| {
                for c in item {
                    acc.push(*c as char);
                }
                acc
            })),
            |s: String| s.parse::<u8>()), |i: u8| i as char));

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


    ast_test!(test_parse_byte_d_1, parse_byte_d, "\\0", '\0');
    ast_test!(test_parse_byte_d_2, parse_byte_d, "\\00", '\0');
    ast_test!(test_parse_byte_d_3, parse_byte_d, "\\000", '\0');
    // TODO: This should parse to the rust string "\u{0}0" make this test reflect that
    ast_test!(test_parse_byte_d_4, parse_byte_d, "\\0000", '\0');
    ast_test!(test_parse_byte_d_5, parse_byte_d, "\\230", '\u{E6}');
    ast_panic_test!(test_parse_byte_d_6, parse_byte_d, "\\256");

    ast_test!(test_parse_byte_x_1, parse_byte_x, "\\x00", '\0');
    // TODO: This should parse to the rust string "\u{0}0" make this test reflect that
    ast_test!(test_parse_byte_x_2, parse_byte_x, "\\x000", '\0');
    ast_test!(test_parse_byte_x_3, parse_byte_x, "\\x23", '\u{23}');
    ast_test!(test_parse_byte_x_4, parse_byte_x, "\\xFf", '\u{FF}');
}
