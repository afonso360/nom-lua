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

use number::parse_number;
use op::parse_op;

named!(parse_vararg< ASTNode >, map!(tag!("..."), |_| ASTNode::VarArg));
named!(parse_nil< ASTNode >, map!(tag!("nil"), |_| ASTNode::Nil));
named!(parse_bool< ASTNode >, alt!(map!(tag!("false"), |_| ASTNode::Bool(false)) |
                                   map!(tag!("true"), |_| ASTNode::Bool(true))));

named!(parse_prefixexp< ASTNode >, alt!(
        //parse_var |
        //parse_functioncall |
        delimited!(tag!("("), ws!(parse_exp), tag!(")"))
));

named!(pub parse_exp< ASTNode >, dbg!(alt!(
                parse_number |
                parse_nil |
                parse_bool |
                parse_op |
                // parse_literal_string
                parse_vararg |
                // parse_functiondef |
                parse_prefixexp
//                parse_tableconstructior
            )));

// TODO: Missing tests
//named!(parse_tableconstructor< ASTNode >,
//       do_parse!(
//           tag!("{") >>
//           f: parse_fieldlist >>
//           tag!("}") >>
//           (ASTNode::TableConstructor(f))));


//named!(parse_fieldlist< ASTNode >, unimplemented!());
//named!(parse_field, alt!(
//        delimited!(tag!("["), ws!(parse_exp), tag!("]")) |
//        tag!(";")
//));
named!(parse_fieldsep, alt!(tag!(",") | tag!(";")));

#[cfg(test)]
mod tests {
    ast_test!(test_parse_nil, parse_nil, "nil", ASTNode::Nil);
    ast_test!(test_parse_bool_t, parse_bool, "true", ASTNode::Bool(true));
    ast_test!(test_parse_bool_f, parse_bool, "false", ASTNode::Bool(false));
    ast_test!(test_parse_vararg, parse_vararg, "...", ASTNode::VarArg);
    ast_valid!(test_parse_fieldsep_1, parse_fieldsep, ";");
    ast_valid!(test_parse_fieldsep_2, parse_fieldsep, ",");

    //make a generalized example of this test, ie: any random char after a tag
    //ast_panic_test!(test_parse_vararg_false, parse_vararg, "....", ASTNode::VarArg);
}
