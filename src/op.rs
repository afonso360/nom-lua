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

//mod relational_ops;
//mod arithmetic_ops;
//mod binary_ops;
//mod unary_ops;
//mod logic_ops;
//mod concat_ops;

use std::str;

use ast::ASTNode;
use ast::ASTNode::*;
use super::exp::parse_exp;
use super::number::parse_number;

//named!(pub parse_op<ASTNode>, dbg_dmp!(alt!(parse_exponent | parse_unop | parse_binop)));
// for some reason we have to use the map
named!(pub parse_op<ASTNode>, map!(parse_exponent, |a|a));

named!(parse_unop<ASTNode>, do_parse!(
           unop: many0!(unop)
        >> right: parse_binop
        >> (fold_unop(unop, right))));

named!(parse_binop<ASTNode>, do_parse!(
           left: parse_atom
        >> right: many0!(do_parse!(bop: binop >> right: parse_atom >> (bop, right)))
        >> (fold_binop(left, right))));

named!(parse_exponent<ASTNode>, do_parse!(
           left: parse_unop
        >> right: many0!(do_parse!(xp: exponent >> right: parse_unop >> (xp, right)))
        >> (fold_binop(left, right))));

named!(parse_atom<ASTNode>, alt!(parse_number | delimited!(tag!("("), ws!(parse_exp), tag!(")"))));

named!(exponent<BinOp>, map!(ws!(tag!("^")), |_| BinOp::Exp));

named!(binop<BinOp>, alt!(
    ws!(tag!("*"))   => { |_| BinOp::Mul } |
    ws!(tag!("/"))   => { |_| BinOp::Div } |
    ws!(tag!("//"))  => { |_| BinOp::FDiv } |
    ws!(tag!("%"))   => { |_| BinOp::Mod } |
    ws!(tag!("+"))   => { |_| BinOp::Add } |
    ws!(tag!("-"))   => { |_| BinOp::Sub } |
    ws!(tag!(".."))  => { |_| BinOp::Concat } |
    ws!(tag!("<<"))  => { |_| BinOp::Lsh } |
    ws!(tag!(">>"))  => { |_| BinOp::Rsh } |
    ws!(tag!("&"))   => { |_| BinOp::BitAnd } |
    ws!(tag!("~"))   => { |_| BinOp::BitXor } |
    ws!(tag!("|"))   => { |_| BinOp::BitOr } |
    ws!(tag!("<"))   => { |_| BinOp::Lt } |
    ws!(tag!(">"))   => { |_| BinOp::Gt } |
    ws!(tag!("<="))  => { |_| BinOp::Le } |
    ws!(tag!(">="))  => { |_| BinOp::Ge } |
    ws!(tag!("~="))  => { |_| BinOp::Ne } |
    ws!(tag!("=="))  => { |_| BinOp::Eq } |
    ws!(tag!("and")) => { |_| BinOp::And } |
    ws!(tag!("or"))  => { |_| BinOp::Or }
));

#[derive(Debug)]
pub enum BinOp {
    Exp,
    Mul,
    Div,
    FDiv,
    Mod,
    Add,
    Sub,
    Concat,
    Lsh,
    Rsh,
    BitAnd,
    BitXor,
    BitOr,
    Lt,
    Gt,
    Le,
    Ge,
    Ne,
    Eq,
    And,
    Or,
}


fn fold_unop(unop: Vec<UnOp>, initial: ASTNode) -> ASTNode {
    unop.into_iter().fold(initial, |acc, op| {
        println!("Proc unop: {:?}", op);
        match op {
            UnOp::BinNot => astb!(BinNot, acc),
            UnOp::Not => astb!(Not, acc),
            UnOp::Len => astb!(Len, acc),
            UnOp::UMin => astb!(UMin, acc),
        }
    })
}

fn fold_binop(left: ASTNode, remainder: Vec<(BinOp, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(left, |acc, pair| {
        let (op, right) = pair;
        println!("Proc binop: {:?}", op);
        match op {
            //TODO: This is right Associative, we need to invert this operation
            BinOp::Exp => astb!(Exp, acc, right),
            BinOp::Mul => astb!(Mul, acc, right),
            BinOp::Div => astb!(Div, acc, right),
            BinOp::FDiv => astb!(FDiv, acc, right),
            BinOp::Mod => astb!(Mod, acc, right),
            BinOp::Add => astb!(Add, acc, right),
            BinOp::Sub => astb!(Sub, acc, right),
            //TODO: This is right Associative, we need to invert this operation
            BinOp::Concat => astb!(Concat, acc, right),
            BinOp::Lsh => astb!(Lsh, acc, right),
            BinOp::Rsh => astb!(Rsh, acc, right),
            BinOp::BitAnd => astb!(BitAnd, acc, right),
            BinOp::BitXor => astb!(BitXor, acc, right),
            BinOp::BitOr => astb!(BitOr, acc, right),
            BinOp::Lt => astb!(Lt, acc, right),
            BinOp::Gt => astb!(Gt, acc, right),
            BinOp::Le => astb!(Le, acc, right),
            BinOp::Ge => astb!(Ge, acc, right),
            BinOp::Ne => astb!(Ne, acc, right),
            BinOp::Eq => astb!(Eq, acc, right),
            BinOp::And => astb!(And, acc, right),
            BinOp::Or => astb!(Or, acc, right),
        }
    })
}


// TODO: Change to be just preceded by whitespace
named!(pub unop<UnOp>, alt!(
    ws!(tag!("not"))  => { |_| UnOp::Not } |
    ws!(tag!("#"))    => { |_| UnOp::Len } |
    ws!(tag!("-"))    => { |_| UnOp::UMin } |
    ws!(tag!("~"))    => { |_| UnOp::BinNot }
));

#[derive(Debug)]
pub enum UnOp {
    Not,
    Len,
    UMin,
    BinNot,
}
