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

// TODO: Should work, Needs tests
//named!(pub parse_varlist<ASTNode>, map!(map!(
//            map!(do_parse!(
//                   a: parse_var
//                >> b: many0!(preceded!(ws!(tag!(",")), parse_var))
//                >> ((a,b))
//            ), |(a, mut b): (_, Vec < ASTNode >) | { b.insert(0, a); b }),
//Box::new), ASTNode::VarList));

   /// indicates a correct parsing, the first field containing the rest of the unparsed data, the second field contains the parsed data
  //Done(I,O),
  /// contains a Err, an enum that can indicate an error code, a position in the input, and a pointer to another error, making a list of errors in the parsing tree
  //Error(Err<I,E>),
  /// Incomplete contains a Needed, an enum than can represent a known quantity of input data, or unknown
  //Incomplete(Needed)

use nom::IResult;
/*
pub fn parse_var(input: &[u8]) -> IResult<&[u8], ASTNode> {
    /// The function tries to parse a `Name`, if it succeds
    /// we attempt to match that with either [ `exp` ] or . `Name`
    /// if parsing of the Name fails we try the `prefixexp`

    let var = match parse_var(input) {
        IResult::Done(un, var) => Ok((un, var)),
        IResult::Incomplete(need) => { return IResult::Incomplete(need) },
        IResult::Error(err) => Err(err) ,
    };

    if let Ok(name_t) = var {
        match opt!(name_t.0, delimited!(ws!(tag!("[")), ws!(parse_exp), tag!("]"))) {
            IResult::Done(un, var) => {
                return IResult::Done(un, astb!(VarPrefixed, name_t.1, var.unwrap()))
            },
            IResult::Incomplete(need) => { return IResult::Incomplete(need) },
            _ => {},
        };

        match opt!(name_t.0, preceded!(ws!(tag!(".")), parse_name)) {
            IResult::Done(un, res) => {
                return IResult::Done(un, astb!(VarPrefixed, name_t.1, res.unwrap()))
            },
            IResult::Incomplete(need) => { return IResult::Incomplete(need) },
            _ => {},
        };

    }

    unimplemented!();
}
*/

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

            return astb!(Var, name);
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
}
