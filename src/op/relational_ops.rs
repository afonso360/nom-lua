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

use super::parse_binary_or;
use super::super::ast::ASTNode;

#[derive(Debug)]
pub enum RelationalOps {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

fn fold_relational_ops(initial: ASTNode, remainder: Vec<(RelationalOps, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(initial, |acc, pair|{
        let (ops, expr) = pair;
        match ops {
            RelationalOps::Lt => ASTNode::Lt(Box::new(acc), Box::new(expr)),
            RelationalOps::Le => ASTNode::Le(Box::new(acc), Box::new(expr)),
            RelationalOps::Gt => ASTNode::Gt(Box::new(acc), Box::new(expr)),
            RelationalOps::Ge => ASTNode::Ge(Box::new(acc), Box::new(expr)),
            RelationalOps::Eq => ASTNode::Eq(Box::new(acc), Box::new(expr)),
            RelationalOps::Ne => ASTNode::Ne(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(pub parse_relational_ops< ASTNode >, do_parse!(
    initial: parse_binary_or >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("<")  >> lt: parse_binary_or >> (RelationalOps::Lt, lt)) |
            do_parse!(tag!("<=") >> le: parse_binary_or >> (RelationalOps::Le, le)) |
            do_parse!(tag!(">")  >> gt: parse_binary_or >> (RelationalOps::Gt, gt)) |
            do_parse!(tag!(">=") >> ge: parse_binary_or >> (RelationalOps::Ge, ge)) |
            do_parse!(tag!("==") >> eq: parse_binary_or >> (RelationalOps::Eq, eq)) |
            do_parse!(tag!("~=") >> ne: parse_binary_or >> (RelationalOps::Ne, ne))
        )
    ) >> (fold_relational_ops(initial, remainder))
));

