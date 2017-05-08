// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate nom_lua;

pub fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();
    let ast = nom_lua::parse_string("10 / 20 * 30".as_bytes()).unwrap();
    println!("{}", ast);
    ast.graphviz_render(&mut f);
}
