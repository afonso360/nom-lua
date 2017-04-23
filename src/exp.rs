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


use ast::ASTNode;
use ast::ASTNode::*;

use number::parse_number;
use op::parse_op;
use string::parse_string;
use function::parse_functiondef;
use field::parse_fieldlist;

named!(parse_vararg<ASTNode>, map!(tag!("..."), |_| ast!(VarArg)));
named!(parse_nil<ASTNode>, map!(tag!("nil"), |_| ast!(Nil)));
named!(parse_bool<ASTNode>, alt!(map!(tag!("false"), |_| ast!(Bool, false)) |
                                 map!(tag!("true"), |_| ast!(Bool, true))));

named!(parse_prefixexp<ASTNode>, alt!(
        //parse_var |
        //parse_functioncall |
        delimited!(tag!("("), ws!(parse_exp), tag!(")"))
));

named!(pub parse_explist<ASTNode>, map!(map!(
            map!(do_parse!(
                   a: parse_exp
                >> b: many0!(preceded!(ws!(tag!(",")), parse_exp))
                >> ((a,b))
            ), |(a, mut b): (_, Vec < ASTNode >) | { b.insert(0, a); b }),
Box::new), ASTNode::ExpList));

named!(pub parse_exp<ASTNode>, alt!(
                parse_op |
                parse_nil |
                parse_bool |
                parse_string |
                parse_vararg |
                parse_functiondef |
                parse_prefixexp |
                parse_tableconstructor
));

// TODO: Missing tests
named!(parse_tableconstructor<ASTNode>,
       map!(
       do_parse!(
              tag!("{")
           >> f: ws!(opt!(parse_fieldlist))
           >> tag!("}")
           >> (Box::new(f))), ASTNode::TableConstructor));



#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_test!(parse_nil, parse_nil, "nil", ast!(Nil));
    ast_test!(parse_bool_t, parse_bool, "true", ast!(Bool, true));
    ast_test!(parse_bool_f, parse_bool, "false", ast!(Bool, false));
    ast_test!(parse_vararg, parse_vararg, "...", ast!(VarArg));

    ast_test!(parse_explist_1, parse_explist, "true", astb!(ExpList, vec![
        ast!(Bool, true)
    ]));
    ast_test!(parse_explist_2, parse_explist, "true , true", astb!(ExpList, vec![
        ast!(Bool, true),
        ast!(Bool, true)
    ]));
    ast_test!(parse_explist_3, parse_explist, "true , false, false", astb!(ExpList, vec![
        ast!(Bool, true),
        ast!(Bool, false),
        ast!(Bool, false)
    ]));
}
