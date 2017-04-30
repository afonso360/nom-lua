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
        for i in ints.iter() {
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
        for i in ints.iter() {
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
        for i in ints.iter() {
            number::parse_number(i.as_bytes());
        }
    });
}
