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

use super::parse_addsub;
use super::super::ast::ASTNode;

#[derive(Debug)]
pub enum ConcatenationOps {
    Concat,
}

fn fold_concat_ops(initial: ASTNode, remainder: Vec<(ConcatenationOps, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(initial, |acc, pair|{
        let (ops, expr) = pair;
        match ops {
            ConcatenationOps::Concat => ASTNode::Concat(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(pub parse_concat< ASTNode >, do_parse!(
    initial: parse_addsub >>
    remainder: many0!(
            do_parse!(tag!("..") >> cc: parse_addsub >> (ConcatenationOps::Concat, cc))
    ) >> (fold_concat_ops(initial, remainder))
));
