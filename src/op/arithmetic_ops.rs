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
pub enum ArithmeticOps {
    Add,
    Sub,
    Mul,
    Div,
    FDiv,
    Mod,
    Exp,
}

fn fold_arithmetic_ops(initial: ASTNode, remainder: Vec<(ArithmeticOps, ASTNode)>) -> ASTNode {
    remainder.into_iter().fold(initial, |acc, pair|{
        let (ops, expr) = pair;
        match ops {
            ArithmeticOps::Add => ASTNode::Add(Box::new(acc), Box::new(expr)),
            ArithmeticOps::Sub => ASTNode::Sub(Box::new(acc), Box::new(expr)),
            ArithmeticOps::Mul => ASTNode::Mul(Box::new(acc), Box::new(expr)),
            ArithmeticOps::Div => ASTNode::Div(Box::new(acc), Box::new(expr)),
            ArithmeticOps::Exp => ASTNode::Exp(Box::new(acc), Box::new(expr)),
            ArithmeticOps::FDiv => ASTNode::FDiv(Box::new(acc), Box::new(expr)),
            ArithmeticOps::Mod => ASTNode::Mod(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(pub parse_exponent< ASTNode >, do_parse!(
    initial: factor >>
    remainder: many0!(
             do_parse!(tag!("^") >> exp: factor >> (ArithmeticOps::Exp, exp))
         ) >> (fold_arithmetic_ops(initial, remainder))
));
//TODO: Below should be parse_unop instead of exponent
named!(pub parse_multdiv< ASTNode >, do_parse!(
    initial: parse_exponent >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("*") >> mul: parse_exponent >> (ArithmeticOps::Mul, mul)) |
             do_parse!(tag!("//") >> fdiv: parse_exponent >> (ArithmeticOps::FDiv, fdiv)) |
             do_parse!(tag!("%") >> lmod: parse_exponent >> (ArithmeticOps::Mod, lmod)) |
             do_parse!(tag!("/") >> div: parse_exponent >> (ArithmeticOps::Div, div))
           )
         ) >> (fold_arithmetic_ops(initial, remainder))
));

named!(pub parse_addsub< ASTNode >, do_parse!(
    initial: parse_multdiv >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("+") >> add: parse_multdiv >> (ArithmeticOps::Add, add)) |
            do_parse!(tag!("-") >> sub: parse_multdiv >> (ArithmeticOps::Sub, sub))
        )
    ) >> (fold_arithmetic_ops(initial, remainder))
));

