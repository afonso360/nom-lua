// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::ASTNode;
use ast::ASTNode::*;
use exp::{parse_prefixexp, parse_exp};
use name::parse_name;
use nom::IResult;

named!(pub parse_varlist<ASTNode>, map!(
            map!(do_parse!(
                   a: parse_var
                >> b: many0!(preceded!(ws!(tag!(",")), parse_var))
                >> ((a,b))
            ), |(a, mut b): (_, Vec < ASTNode >) | { b.insert(0, a); b }),
ASTNode::VarList));

named!(pub parse_var<ASTNode>, alt!(
    map!(do_parse!(
           name: parse_name
        >> r: opt!(alt!(complete!(delimited!(ws!(tag!("[")), ws!(parse_exp), tag!("]"))) |
                        complete!(preceded!(ws!(tag!(".")), parse_name))))
        >> ((name, r))
        ), |(name, r)| {
            if let Some(rest) = r {
                match rest {
                    ASTNode::Name(_) => {
                        return astb!(VarListAccess, astb!(PrefixExp, astb!(Var, name)), rest);
                    },
                    _ => {
                        return astb!(VarPrefixed, astb!(PrefixExp, astb!(Var, name)), rest);
                    },
                }
            }

            astb!(Var, name)
        }
    ) |
    do_parse!(
           pe: parse_prefixexp
           // The ws!'s are this way, to not eat any whitespace
           // outside of the expression
        >> e: delimited!(ws!(tag!("[")), ws!(parse_exp), tag!("]"))
        >> (astb!(VarPrefixed, pe, e))) |
    do_parse!(
           pe: parse_prefixexp
        >> n: preceded!(ws!(tag!(".")), parse_name)
        >> (astb!(VarListAccess, pe, n)))
));

#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_test!(parse_var_1, parse_var, "ayy", astb!(Var, ast!(Name, "ayy".into())));
    ast_test!(parse_var_2, parse_var, "ayy [ true ]",
              astb!(VarPrefixed,
                    astb!(PrefixExp, astb!(Var, ast!(Name, "ayy".into()))),
                    ast!(Bool, true)));
    ast_test!(parse_var_3, parse_var, "ayy.zxc",
              astb!(VarListAccess,
                    astb!(PrefixExp, astb!(Var, ast!(Name, "ayy".into()))),
                    ast!(Name, "zxc".into())));

    ast_test!(parse_varlist_1, parse_varlist, "xcz", ast!(VarList, vec![
        astb!(Var, ast!(Name, "xcz".into()))
    ]));
    ast_test!(parse_varlist_2, parse_varlist, "xcz , mcx", ast!(VarList, vec![
        astb!(Var, ast!(Name, "xcz".into())),
        astb!(Var, ast!(Name, "mcx".into()))
    ]));
    ast_test!(parse_varlist_3, parse_varlist, "lak , k, jd3", ast!(VarList, vec![
        astb!(Var, ast!(Name, "lak".into())),
        astb!(Var, ast!(Name, "k".into())),
        astb!(Var, ast!(Name, "jd3".into()))
    ]));
}
