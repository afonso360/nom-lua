
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
use name::{parse_label, parse_name};
//use function::parse_local_function;
use exp::parse_explist;

named!(parse_goto<ASTNode>, map!(map!(preceded!(tag!("goto"), ws!(parse_name)), Box::new), ASTNode::Goto));

named!(parse_semicolon, ws!(tag!(";")));
named!(parse_semicolon_statement<ASTNode>, map!(parse_semicolon, |_| ASTNode::EmptyStatement));

named!(pub parse_statement<ASTNode>, alt!(
        parse_semicolon_statement |
        //varlist=explist
        //functioncall
        parse_label |
        parse_goto
        //do end (deps block)
        //while (deps exp, block)
        //repeat until (deps exp, block)
        //if (deps exp, block)
        //for (deps exp, block)
        //for in (deps exp, explist, block)
        //function (deps funcname)
        //parse_local_function
        //local (deps namelist, explist)
));

named!(pub parse_retstat<ASTNode>, map!(map!(
        delimited!(tag!("return"), ws!(opt!(parse_explist)), opt!(tag!(";"))),
        Box::new), ASTNode::RetStat));

#[cfg(test)]
mod tests {
    ast_valid!(test_parse_semicolon, parse_semicolon, ";");

    ast_test!(test_parse_goto_1, parse_goto, "goto valid",
              ASTNode::Goto(Box::new(ASTNode::Name("valid".into()))));
    ast_panic_test!(test_parse_goto_2, parse_goto, "goto 17");
    ast_panic_test!(test_parse_goto_3, parse_goto, "got 17");
}
