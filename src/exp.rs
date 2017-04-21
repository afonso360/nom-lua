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

named!(parse_vararg< ASTNode >, map!(tag!("..."), |_| ASTNode::VarArg));
named!(parse_nil< ASTNode >, map!(tag!("nil"), |_| ASTNode::Nil));
named!(parse_bool< ASTNode >, alt!(map!(tag!("false"), |_| ASTNode::Bool(false)) |
                                   map!(tag!("true"), |_| ASTNode::Bool(true))));

named!(pub parse_exp< ASTNode >, alt!(
                parse_nil |
                parse_bool |
                parse_number |
                // parse_literal_string
                parse_vararg
                // parse_functiondef |
                // prefixexp
//                parse_tableconstructior
                // exp binop exp
                // unop exp
            ));

// TODO: Missing tests
//named!(parse_tableconstructor< ASTNode >,
//       do_parse!(
//           tag!("{") >>
//           f: parse_fieldlist >>
//           tag!("}") >>
//           (ASTNode::TableConstructor(f))));


//named!(parse_fieldlist< ASTNode >, unimplemented!());

#[cfg(test)]
mod tests {
    ast_test!(test_parse_nil, parse_nil, "nil", ASTNode::Nil);
    ast_test!(test_parse_bool_t, parse_bool, "true", ASTNode::Bool(true));
    ast_test!(test_parse_bool_f, parse_bool, "false", ASTNode::Bool(false));
    ast_test!(test_parse_vararg, parse_vararg, "...", ASTNode::VarArg);

    //make a generalized example of this test, ie: any random char after a tag
    //ast_panic_test!(test_parse_vararg_false, parse_vararg, "....", ASTNode::VarArg);
}
