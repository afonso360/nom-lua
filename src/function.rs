// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::ASTNode;
use ast::ASTNode::*;
use statement::{parse_retstat, parse_statement};
use name::{parse_name, parse_namelist};

// TODO: Needs ws! macros

named!(pub parse_functiondef<ASTNode>,
       do_parse!(tag!("function") >> f: ws!(parse_funcbody) >> (astb!(Function, f))));

named!(pub parse_local_function<ASTNode>, do_parse!(
           tag!("local")
        >> ws!(tag!("function"))
        >> n: parse_name
        >> f: parse_funcbody
        >> (astb!(NamedFunction, n, f))));

named!(parse_funcbody<ASTNode>, do_parse!(
           parlist: delimited!(tag!("("), opt!(ws!(parse_parlist)), tag!(")"))
        >> block: ws!(parse_block)
        >> tag!("end")
        >> (astb!(FunctionBody, parlist, block))));

// This is here because rustc complains about lack of type annotations
named!(parse_multiname<Vec<ASTNode>>, many1!(preceded!(ws!(tag!(".")), parse_name)));
named!(parse_funcname<ASTNode>, do_parse!(
       n: map!(parse_name, Box::new)
    >> m: opt!(complete!(parse_multiname))
    >> f: opt!(map!(complete!(preceded!(ws!(tag!(":")), parse_name)), Box::new))
    >> (ASTNode::FunctionName(n, m, f))
));

named!(parse_parlist<ASTNode>, do_parse!(
       nl: opt!(complete!(parse_namelist))
    >> opt!(complete!(ws!(tag!(","))))
    >> va: opt!(complete!(ws!(tag!("..."))))
    >> (ASTNode::ParameterList(Box::new(nl), va.is_some()))
));

named!(pub parse_block<ASTNode>, do_parse!(
           s: many0!(complete!(parse_statement))
        >> rs: opt!(ws!(complete!(parse_retstat)))
        >> (ast!(Block, s, Box::new(rs)))
));

#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_test!(parse_parlist_1, parse_parlist, "...",
              ast!(ParameterList, Box::new(None), true));

    ast_test!(parse_parlist_2, parse_parlist, "",
              ast!(ParameterList, Box::new(None), false));

    ast_test!(parse_parlist_3, parse_parlist, "name , ...",
              ast!(ParameterList, Box::new(Some(ast!(NameList, vec![
                ast!(Name, "name".into())
              ]))), true));

    ast_test!(parse_parlist_5, parse_parlist, "a,b",
              ast!(ParameterList, Box::new(Some(ast!(NameList, vec![
                ast!(Name, "a".into()),
                ast!(Name, "b".into())
              ]))), false));



    ast_test!(parse_block_1, parse_block, "", ast!(Block, vec![], Box::new(None)));
    ast_test!(parse_block_2, parse_block, "::a::", ast!(Block, vec![
        ast!(Label, "a".into())
    ], Box::new(None)));

    ast_test!(parse_block_3, parse_block, "::b:: return 1.0", ast!(Block, vec![
        ast!(Label, "b".into())
    ], Box::new(Some(astb!(RetStat, Some(ast!(ExpList, vec![
        ast!(Float, 1.0)
    ])))))));


    ast_test!(parse_funcbody_1, parse_funcbody, "( a, b ) ; end",
        astb!(FunctionBody,
              Some(ast!(ParameterList, Box::new(Some(ast!(NameList, vec![
                ast!(Name, "a".into()),
                ast!(Name, "b".into())
              ]))), false)),
              ast!(Block, vec![
                ast!(EmptyStatement)
              ], Box::new(None))));

    ast_test!(parse_functiondef_1, parse_functiondef, "function (...) ; end",
        astb!(Function,
         astb!(FunctionBody,
              Some(ast!(ParameterList, Box::new(None), true)),
              ast!(Block, vec![ ast!(EmptyStatement) ], Box::new(None)))));

    ast_test!(parse_local_function_1, parse_local_function, "local function b() ; end",
        astb!(NamedFunction,
              ast!(Name, "b".into()),
              astb!(FunctionBody,
                    Some(ast!(ParameterList, Box::new(None), false)),
                    ast!(Block, vec![ ast!(EmptyStatement) ], Box::new(None)))));

    ast_test!(parse_funcname_1, parse_funcname, "a",
              ast!(FunctionName, Box::new(ast!(Name, "a".into())), None, None));
    ast_test!(parse_funcname_2, parse_funcname, "a.b",
              ast!(FunctionName,
                   Box::new(ast!(Name, "a".into())),
                   Some(vec![
                        ast!(Name, "b".into())
                   ]),
                   None));
    ast_test!(parse_funcname_3, parse_funcname, "a. b . c",
              ast!(FunctionName,
                   Box::new(ast!(Name, "a".into())),
                   Some(vec![
                        ast!(Name, "b".into()),
                        ast!(Name, "c".into())
                   ]),
                   None));
    ast_test!(parse_funcname_4, parse_funcname, "a.b:c",
              ast!(FunctionName,
                   Box::new(ast!(Name, "a".into())),
                   Some(vec![
                        ast!(Name, "b".into()),
                   ]),
                   Some(Box::new(ast!(Name, "c".into())))));
}
//		 function funcname funcbody |
//		 local function Name funcbody |
