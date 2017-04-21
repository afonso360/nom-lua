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

use nom::{digit};
use std::str;
use std::str::FromStr;

use ast::ASTNode;
use super::exp::parse_exp;

//use self::relational_ops::parse_relational_ops;
//use self::arithmetic_ops::{parse_addsub, parse_exponent};
//use self::binary_ops::{parse_binary_or};
//use self::logic_ops::{parse_logic_or};
//use self::concat_ops::{parse_concat};

/// Here is the call chain for the exp parser
/// fold_arithmetic_ops ( ^ )
/// fold_unary_ops ( all of them )
/// fold_arithmetic_ops ( * // % / )
/// fold_arithmetic_ops ( + - )
/// fold_concat_ops ( + - )
/// fold_binary_ops ( << >> )
/// fold_binary_ops ( & )
/// fold_binary_ops ( ~ )
/// fold_binary_ops ( | )
/// parse_relational_ops ( < <= > >= == ~= )
/// logic_ops ( and )
/// logic_ops ( or )

//This is marked just for convenience so users know where to enter
named!(pub parse_op<ASTNode>, dbg!(alt!(parse_unop | parse_binop)));

fn fold_binop(bop: BinOp, left: ASTNode, right: ASTNode) -> ASTNode {
    match bop {
        BinOp::Exp => ASTNode::Exp(Box::new(left), Box::new(right)),
        BinOp::Mul => ASTNode::Mul(Box::new(left), Box::new(right)),
        BinOp::Div => ASTNode::Div(Box::new(left), Box::new(right)),
        BinOp::FDiv => ASTNode::FDiv(Box::new(left), Box::new(right)),
        BinOp::Mod => ASTNode::Mod(Box::new(left), Box::new(right)),
        BinOp::Add => ASTNode::Add(Box::new(left), Box::new(right)),
        BinOp::Sub => ASTNode::Sub(Box::new(left), Box::new(right)),
        BinOp::Concat => ASTNode::Concat(Box::new(left), Box::new(right)),
        BinOp::Lsh => ASTNode::Lsh(Box::new(left), Box::new(right)),
        BinOp::Rsh => ASTNode::Rsh(Box::new(left), Box::new(right)),
        BinOp::BitAnd => ASTNode::BitAnd(Box::new(left), Box::new(right)),
        BinOp::BitXor => ASTNode::BitXor(Box::new(left), Box::new(right)),
        BinOp::BitOr => ASTNode::BitOr(Box::new(left), Box::new(right)),
        BinOp::Lt => ASTNode::Lt(Box::new(left), Box::new(right)),
        BinOp::Gt => ASTNode::Gt(Box::new(left), Box::new(right)),
        BinOp::Le => ASTNode::Le(Box::new(left), Box::new(right)),
        BinOp::Ge => ASTNode::Ge(Box::new(left), Box::new(right)),
        BinOp::Ne => ASTNode::Ne(Box::new(left), Box::new(right)),
        BinOp::Eq => ASTNode::Eq(Box::new(left), Box::new(right)),
        BinOp::And => ASTNode::And(Box::new(left), Box::new(right)),
        BinOp::Or => ASTNode::Or(Box::new(left), Box::new(right)),
    }
}

named!(pub parse_binop<ASTNode>, do_parse!(
        left: factor >>
        bop: binop >>
        right: factor >> (fold_binop(bop, left, right))));

named!(binop<BinOp>, alt!(
    ws!(tag!("^"))   => { |_| BinOp::Exp } |
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


named!(parse_unop<ASTNode>, do_parse!(
        unop: unop >>
        right: factor >> (fold_unop(unop, right))));


fn fold_unop(unop: UnOp, right: ASTNode) -> ASTNode {
    match unop {
        UnOp::BinNot => ASTNode::BinNot(Box::new(right)),
        UnOp::Not => ASTNode::Not(Box::new(right)),
        UnOp::Len => ASTNode::Len(Box::new(right)),
        UnOp::UMin => ASTNode::UMin(Box::new(right)),
    }
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
    BinNot
}


named!(pub parens< ASTNode >,
       delimited!(
            ws!(tag!("(")),
            map!(map!(parse_op, Box::new), ASTNode::Paren),
            ws!(tag!(")"))
       ));

named!(pub factor< ASTNode >,
       alt_complete!(
           map!(
               map_res!(
                   map_res!(
                       ws!(digit),
                   str::from_utf8),
               FromStr::from_str),
           ASTNode::Integer)
       |
           parens
       ));
