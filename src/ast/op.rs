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

#[derive(PartialEq)]
enum BinOp<'a> {
    // ArithmeticOps
    Add(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Sub(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Mul(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Div(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Exp(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    FDiv(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Mod(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),

    // LogicOps
    And(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Or(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),

    // RelationalOps
    Lt(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Le(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Gt(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Ge(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Eq(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Ne(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),

    // BinaryOps
    BitOr(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    BitAnd(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    BitXor(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Rsh(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
    Lsh(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),

    // ConcatenationOps
    Concat(&'a mut ASTNode<'a>, &'a mut ASTNode<'a>),
}

impl<'a> Clone for BinOp<'a> {
    fn clone(&self) -> BinOp<'a> {
        use self::BinOp::*;
        match *self {
            // ArithmeticOps
            Add(astl, astr) => Add(&mut astl.clone(), &mut astr.clone()),
            Sub(astl, astr) => Sub(&mut astl.clone(), &mut astr.clone()),
            Mul(astl, astr) => Mul(&mut astl.clone(), &mut astr.clone()),
            Div(astl, astr) => Div(&mut astl.clone(), &mut astr.clone()),
            Exp(astl, astr) => Exp(&mut astl.clone(), &mut astr.clone()),
            FDiv(astl, astr) => FDiv(&mut astl.clone(), &mut astr.clone()),
            Mod(astl, astr) => Mod(&mut astl.clone(), &mut astr.clone()),

            // LogicOps
            And(astl, astr) => And(&mut astl.clone(), &mut astr.clone()),
            Or(astl, astr) => Or(&mut astl.clone(), &mut astr.clone()),

            // RelationalOps
            Lt(astl, astr) => Lt(&mut astl.clone(), &mut astr.clone()),
            Le(astl, astr) => Le(&mut astl.clone(), &mut astr.clone()),
            Gt(astl, astr) => Gt(&mut astl.clone(), &mut astr.clone()),
            Ge(astl, astr) => Ge(&mut astl.clone(), &mut astr.clone()),
            Eq(astl, astr) => Eq(&mut astl.clone(), &mut astr.clone()),
            Ne(astl, astr) => Ne(&mut astl.clone(), &mut astr.clone()),

            // BinaryOps
            BitOr(astl, astr) => BitOr(&mut astl.clone(), &mut astr.clone()),
            BitAnd(astl, astr) => BitAnd(&mut astl.clone(), &mut astr.clone()),
            BitXor(astl, astr) => BitXor(&mut astl.clone(), &mut astr.clone()),
            Rsh(astl, astr) => Rsh(&mut astl.clone(), &mut astr.clone()),
            Lsh(astl, astr) => Lsh(&mut astl.clone(), &mut astr.clone()),

            // ConcatenationOps
            Concat(astl, astr) => Concat(&mut astl.clone(), &mut astr.clone()),
        }
    }
}

impl<'a> Debug for BinOp<'a> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::BinOp::*;
        match *self {
            // ArithmeticOps
            Add(ref left, ref right) => write!(format, "({} + {})", left, right),
            Sub(ref left, ref right) => write!(format, "({} - {})", left, right),
            Mul(ref left, ref right) => write!(format, "({} * {})", left, right),
            Div(ref left, ref right) => write!(format, "({} / {})", left, right),
            Exp(ref left, ref right) => write!(format, "({} ^ {})", left, right),
            FDiv(ref left, ref right) => write!(format, "({} // {})", left, right),
            Mod(ref left, ref right) => write!(format, "({} % {})", left, right),

            // LogicOps
            And(ref left, ref right) => write!(format, "({} and {})", left, right),
            Or(ref left, ref right) => write!(format, "({} or {})", left, right),

            // ArithmeticOps
            Lt(ref left, ref right) => write!(format, "({} < {})", left, right),
            Le(ref left, ref right) => write!(format, "({} <= {})", left, right),
            Gt(ref left, ref right) => write!(format, "({} > {})", left, right),
            Ge(ref left, ref right) => write!(format, "({} >= {})", left, right),
            Eq(ref left, ref right) => write!(format, "({} == {})", left, right),
            Ne(ref left, ref right) => write!(format, "({} ~= {})", left, right),

            // BinaryOps
            BitAnd(ref left, ref right) => write!(format, "({} & {})", left, right),
            BitOr(ref left, ref right) => write!(format, "({} | {})", left, right),
            BitXor(ref left, ref right) => write!(format, "({} ~ {})", left, right),
            Rsh(ref left, ref right) => write!(format, "({} >> {})", left, right),
            Lsh(ref left, ref right) => write!(format, "({} << {})", left, right),


            // ConcatenationOps
            Concat(ref left, ref right) => write!(format, "{} .. {}", left, right),
        }
    }
}

#[derive(PartialEq)]
enum UnOp<'a> {
    // UnaryOps
    BinNot(&'a mut ASTNode<'a>),
    Not(&'a mut ASTNode<'a>),
    Len(&'a mut ASTNode<'a>),
    UMin(&'a mut ASTNode<'a>),
}

impl<'a> Clone for UnOp<'a> {
    fn clone(&self) -> UnOp<'a> {
        use self::UnOp::*;
        match *self {
            BinNot(ast) => BinNot(&mut ast.clone()),
            Not(ast) => Not(&mut ast.clone()),
            Len(ast) => Len(&mut ast.clone()),
            UMin(ast) => UMin(&mut ast.clone()),
        }
    }
}

impl<'a> Debug for UnOp<'a> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::UnOp::*;
        match *self {
            // UnaryOps
            BinNot(ref right) => write!(format, "~{}", right),
            Len(ref right) => write!(format, "#{}", right),
            UMin(ref right) => write!(format, "-{}", right),
            Not(ref right) => write!(format, "not {}", right),
        }
    }
}
