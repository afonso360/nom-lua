// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use super::ASTNode;
use std::clone::Clone;

#[derive(PartialEq, Clone)]
pub enum BinOp<'a> {
    // ArithmeticOps
    Add(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Sub(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Mul(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Div(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Exp(&'a ASTNode<'a>, &'a ASTNode<'a>),
    FDiv(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Mod(&'a ASTNode<'a>, &'a ASTNode<'a>),

    // LogicOps
    And(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Or(&'a ASTNode<'a>, &'a ASTNode<'a>),

    // RelationalOps
    Lt(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Le(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Gt(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Ge(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Eq(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Ne(&'a ASTNode<'a>, &'a ASTNode<'a>),

    // BinaryOps
    BitOr(&'a ASTNode<'a>, &'a ASTNode<'a>),
    BitAnd(&'a ASTNode<'a>, &'a ASTNode<'a>),
    BitXor(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Rsh(&'a ASTNode<'a>, &'a ASTNode<'a>),
    Lsh(&'a ASTNode<'a>, &'a ASTNode<'a>),

    // ConcatenationOps
    Concat(&'a ASTNode<'a>, &'a ASTNode<'a>),
}

impl<'a> Debug for BinOp<'a> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::BinOp::*;
        match *self {
            // ArithmeticOps
            Add(ref left, ref right) => write!(format, "({:?} + {:?})", left, right),
            Sub(ref left, ref right) => write!(format, "({:?} - {:?})", left, right),
            Mul(ref left, ref right) => write!(format, "({:?} * {:?})", left, right),
            Div(ref left, ref right) => write!(format, "({:?} / {:?})", left, right),
            Exp(ref left, ref right) => write!(format, "({:?} ^ {:?})", left, right),
            FDiv(ref left, ref right) => write!(format, "({:?} // {:?})", left, right),
            Mod(ref left, ref right) => write!(format, "({:?} % {:?})", left, right),

            // LogicOps
            And(ref left, ref right) => write!(format, "({:?} and {:?})", left, right),
            Or(ref left, ref right) => write!(format, "({:?} or {:?})", left, right),

            // ArithmeticOps
            Lt(ref left, ref right) => write!(format, "({:?} < {:?})", left, right),
            Le(ref left, ref right) => write!(format, "({:?} <= {:?})", left, right),
            Gt(ref left, ref right) => write!(format, "({:?} > {:?})", left, right),
            Ge(ref left, ref right) => write!(format, "({:?} >= {:?})", left, right),
            Eq(ref left, ref right) => write!(format, "({:?} == {:?})", left, right),
            Ne(ref left, ref right) => write!(format, "({:?} ~= {:?})", left, right),

            // BinaryOps
            BitAnd(ref left, ref right) => write!(format, "({:?} & {:?})", left, right),
            BitOr(ref left, ref right) => write!(format, "({:?} | {:?})", left, right),
            BitXor(ref left, ref right) => write!(format, "({:?} ~ {:?})", left, right),
            Rsh(ref left, ref right) => write!(format, "({:?} >> {:?})", left, right),
            Lsh(ref left, ref right) => write!(format, "({:?} << {:?})", left, right),


            // ConcatenationOps
            Concat(ref left, ref right) => write!(format, "{:?} .. {:?}", left, right),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum UnOp<'a> {
    // UnaryOps
    BinNot(&'a ASTNode<'a>),
    Not(&'a ASTNode<'a>),
    Len(&'a ASTNode<'a>),
    UMin(&'a ASTNode<'a>),
}

impl<'a> Debug for UnOp<'a> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::UnOp::*;
        match *self {
            // UnaryOps
            BinNot(ref right) => write!(format, "~{:?}", right),
            Len(ref right) => write!(format, "#{:?}", right),
            UMin(ref right) => write!(format, "-{:?}", right),
            Not(ref right) => write!(format, "not {:?}", right),
        }
    }
}
