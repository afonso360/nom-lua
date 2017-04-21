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

// TODO: Should work, Needs tests
//named!(pub parse_varlist<ASTNode>, map!(map!(
//            map!(do_parse!(
//                   a: parse_var
//                >> b: many0!(preceded!(ws!(tag!(",")), parse_var))
//                >> ((a,b))
//            ), |(a, mut b): (_, Vec < ASTNode >) | { b.insert(0, a); b }),
//Box::new), ASTNode::VarList));

#[cfg(test)]
mod tests {
}
