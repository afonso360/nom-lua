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

//#![deny(missing_docs)]
//#![deny(warnings)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate hexf;

macro_rules! ast_panic_test {
    ($name: ident, $func: ident, $input: expr, $output: expr) => {
        #[test]
        #[should_panic]
        fn $name () {
            use super::*;
            assert_eq!($func($input.as_bytes()).unwrap().1, $output);
        }
    }
}

macro_rules! ast_test {
    ($name: ident, $func: ident, $input: expr, $output: expr) => {
        #[test]
        fn $name () {
            use super::*;
            assert_eq!($func($input.as_bytes()).unwrap().1, $output);
        }
    }
}

macro_rules! ast_valid {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        fn $name () {
            use super::*;
            assert!(match $func($input.as_bytes()).unwrap().1 {
                _ => true,
            });
        }
    }
}

macro_rules! ast_invalid {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        #[should_panic]
        fn $name () {
            use super::*;
            $func($input.as_bytes()).unwrap().1;
        }
    }
}


pub mod ast;
pub mod op;
pub mod number;


pub fn exec_repl() {
    use std::io::{BufRead, Write};
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        if let nom::IResult::Done(_, string) =
            op::parse_op(line.expect("Failed to read line").as_bytes()) {
            println!("EVAL: {}", string);
        } else {
            println!("ERROR: Parse Error");
        }
        print!("> ");
        stdout.lock().flush().expect("Failed to flush");
    }
}
