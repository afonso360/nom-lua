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

#[derive(Debug, PartialEq)]
enum BinOp<'a> {
    // ArithmeticOps
    Add(&'a mut ASTNode, &'a mut ASTNode),
    Sub(&'a mut ASTNode, &'a mut ASTNode),
    Mul(&'a mut ASTNode, &'a mut ASTNode),
    Div(&'a mut ASTNode, &'a mut ASTNode),
    Exp(&'a mut ASTNode, &'a mut ASTNode),
    FDiv(&'a mut ASTNode, &'a mut ASTNode),
    Mod(&'a mut ASTNode, &'a mut ASTNode),

    // LogicOps
    And(&'a mut ASTNode, &'a mut ASTNode),
    Or(&'a mut ASTNode, &'a mut ASTNode),

    // RelationalOps
    Lt(&'a mut ASTNode, &'a mut ASTNode),
    Le(&'a mut ASTNode, &'a mut ASTNode),
    Gt(&'a mut ASTNode, &'a mut ASTNode),
    Ge(&'a mut ASTNode, &'a mut ASTNode),
    Eq(&'a mut ASTNode, &'a mut ASTNode),
    Ne(&'a mut ASTNode, &'a mut ASTNode),

    // BinaryOps
    BitOr(&'a mut ASTNode, &'a mut ASTNode),
    BitAnd(&'a mut ASTNode, &'a mut ASTNode),
    BitXor(&'a mut ASTNode, &'a mut ASTNode),
    Rsh(&'a mut ASTNode, &'a mut ASTNode),
    Lsh(&'a mut ASTNode, &'a mut ASTNode),

    // ConcatenationOps
    Concat(&'a mut ASTNode, &'a mut ASTNode),
}

#[derive(Debug, PartialEq)]
enum UnOp<'a> {
    // UnaryOps
    BinNot(&'a mut ASTNode),
    Not(&'a mut ASTNode),
    Len(&'a mut ASTNode),
    UMin(&'a mut ASTNode),
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
