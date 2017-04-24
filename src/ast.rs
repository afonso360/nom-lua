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
    String(String),
    Label(String),
    Name(String),
    Paren(Box<ASTNode>),

    Block(Box<Vec<ASTNode>>, Box<Option<ASTNode>>),

    //Statements
    EmptyStatement,
    Break,
    Goto(Box<ASTNode>),
    RetStat(Box<Option<ASTNode>>),

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
    TableConstructor(Box<Option<ASTNode>>),

    // Function
    /// Takes a FunctionBody
    Function(Box<ASTNode>),
    /// Takes a ParameterList and a Block
    FunctionBody(Box<Option<ASTNode>>, Box<ASTNode>),
    /// Has 3 parameters
    /// the example: log.ms:al
    /// would produce
    /// Name log
    /// Name ms
    /// Name al
    FunctionName(Box<ASTNode>, Option<Box<Vec<ASTNode>>>, Option<Box<ASTNode>>),
    /// Takes a Name and a FunctionBody
    NamedFunction(Box<ASTNode>, Box<ASTNode>),

    // Lists
    ExpList(Box<Vec<ASTNode>>),
    VarList(Box<Vec<ASTNode>>),
    NameList(Box<Vec<ASTNode>>),
    FieldList(Box<Vec<ASTNode>>),
    ParameterList(Box<Option<ASTNode>>, bool),

    // Field
    /// Contains an expr
    FieldSingle(Box<ASTNode>),
    /// The first node may be an expr to be resolved or a Name
    /// The second node is the assigned expr
    FieldAssign(Box<ASTNode>, Box<ASTNode>),

    // Local
    Local(Box<ASTNode>),
}

impl Display for ASTNode {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::ASTNode::*;
        match *self {
            Integer(val) => write!(format, "{}", val),
            Float(val) => write!(format, "{}f", val),
            Bool(val) => write!(format, "{}", val),
            String(ref val) => write!(format, "\"{}\"", val),

            /// Holds a lua name, usually a function or variable name
            /// Contains `ASTNode::String`
            Name(ref val) => write!(format, "(name {})", val),

            /// Holds a lua label name
            /// Contains `ASTNode::Name`
            Label(ref val) => write!(format, "::{}::", val),
            /// Contains an expression
            Paren(ref expr) => write!(format, "({})", expr),

            // Block
            Block(ref statements, ref retstat) => write!(format, "(block)"),

            // Statements
            EmptyStatement => write!(format, "(statement)"),
            RetStat(ref para) => write!(format, "(ret {:?})", para),
            Break => write!(format, "(break)"),
            Goto(ref loc) => write!(format, "goto {}", loc),

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
            //TODO: Remove this debug impl
            TableConstructor(ref fieldlist) => write!(format, "{{ {:?} }}", fieldlist),

            //Function
            Function(ref f) => write!(format, "{}", f),
            FunctionBody(ref parlist, ref fbody) => write!(format, "function ({:?}) {}", parlist, fbody),
            FunctionName(ref n, ref m, ref f) => write!(format, "{}.{:?}:{:?}", n, m, f),
            NamedFunction(ref n, ref f) => write!(format, "(named {} {})", n, f),

            //TODO: Make this actually print thecontents
            ExpList(ref explist) => write!(format, "(explist)"),
            VarList(ref varlist) => write!(format, "(varlist)"),
            NameList(ref namelist) => write!(format, "(namelist)"),
            ParameterList(ref plist, ref va) => write!(format, "(parameterlist, vararg: {})", va),
            FieldList(ref namelist) => write!(format, "(fieldlist)"),

            // Field
            FieldSingle(ref e) => write!(format, "(field {})", e),
            FieldAssign(ref n, ref e) => write!(format, "(field {} => {})", n, e),

            //Local
            Local(ref inner) => write!(format, "local {}", inner),
        }
    }
}
