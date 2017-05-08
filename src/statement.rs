// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
        delimited!(
            tag!("return"),
            ws!(opt!(complete!(parse_explist))),
            opt!(complete!(tag!(";")))
        ),
        Box::new), ASTNode::RetStat));

#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_valid!(parse_semicolon, parse_semicolon, ";");

    ast_test!(parse_goto_1, parse_goto, "goto valid",
              astb!(Goto, ast!(Name, "valid".into())));
    ast_panic_test!(parse_goto_2, parse_goto, "goto 17");
    ast_panic_test!(parse_goto_3, parse_goto, "got 17");

    ast_test!(parse_retstat_1, parse_retstat, "return false,true ;",
              astb!(RetStat, Some(ast!(ExpList, vec![
                ast!(Bool, false),
                ast!(Bool, true)
              ]))));

    ast_test!(parse_retstat_2, parse_retstat, "return 1.0",
              astb!(RetStat, Some(ast!(ExpList, vec![
                ast!(Float, 1.0),
              ]))));

    ast_test!(parse_retstat_3, parse_retstat, "return",
              astb!(RetStat, None));
}
