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
use nom::{hex_digit, digit, sp, IResult};
use std::str;
use std_unicode;

named!(pub parse_string< ASTNode >,
       map!(alt!(/*parse_string_literal |*/ parse_string_short_literal), |s| ASTNode::String(s)));


//named!(parse_string_literal<String>, map_res!(raw_string, |s, _| s));

//pub fn raw_string(input: &str) -> IResult<&str, (String, usize)> {
//    let mut chars = input.char_indices();
//    let mut n = 0;
//    while let Some((byte_offset, ch)) = chars.next() {
//        match ch {
//            '"' => {
//                n = byte_offset;
//                break;
//            }
//            '#' => {}
//            _ => return IResult::Error,
//        }
//    }
//    let mut s = String::new();
//    for (byte_offset, ch) in chars {
//        match ch {
//            '"' if input[byte_offset + 1..].starts_with(&input[..n]) => {
//                let rest = &input[byte_offset + 1 + n..];
//                return IResult::Done(rest, (s, n));
//            }
//            '\r' => {}
//            _ => s.push(ch),
//        }
//    }
//    IResult::Error
//}

// TODO: A short literal string cannot contain unescaped line breaks nor escapes not forming a valid escape sequence.
// TODO: " ' inside strings are valid
named!(parse_string_short_literal<String>,
       delimited!(
        alt!(tag!("\"") | tag!("'")),
        fold_many0!(alt!(
            map!(linebreak, |_| '\n') |
            parse_byte |
            parse_unicode |
            one_of!("\x07\x08\x09\x0A\x0B\x0C\x0D")
            // Find a way to discard the output from this: preceded!(tag!(r#"\z"#), alt!(sp |
            // linebreak))
        ), String::new(), |mut acc: String, item| {
            acc.push(item);
            acc
        }),
        alt!(tag!("\"") | tag!("'"))));

named!(parse_byte<char>, alt!(parse_byte_x | parse_byte_d));

named!(parse_byte_x<char>, map!(map_res!(map_res!(
                preceded!(tag!("\\x"), hex_digit),
                str::from_utf8),
            |s| u8::from_str_radix(s, 16)), |i: u8| i as char));

named!(linebreak, alt!(tag!("\\\r\n") | tag!("\\\n\r") | tag!("\\\n")));


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
    ast_panic_test!(test_parse_unicode_1, parse_unicode, r#"\u{}"#);
    ast_test!(test_parse_unicode_2, parse_unicode, r#"\u{A}"#, std_unicode::char::from_u32(0xA).unwrap());
    ast_test!(test_parse_unicode_3, parse_unicode, r#"\u{a2}"#, std_unicode::char::from_u32(0xa2).unwrap());
    ast_test!(test_parse_unicode_4, parse_unicode, r#"\u{AFf9}"#, std_unicode::char::from_u32(0xAFf9).unwrap());
    ast_test!(test_parse_unicode_5, parse_unicode, r#"\u{0000000000000FFFF}"#, std_unicode::char::from_u32(0xFFFF).unwrap());
    ast_test!(test_parse_unicode_6, parse_unicode, r#"\u{10FFFF}"#, std_unicode::char::from_u32(0x10FFFF).unwrap());
    ast_panic_test!(test_parse_unicode_7, parse_unicode, r#"\u{110000}"#);


    ast_test!(test_parse_byte_d_1, parse_byte_d, r#"\0"#, '\0');
    ast_test!(test_parse_byte_d_2, parse_byte_d, r#"\00"#, '\0');
    ast_test!(test_parse_byte_d_3, parse_byte_d, r#"\000"#, '\0');
    // TODO: This should parse to the rust string r#"\u{0}0"# make this test reflect that
    ast_test!(test_parse_byte_d_4, parse_byte_d, r#"\0000"#, '\0');
    ast_test!(test_parse_byte_d_5, parse_byte_d, r#"\230"#, '\u{E6}');
    ast_panic_test!(test_parse_byte_d_6, parse_byte_d, r#"\256"#);


    ast_test!(test_parse_byte_x_1, parse_byte_x, r#"\x00"#, '\0');
    // TODO: This should parse to the rust string "\u{0a}0" make this test reflect that
    //ast_test!(test_parse_byte_x_2, parse_byte_x, r#"\x0a0"#, '\x0a');
    ast_test!(test_parse_byte_x_3, parse_byte_x, r#"\x23"#, '\u{23}');
    ast_test!(test_parse_byte_x_4, parse_byte_x, r#"\x000023"#, '\u{23}');
    ast_test!(test_parse_byte_x_5, parse_byte_x, r#"\xFf"#, '\u{FF}');

    ast_test!(test_parse_string_short_literal_1, parse_string_short_literal, r#""""#, "");
    ast_test!(test_parse_string_short_literal_2, parse_string_short_literal, r#"''"#, "");
    ast_test!(test_parse_string_short_literal_3, parse_string_short_literal, r#"'\u{1F62A}'"#, "😪");
    ast_test!(test_parse_string_short_literal_4, parse_string_short_literal, r#"'\097'"#, "a");
    ast_test!(test_parse_string_short_literal_5, parse_string_short_literal, format!("'{}'", "\x07\x08\x09\x0A\x0B\x0C\x0D"), "\x07\x08\x09\x0A\x0B\x0C\x0D");
    ast_test!(test_parse_string_short_literal_6, parse_string_short_literal, "'\\\n\r'", "\n");
    ast_test!(test_parse_string_short_literal_7, parse_string_short_literal, "'\\\r\n'", "\n");
    ast_test!(test_parse_string_short_literal_8, parse_string_short_literal, "'\\\n'", "\n");
}
