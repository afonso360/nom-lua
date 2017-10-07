// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use std::str::FromStr;

use super::nom::{digit, hex_digit};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Number {
    Float(f64),
    Integer(i64),
}
use self::Number::*;

//TODO: LOCALE dependent decimal point!
//TODO: Hex numbers

named!(parse_int_overflow<Number>, map!(
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           Number::Float));

named!(parse_int<Number>, map!(
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           Number::Integer));

named!(parse_hex_int<Number>,
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

named!(parse_float<Number>,
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


fn build_hex_float(((dec, frac), exp): ((Option<&[u8]>, Option<&[u8]>), Option<&[u8]>)) -> Number {
    use self::Number::Float;

    if let Some(x) = dec {
        return Float(
            FromStr::from_str(
                str::from_utf8(x).expect("Invalid utf8")
            ).expect("Invalid f64")
        )
    }
    Float(0.0)
}

named!(parse_hex_float_exp<&[u8]>, recognize!(do_parse!(
               alt!(tag!("p") | tag!("P"))
            >> opt!(alt!(tag!("+") | tag!("-")))
            >> d: hex_digit
            >> (d))));


named!(parse_hex_float<Number>, do_parse!(
       preceded!(tag!("0"), alt!(tag!("x") | tag!("X")))
    >> main: alt!(
           do_parse!(
                  d: hex_digit
               >> tag!(".")
               >> f: opt!(complete!(hex_digit))
               >> ((Some(d), f))
            ) |
           do_parse!(
                  d: opt!(hex_digit)
               >> tag!(".")
               >> f: hex_digit
               >> ((d, Some(f)))
            ) |
           do_parse!(d: hex_digit >> ((Some(d), None)))
       )
    >> exp: opt!(complete!(parse_float_exp))
    >> (build_hex_float((main, exp)))
));

named!(pub parse_number<Number>, dbg_dmp!(alt!(
            complete!(parse_hex_float) |
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

    // preceding +/- are separate Nodes
    ast_panic_test!(parse_int_4, parse_int, "-20");
    ast_panic_test!(parse_int_5, parse_int, "+20");


    ast_test!(parse_hex_1, parse_hex_int, "0X20", ast!(Integer, 0x20));
    ast_test!(parse_hex_2, parse_hex_int, "0x20", ast!(Integer, 0x20));
    ast_test!(parse_hex_3, parse_hex_int, "0x20a", ast!(Integer, 0x20A));
    ast_test!(parse_hex_4, parse_hex_int, "0x20aB", ast!(Integer, 0x20AB));
    ast_test!(parse_hex_5, parse_hex_int, "0X20F", ast!(Integer, 0x20F));
    // need 0x preceding to parse sucessfully
    ast_panic_test!(parse_hex_6, parse_hex_int, "20");
    // preceding +/- are separate Nodes
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

    // preceding +/- are separate Nodes
    ast_panic_test!(parse_float_13, parse_float, "-20.0");
    ast_panic_test!(parse_float_14, parse_float, "+20.0");

    ast_test!(parse_float_15, parse_float, format!("{:.64}", 1.0), ast!(Float, 1.0));


    ast_test!(parse_hex_float_1, parse_hex_float, "0x1.0", ast!(Float, 1.0));
    //ast_test!(parse_hex_float_2, parse_hex_float, "0xFF.0p0", ast!(Float, 255.0));
    //ast_test!(parse_hex_float_3, parse_hex_float, "0x0.1E", ast!(Float, 0.1171875));
    ast_test!(parse_hex_float_4, parse_hex_float, "0x0.p1", ast!(Float, 0.0));
    ast_test!(parse_hex_float_5, parse_hex_float, "0X0.", ast!(Float, 0.0));
    //ast_test!(parse_hex_float_6, parse_hex_float, "0X0.p11323123", ast!(Float, 0.0));
    //ast_test!(parse_hex_float_7, parse_hex_float, "0x1.9f1a12718ed76p-67", ast!(Float, 1.0987654321e-20));

    ast_test!(parse_number_1, parse_number, "20", ast!(Integer, 20));
    ast_test!(parse_number_2, parse_number, "20.0", ast!(Float, 20.0));
    ast_test!(parse_number_3, parse_number, "0x20", ast!(Integer, 0x20));
    ast_test!(parse_number_4, parse_number, "1000000000000000000000000", ast!(Float, 1e+24));
    //ast_panic_test!(parse_number_5, parse_number, "10f");

    quickcheck! {
        fn quickcheck_can_parse_default_int_formatter(x: u64) -> bool {
            use number::Number::*;
            use nom::IResult;
            let formatted = format!("{}", x);
            let parsed = super::parse_int(formatted.as_bytes());
            if let IResult::Done(a, b) = parsed {
                return b == ast!(Integer, x as i64);
            }
            false
        }
        fn quickcheck_can_parse_default_float_formatter(x: f64) -> bool {
            use number::Number::*;
            use nom::IResult;
            let formatted = format!("{:.64}", x.abs());
            let parsed = super::parse_float(formatted.as_bytes());
            println!("{} === {:?}", formatted, parsed);
            if let IResult::Done(a, b) = parsed {
                return b == ast!(Float, x.abs());
            }
            false
        }
    }
}
