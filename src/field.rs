// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::ASTNode;
use ast::ASTNode::*;
use exp::parse_exp;
use name::parse_name;

named!(pub parse_fieldlist< ASTNode >,
       map!(
           separated_nonempty_list!(parse_fieldsep, parse_field),
           ASTNode::FieldList
        )
);

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

    ast_valid!(parse_fieldsep_1, parse_fieldsep, ";");
    ast_valid!(parse_fieldsep_2, parse_fieldsep, ",");

    ast_test!(parse_field_assign_1, parse_field, " [ true ] = true ",
              astb!(FieldAssign, ast!(Bool, true), ast!(Bool, true)));
    ast_test!(parse_field_assign_2, parse_field, "[true]=nil",
              astb!(FieldAssign, ast!(Bool, true), ast!(Nil)));
    ast_test!(parse_field_assign_3, parse_field, "is=true",
              astb!(FieldAssign, ast!(Name, "is".into()), ast!(Bool, true)));
    ast_test!(parse_field_single_1, parse_field, "true",
              astb!(FieldSingle, ast!(Bool, true)));
}
