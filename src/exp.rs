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

use nom::{digit};
use std::str;
use std::str::FromStr;

use ast::ASTNode;

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

named!(pub exp< ASTNode >, ws!(parse_or));

named!(parens< ASTNode >,
       delimited!(
            ws!(tag!("(")),
            map!(map!(exp, Box::new), ASTNode::Paren),
            ws!(tag!(")"))
       ));

named!(factor< ASTNode >,
       alt_complete!(
           map!(
               map_res!(
                   map_res!(
                       ws!(digit),
                   str::from_utf8),
               FromStr::from_str),
           ASTNode::Value)
       |
           parens
       ));

named!(parse_exp< ASTNode >, do_parse!(
    initial: factor >>
    remainder: many0!(
             do_parse!(tag!("^") >> exp: factor >> (ArithmeticOps::Exp, exp))
         ) >> (fold_arithmetic_ops(initial, remainder))
));

// TODO: Unary operators go here (not # - ~)

named!(term< ASTNode >, do_parse!(
    initial: parse_exp >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("*") >> mul: parse_exp >> (ArithmeticOps::Mul, mul)) |
             do_parse!(tag!("//") >> fdiv: parse_exp >> (ArithmeticOps::FDiv, fdiv)) |
             do_parse!(tag!("%") >> lmod: parse_exp >> (ArithmeticOps::Mod, lmod)) |
             do_parse!(tag!("/") >> div: parse_exp >> (ArithmeticOps::Div, div))
           )
         ) >> (fold_arithmetic_ops(initial, remainder))
));

named!(expr< ASTNode >, do_parse!(
    initial: term >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("+") >> add: term >> (ArithmeticOps::Add, add)) |
            do_parse!(tag!("-") >> sub: term >> (ArithmeticOps::Sub, sub))
        )
    ) >> (fold_arithmetic_ops(initial, remainder))
));

named!(parse_concat< ASTNode >, do_parse!(
    initial: expr >>
    remainder: many0!(
            do_parse!(tag!("..") >> cc: expr >> (ConcatenationOps::Concat, cc))
    ) >> (fold_concat_ops(initial, remainder))
));

named!(parse_bsh< ASTNode >, do_parse!(
    initial: parse_concat >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("<<") >> lsh: parse_concat >> (BinaryOps::Lsh, lsh)) |
            do_parse!(tag!(">>") >> rsh: parse_concat >> (BinaryOps::Rsh, rsh))
        )
    ) >> (fold_binary_ops(initial, remainder))
));

named!(parse_band< ASTNode >, do_parse!(
    initial: parse_bsh >>
    remainder: many0!(
            do_parse!(tag!("&") >> band: parse_bsh >> (BinaryOps::BitAnd, band))
    ) >> (fold_binary_ops(initial, remainder))
));

named!(parse_bxor< ASTNode >, do_parse!(
    initial: parse_band >>
    remainder: many0!(
            do_parse!(tag!("~") >> bxor: parse_band >> (BinaryOps::BitXor, bxor))
    ) >> (fold_binary_ops(initial, remainder))
));

named!(parse_bor< ASTNode >, do_parse!(
    initial: parse_bxor >>
    remainder: many0!(
            do_parse!(tag!("|") >> bor: parse_bxor >> (BinaryOps::BitOr, bor))
    ) >> (fold_binary_ops(initial, remainder))
));

named!(parse_cmp< ASTNode >, do_parse!(
    initial: parse_bor >>
    remainder: many0!(
        alt!(
            do_parse!(tag!("<")  >> lt: parse_bor >> (RelationalOps::Lt, lt)) |
            do_parse!(tag!("<=") >> le: parse_bor >> (RelationalOps::Le, le)) |
            do_parse!(tag!(">")  >> gt: parse_bor >> (RelationalOps::Gt, gt)) |
            do_parse!(tag!(">=") >> ge: parse_bor >> (RelationalOps::Ge, ge)) |
            do_parse!(tag!("==") >> eq: parse_bor >> (RelationalOps::Eq, eq)) |
            do_parse!(tag!("~=") >> ne: parse_bor >> (RelationalOps::Ne, ne))
        )
    ) >> (fold_relational_ops(initial, remainder))
));

named!(parse_and< ASTNode >, do_parse!(
    initial: parse_cmp >>
    remainder: many0!(
             do_parse!(tag!("and") >> and: parse_cmp >> (LogicOps::And, and))
         ) >> (fold_logic_ops(initial, remainder))
));

named!(parse_or< ASTNode >, do_parse!(
    initial: parse_and >>
    remainder: many0!(
             do_parse!(tag!("or") >> or: parse_and >> (LogicOps::Or, or))
         ) >> (fold_logic_ops(initial, remainder))
));

