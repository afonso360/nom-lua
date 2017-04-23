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
use statement::{parse_retstat, parse_statement};
use name::{parse_name, parse_namelist};

// TODO: Needs ws! macros

named!(pub parse_functiondef<ASTNode>,
       do_parse!(tag!("function") >> f: parse_funcbody >> (ASTNode::Function(Box::new(f)))));

named!(pub parse_local_function<ASTNode>, do_parse!(
           tag!("function")
        >> ws!(tag!("function"))
        >> n: parse_name
        >> f: parse_funcbody
        >> (ASTNode::NamedFunction(Box::new(n), Box::new(f)))));

named!(parse_funcbody<ASTNode>, do_parse!(
           tag!("(")
        >> parlist: opt!(parse_parlist)
        >> tag!(")")
        >> block: parse_block
        >> tag!("end")
        >> (ASTNode::FunctionBody(Box::new(parlist), Box::new(block)))));

named!(parse_parlist<ASTNode>, do_parse!(
       nl: opt!(complete!(parse_namelist))
    >> opt!(complete!(ws!(tag!(","))))
    >> va: opt!(complete!(ws!(tag!("..."))))
    >> (ASTNode::ParameterList(Box::new(nl), va.is_some()))
));

named!(pub parse_block<ASTNode>, do_parse!(
           //TODO: many0 or many1?
           s: many0!(parse_statement)
        >> rs: opt!(parse_retstat)
        >> (ASTNode::Block(Box::new(s), Box::new(rs)))
));

#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_test!(parse_parlist_1, parse_parlist, "...",
              ast!(ParameterList, Box::new(None), true));

    ast_test!(parse_parlist_2, parse_parlist, "",
              ast!(ParameterList, Box::new(None), false));

    ast_test!(parse_parlist_3, parse_parlist, "name , ...",
              ast!(ParameterList, Box::new(Some(astb!(NameList, vec![
                ast!(Name, "name".into())
              ]))), true));

    ast_test!(parse_parlist_5, parse_parlist, "a,b",
              ast!(ParameterList, Box::new(Some(astb!(NameList, vec![
                ast!(Name, "a".into()),
                ast!(Name, "b".into())
              ]))), false));



    //ast_test!(test_parse_block_1, parse_block, "");
    //ast_test!(test_parse_funcbody_1, parse_funcbody, "");
}

