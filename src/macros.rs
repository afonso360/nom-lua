// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


macro_rules! ast_panic_test {
    ($name: ident, $func: ident, $input: expr) => {
        #[test]
        #[should_panic]
        fn $name () {
            use super::*;
            $func($input.as_bytes()).unwrap().1;
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

macro_rules! astb {
    ($name: ident, $($a: expr),*) => {
        $name($(Box::new($a)),*)
    };
}

macro_rules! astmr {
    ($name: path, $($a: expr),*) => {
        $name($(&mut $a),*)
    };
}

macro_rules! ast {
    ($name: ident) => {
        $name
    };
    ($name: ident, $($a: expr),*) => {
        $name($($a),*)
    };
}

