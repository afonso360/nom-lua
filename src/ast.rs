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

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Paren(Box<ASTNode>),

    // ArithmeticOps
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    Exp(Box<ASTNode>, Box<ASTNode>),
    FDiv(Box<ASTNode>, Box<ASTNode>),
    Mod(Box<ASTNode>, Box<ASTNode>),

    // LogicOps
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),

    // RelationalOps
    Lt(Box<ASTNode>, Box<ASTNode>),
    Le(Box<ASTNode>, Box<ASTNode>),
    Gt(Box<ASTNode>, Box<ASTNode>),
    Ge(Box<ASTNode>, Box<ASTNode>),
    Eq(Box<ASTNode>, Box<ASTNode>),
    Ne(Box<ASTNode>, Box<ASTNode>),

    // BinaryOps
    BitOr(Box<ASTNode>, Box<ASTNode>),
    BitAnd(Box<ASTNode>, Box<ASTNode>),
    BitXor(Box<ASTNode>, Box<ASTNode>),
    Rsh(Box<ASTNode>, Box<ASTNode>),
    Lsh(Box<ASTNode>, Box<ASTNode>),

    // UnaryOps
    BinNot(Box<ASTNode>),
    Not(Box<ASTNode>),
    Len(Box<ASTNode>),
    UMin(Box<ASTNode>),

    // ConcatenationOps
    Concat(Box<ASTNode>, Box<ASTNode>),

    // Expression
    Nil,
    VarArg,
    TableConstructor(Box<ASTNode>),

    // Function
    Function(Box<ASTNode>, Box<ASTNode>),
}

impl Display for ASTNode {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::ASTNode::*;
        match *self {
            Integer(val) => write!(format, "{}", val),
            Float(val) => write!(format, "{}f", val),
            Bool(val) => write!(format, "{}", val),
            Paren(ref expr) => write!(format, "({})", expr),

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

            // UnaryOps
            BinNot(ref right) => write!(format, "~{}", right),
            Len(ref right) => write!(format, "#{}", right),
            UMin(ref right) => write!(format, "-{}", right),
            Not(ref right) => write!(format, "not {}", right),

            // ConcatenationOps
            Concat(ref left, ref right) => write!(format, "{} .. {}", left, right),

            // Exp
            Nil => write!(format, "nil"),
            VarArg => write!(format, "..."),
            TableConstructor(ref fieldlist) => write!(format, "{{ {} }}", fieldlist),

            //Function
            Function(ref parlist, ref fbody) => write!(format, "function ({}) {}", parlist, fbody),
        }
    }
}

