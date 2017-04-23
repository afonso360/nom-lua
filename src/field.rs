
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
use exp::parse_exp;
use name::parse_name;

named!(pub parse_fieldlist<ASTNode>, map!(map!(
            map!(do_parse!(
                   a: parse_field
                >> b: many0!(preceded!(parse_fieldsep, parse_field))
                >> ((a,b))
            ), |(a, mut b): (_, Vec <ASTNode>) | { b.insert(0, a); b }),
Box::new), ASTNode::FieldList));

named!(parse_field<ASTNode>, ws!(alt!(
        do_parse!(
               n: delimited!(tag!("["), ws!(parse_exp), tag!("]"))
            >> ws!(tag!("="))
            >> e: parse_exp
            >> (astb!(FieldAssign, n, e)))|
        do_parse!(
               n: parse_name
            >> ws!(tag!("="))
            >> e: parse_exp
            >> (astb!(FieldAssign, n, e)))|
        map!(map!(parse_exp, Box::new), ASTNode::FieldSingle)
)));

named!(parse_fieldsep, alt!(tag!(",") | tag!(";")));

#[cfg(test)]
mod tests {
    use ast::ASTNode::*;

    ast_valid!(test_parse_fieldsep_1, parse_fieldsep, ";");
    ast_valid!(test_parse_fieldsep_2, parse_fieldsep, ",");

    ast_test!(test_parse_field_assign_1, parse_field, " [ true ] = true ",
              astb!(FieldAssign, ast!(Bool, true), ast!(Bool, true)));
    ast_test!(test_parse_field_assign_2, parse_field, "[true]=nil",
              astb!(FieldAssign, ast!(Bool, true), ast!(Nil)));
    ast_test!(test_parse_field_assign_3, parse_field, "is=true",
              astb!(FieldAssign, ast!(Name, "is".into()), ast!(Bool, true)));
    ast_test!(test_parse_field_single_1, parse_field, "true",
              astb!(FieldSingle, ast!(Bool, true)));
}
