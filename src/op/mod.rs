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

mod relational_ops;
mod arithmetic_ops;
mod binary_ops;
mod unary_ops;
mod logic_ops;
mod concat_ops;

use nom::{digit};
use std::str;
use std::str::FromStr;

use ast::ASTNode;

use self::relational_ops::parse_relational_ops;
use self::arithmetic_ops::{parse_addsub, parse_exponent};
use self::binary_ops::{parse_binary_or};
use self::logic_ops::{parse_logic_or};
use self::concat_ops::{parse_concat};

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
named!(pub parse_op< ASTNode >, ws!(parse_logic_or));

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

// TODO: Build tests for all elements in this crate (see nom brainfuck parser)
