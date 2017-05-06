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

    ast_test!(parse_varlist_1, parse_varlist, "xcz", astb!(VarList, vec![
        astb!(Var, ast!(Name, "xcz".into()))
    ]));
    ast_test!(parse_varlist_2, parse_varlist, "xcz , mcx", astb!(VarList, vec![
        astb!(Var, ast!(Name, "xcz".into())),
        astb!(Var, ast!(Name, "mcx".into()))
    ]));
    ast_test!(parse_varlist_3, parse_varlist, "lak , k, jd3", astb!(VarList, vec![
        astb!(Var, ast!(Name, "lak".into())),
        astb!(Var, ast!(Name, "k".into())),
        astb!(Var, ast!(Name, "jd3".into()))
    ]));
}
