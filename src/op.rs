// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//mod relational_ops;
//mod arithmetic_ops;
//mod binary_ops;
//mod unary_ops;
//mod logic_ops;
//mod concat_ops;

use std::str;

use ast::{ ASTNode, UnOp, BinOp };
use super::exp::parse_exp;
use super::number::parse_number;

//named!(pub parse_op<ASTNode>, dbg_dmp!(alt!(parse_exponent | parse_unop | parse_binop)));
// for some reason we have to use the map
//named!(pub parse_op<ASTNode>, map!(parse_exponent, |a|a));
named!(pub parse_op<ASTNode>, do_parse!(tag!("") >> (ASTNode::UnOp(astmr!(UnOp::Not, ASTNode::Integer(10))))));

/*
named!(parse_unop<ASTNode>, map!(do_parse!(
           unop: many0!(unop)
        >> right: parse_binop
        >> (fold_unop(unop, right))),
        ASTNode::UnOp
));

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

fn fold_unop<'a>(unop: Vec<ASTNode<'a>>, initial: ASTNode<'a>) -> UnOp<'a> {
    unop.into_iter().fold(initial, |acc, op| {
        println!("Proc unop: {:?}", op);
        match op {
            UnOp::BinNot => astmr!(UnOp::BinNot, acc),
            UnOp::Not => astmr!(UnOp::Not, acc),
            UnOp::Len => astmr!(UnOp::Len, acc),
            UnOp::UMin => astmr!(UnOp::UMin, acc),
        }
    })
}

fn fold_binop<'a>(left: ASTNode<'a>, remainder: Vec<(BinOp<'a>, ASTNode<'a>)>) -> BinOp<'a> {
    remainder.into_iter().fold(left, |acc, pair| {
        let (op, right) = pair;
        println!("Proc binop: {:?}", op);
        match op {
            //TODO: This is right Associative, we need to invert this operation
            Exp => astmr!(Exp, acc, right),
            Mul => astmr!(Mul, acc, right),
            Div => astmr!(Div, acc, right),
            FDiv => astmr!(FDiv, acc, right),
            Mod => astmr!(Mod, acc, right),
            Add => astmr!(Add, acc, right),
            Sub => astmr!(Sub, acc, right),
            //TODO: This is right Associative, we need to invert this operation
            Concat => astmr!(Concat, acc, right),
            Lsh => astmr!(Lsh, acc, right),
            Rsh => astmr!(Rsh, acc, right),
            BitAnd => astmr!(BitAnd, acc, right),
            BitXor => astmr!(BitXor, acc, right),
            BitOr => astmr!(BitOr, acc, right),
            Lt => astmr!(Lt, acc, right),
            Gt => astmr!(Gt, acc, right),
            Le => astmr!(Le, acc, right),
            Ge => astmr!(Ge, acc, right),
            Ne => astmr!(Ne, acc, right),
            Eq => astmr!(Eq, acc, right),
            And => astmr!(And, acc, right),
            Or => astmr!(Or, acc, right),
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
*/
