// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::ASTNode;
use ast::ASTNode::*;

use number::parse_number;
use op::parse_op;
use string::parse_string;
use function::parse_functiondef;
use field::parse_fieldlist;
use var::parse_var;

named!(parse_vararg<ASTNode>, map!(tag!("..."), |_| ast!(VarArg)));
named!(parse_nil<ASTNode>, map!(tag!("nil"), |_| ast!(Nil)));
named!(parse_bool<ASTNode>, alt!(map!(tag!("false"), |_| ast!(Bool, false)) |
                                 map!(tag!("true"), |_| ast!(Bool, true))));

named!(pub parse_prefixexp<ASTNode>, map!(map!(alt!(
        //parse_functioncall |
        delimited!(tag!("("), ws!(parse_exp), tag!(")")) |
        parse_var
), Box::new), ASTNode::PrefixExp)) ;

named!(pub parse_explist<ASTNode>, map!(
            map!(do_parse!(
                   a: parse_exp
                >> b: many0!(preceded!(ws!(tag!(",")), parse_exp))
                >> (a,b)
            ), |(a, mut b): (_, Vec < ASTNode >) | { b.insert(0, a); b }),
ASTNode::ExpList));

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

    ast_test!(parse_explist_1, parse_explist, "true", ast!(ExpList, vec![
        ast!(Bool, true)
    ]));
    ast_test!(parse_explist_2, parse_explist, "true , true", ast!(ExpList, vec![
        ast!(Bool, true),
        ast!(Bool, true)
    ]));
    ast_test!(parse_explist_3, parse_explist, "true , false, false", ast!(ExpList, vec![
        ast!(Bool, true),
        ast!(Bool, false),
        ast!(Bool, false)
    ]));
}
