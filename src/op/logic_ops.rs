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

use super::parse_relational_ops;
use super::super::ast::ASTNode;

#[derive(Debug)]
pub enum LogicOps {
    Or,
    And,
}

fn fold_logic_ops(initial: ASTNode, remainder: Vec<(LogicOps, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(initial, |acc, pair|{
        let (ops, expr) = pair;
        match ops {
            LogicOps::And => ASTNode::And(Box::new(acc), Box::new(expr)),
            LogicOps::Or => ASTNode::Or(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(pub parse_logic_and< ASTNode >, do_parse!(
    initial: parse_relational_ops >>
    remainder: many0!(
             do_parse!(tag!("and") >> and: parse_relational_ops >> (LogicOps::And, and))
         ) >> (fold_logic_ops(initial, remainder))
));

named!(pub parse_logic_or< ASTNode >, do_parse!(
    initial: parse_logic_and >>
    remainder: many0!(
             do_parse!(tag!("or") >> or: parse_logic_and >> (LogicOps::Or, or))
         ) >> (fold_logic_ops(initial, remainder))
));

