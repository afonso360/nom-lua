// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(test)]

extern crate test;
extern crate nom_lua;

use test::Bencher;
use nom_lua::number;

#[bench]
fn parse_hex_int(b: &mut Bencher) {
    let ints: Vec<String> =
        test::black_box((0..32)
                        .map(|i| format!("0x{:X}", i).to_string())
                        .collect());

    b.iter(|| {
        for i in &ints {
            number::parse_number(i.as_bytes());
        }
    });
}

#[bench]
fn parse_int(b: &mut Bencher) {
    let ints: Vec<String> =
        test::black_box((0..32)
                        .map(|i| format!("{}", i).to_string())
                        .collect());

    b.iter(|| {
        for i in &ints {
            number::parse_number(i.as_bytes());
        }
    });
}

#[bench]
fn parse_float(b: &mut Bencher) {
    let ints: Vec<String> =
        test::black_box((0..32)
                        .map(|i| format!("{}", i as f32).to_string())
                        .collect());

    b.iter(|| {
        for i in &ints {
            number::parse_number(i.as_bytes());
        }
    });
}
