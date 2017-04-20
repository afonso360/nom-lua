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

use super::factor;
use super::super::ast::ASTNode;


#[derive(Debug)]
pub enum UnaryOps {
    BinNot,
    Not,
    Len,
    UMin,
}

fn fold_unary_ops(ops: Option<UnaryOps>, num: ASTNode) -> ASTNode {
        if let Some(op) = ops {
            match op {
                UnaryOps::BinNot => ASTNode::BinNot(Box::new(num)),
                UnaryOps::Not => ASTNode::Not(Box::new(num)),
                UnaryOps::Len => ASTNode::Len(Box::new(num)),
                UnaryOps::UMin => ASTNode::UMin(Box::new(num)),
            }
        } else {
            num
        }
}


//named!(pub parse_unop< ASTNode >, do_parse!(
//    ops: alt!( do_parse!(opt!(tag!("~")) >> (UnaryOps::BinNot)) |
//               do_parse!(opt!(tag!("not")) >> (UnaryOps::Not)) |
//               do_parse!(opt!(tag!("#")) >> (UnaryOps::Len)) |
//               do_parse!(opt!(tag!("-")) >> (UnaryOps::UMin))
//           ) >>
//    num: factor
//>> (fold_unary_ops(ops, num))
//));

