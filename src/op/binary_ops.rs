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

use super::{parse_concat};
use super::super::ast::ASTNode;

#[derive(Debug)]
pub enum BinaryOps {
    BitOr,
    BitAnd,
    BitXor,
    Rsh,
    Lsh,
}

fn fold_binary_ops(initial: ASTNode, remainder: Vec<(BinaryOps, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(initial, |acc, pair|{
        let (ops, expr) = pair;
        match ops {
            BinaryOps::BitOr => ASTNode::BitOr(Box::new(acc), Box::new(expr)),
            BinaryOps::BitAnd => ASTNode::BitAnd(Box::new(acc), Box::new(expr)),
            BinaryOps::BitXor => ASTNode::BitXor(Box::new(acc), Box::new(expr)),
            BinaryOps::Rsh => ASTNode::Rsh(Box::new(acc), Box::new(expr)),
            BinaryOps::Lsh => ASTNode::Lsh(Box::new(acc), Box::new(expr)),
        }
    })
}



named!(pub parse_binary_shift< ASTNode >, do_parse!(
    initial: parse_concat >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("<<") >> lsh: parse_concat >> (BinaryOps::Lsh, lsh)) |
            do_parse!(tag!(">>") >> rsh: parse_concat >> (BinaryOps::Rsh, rsh))
        )
    ) >> (fold_binary_ops(initial, remainder))
));


named!(pub parse_binary_and< ASTNode >, do_parse!(
    initial: parse_binary_shift >>
    remainder: many0!(
            do_parse!(tag!("&") >> band: parse_binary_shift >> (BinaryOps::BitAnd, band))
    ) >> (fold_binary_ops(initial, remainder))
));

named!(pub parse_binary_xor< ASTNode >, do_parse!(
    initial: parse_binary_and >>
    remainder: many0!(
            do_parse!(tag!("~") >> bxor: parse_binary_and >> (BinaryOps::BitXor, bxor))
    ) >> (fold_binary_ops(initial, remainder))
));
named!(pub parse_binary_or< ASTNode >, do_parse!(
    initial: parse_binary_xor >>
    remainder: many0!(
            do_parse!(tag!("|") >> bor: parse_binary_xor >> (BinaryOps::BitOr, bor))
    ) >> (fold_binary_ops(initial, remainder))
));


