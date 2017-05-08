// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate nom_lua;

fn exec_repl() {
    use std::io::{BufRead, Write};
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        if let nom_lua::IResult::Done(_, string) =
            nom_lua::parse_chunk(line.expect("Failed to read line").as_bytes()) {
            println!("EVAL: {}", string);
        } else {
            println!("ERROR: Parse Error");
        }
        print!("> ");
        stdout.lock().flush().expect("Failed to flush");
    }
}

fn main() {
    exec_repl();
}
