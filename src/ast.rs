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

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Label(String),
    Name(String),
    Paren(Box<ASTNode>),

    Block(Vec<ASTNode>, Box<Option<ASTNode>>),

    //Statements
    EmptyStatement,
    Break,
    Goto(Box<ASTNode>),
    RetStat(Box<Option<ASTNode>>),

    // ArithmeticOps
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    Exp(Box<ASTNode>, Box<ASTNode>),
    FDiv(Box<ASTNode>, Box<ASTNode>),
    Mod(Box<ASTNode>, Box<ASTNode>),

    // LogicOps
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),

    // RelationalOps
    Lt(Box<ASTNode>, Box<ASTNode>),
    Le(Box<ASTNode>, Box<ASTNode>),
    Gt(Box<ASTNode>, Box<ASTNode>),
    Ge(Box<ASTNode>, Box<ASTNode>),
    Eq(Box<ASTNode>, Box<ASTNode>),
    Ne(Box<ASTNode>, Box<ASTNode>),

    // BinaryOps
    BitOr(Box<ASTNode>, Box<ASTNode>),
    BitAnd(Box<ASTNode>, Box<ASTNode>),
    BitXor(Box<ASTNode>, Box<ASTNode>),
    Rsh(Box<ASTNode>, Box<ASTNode>),
    Lsh(Box<ASTNode>, Box<ASTNode>),

    // UnaryOps
    BinNot(Box<ASTNode>),
    Not(Box<ASTNode>),
    Len(Box<ASTNode>),
    UMin(Box<ASTNode>),

    // ConcatenationOps
    Concat(Box<ASTNode>, Box<ASTNode>),

    // Expression
    /// Takes one of
    /// Var
    /// FunctionCall
    /// Exp
    PrefixExp(Box<ASTNode>),

    Nil,
    VarArg,
    TableConstructor(Box<Option<ASTNode>>),

    // Function
    /// Takes a FunctionBody
    Function(Box<ASTNode>),
    /// Takes a ParameterList and a Block
    FunctionBody(Box<Option<ASTNode>>, Box<ASTNode>),
    /// Has 3 parameters
    /// the example: log.ms:al
    /// would produce
    /// Name log
    /// Name ms
    /// Name al
    FunctionName(Box<ASTNode>, Option<Vec<ASTNode>>, Option<Box<ASTNode>>),
    /// Takes a Name and a FunctionBody
    NamedFunction(Box<ASTNode>, Box<ASTNode>),

    // Lists
    ExpList(Vec<ASTNode>),
    VarList(Vec<ASTNode>),
    NameList(Vec<ASTNode>),
    FieldList(Vec<ASTNode>),
    /// Takes a list of parameters and is vararg
    ParameterList(Box<Option<ASTNode>>, bool),

    // Field
    /// Contains an expr
    FieldSingle(Box<ASTNode>),
    /// The first node may be an expr to be resolved or a Name
    /// The second node is the assigned expr
    FieldAssign(Box<ASTNode>, Box<ASTNode>),

    // Local
    Local(Box<ASTNode>),

    // Var
    /// Takes a Name
    Var(Box<ASTNode>),
    /// Takes a prefixexp and a exp
    VarPrefixed(Box<ASTNode>, Box<ASTNode>),
    /// Takes a prefixexp and a Name
    VarListAccess(Box<ASTNode>, Box<ASTNode>),
}

impl Debug for ASTNode {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::ASTNode::*;
        match *self {
            Integer(val) => write!(format, "{}", val),
            Float(val) => write!(format, "{}f", val),
            Bool(val) => write!(format, "{}", val),
            String(ref val) => write!(format, "\"{}\"", val),

            /// Holds a lua name, usually a function or variable name
            /// Contains `ASTNode::String`
            Name(ref val) => write!(format, "(name {})", val),

            /// Holds a lua label name
            /// Contains `ASTNode::Name`
            Label(ref val) => write!(format, "::{}::", val),
            /// Contains an expression
            Paren(ref expr) => write!(format, "({})", expr),

            // Block
            Block(ref statements, ref retstat) => {
                write!(format, "(block\n");
                for e in statements.iter() {
                    write!(format, "\t{}\n", e);
                }
                if let Some(ref ret_ast) = **retstat {
                    write!(format, "\treturn {}\n", ret_ast);
                }
                write!(format, ")")
            }

            // Statements
            EmptyStatement => write!(format, "(statement)"),
            RetStat(ref para) => write!(format, "(ret {:?})", para),
            Break => write!(format, "(break)"),
            Goto(ref loc) => write!(format, "goto {}", loc),

            // ArithmeticOps
            Add(ref left, ref right) => write!(format, "({} + {})", left, right),
            Sub(ref left, ref right) => write!(format, "({} - {})", left, right),
            Mul(ref left, ref right) => write!(format, "({} * {})", left, right),
            Div(ref left, ref right) => write!(format, "({} / {})", left, right),
            Exp(ref left, ref right) => write!(format, "({} ^ {})", left, right),
            FDiv(ref left, ref right) => write!(format, "({} // {})", left, right),
            Mod(ref left, ref right) => write!(format, "({} % {})", left, right),

            // LogicOps
            And(ref left, ref right) => write!(format, "({} and {})", left, right),
            Or(ref left, ref right) => write!(format, "({} or {})", left, right),

            // ArithmeticOps
            Lt(ref left, ref right) => write!(format, "({} < {})", left, right),
            Le(ref left, ref right) => write!(format, "({} <= {})", left, right),
            Gt(ref left, ref right) => write!(format, "({} > {})", left, right),
            Ge(ref left, ref right) => write!(format, "({} >= {})", left, right),
            Eq(ref left, ref right) => write!(format, "({} == {})", left, right),
            Ne(ref left, ref right) => write!(format, "({} ~= {})", left, right),

            // BinaryOps
            BitAnd(ref left, ref right) => write!(format, "({} & {})", left, right),
            BitOr(ref left, ref right) => write!(format, "({} | {})", left, right),
            BitXor(ref left, ref right) => write!(format, "({} ~ {})", left, right),
            Rsh(ref left, ref right) => write!(format, "({} >> {})", left, right),
            Lsh(ref left, ref right) => write!(format, "({} << {})", left, right),

            // UnaryOps
            BinNot(ref right) => write!(format, "~{}", right),
            Len(ref right) => write!(format, "#{}", right),
            UMin(ref right) => write!(format, "-{}", right),
            Not(ref right) => write!(format, "not {}", right),

            // ConcatenationOps
            Concat(ref left, ref right) => write!(format, "{} .. {}", left, right),

            // Exp
            PrefixExp(ref e) => write!(format, "{}", e),

            Nil => write!(format, "nil"),
            VarArg => write!(format, "..."),
            //TODO: Remove this debug impl
            TableConstructor(ref fieldlist) => write!(format, "{{ {:?} }}", fieldlist),

            // Function
            Function(ref f) => write!(format, "{}", f),
            FunctionBody(ref parlist, ref fbody) => write!(format, "function ({:?}) {}", parlist, fbody),
            FunctionName(ref n, ref m, ref f) => write!(format, "{}.{:?}:{:?}", n, m, f),
            NamedFunction(ref n, ref f) => write!(format, "(named {} {})", n, f),

            // Lists
            ExpList(ref explist) => {
                write!(format, "(explist\n");
                for e in explist.iter() {
                    write!(format, "\t{}\n", e);
                }
                write!(format, ")")
            },
            VarList(ref varlist) => {
                write!(format, "(varlist\n");
                for e in varlist.iter() {
                    write!(format, "\t{}\n", e);
                }
                write!(format, ")")
            },
            NameList(ref namelist) => {
                write!(format, "(namelist\n");
                for e in namelist.iter() {
                    write!(format, "\t{}\n", e);
                }
                write!(format, ")")
            },
            ParameterList(ref plist, ref va) => {
                write!(format, "(paramlist\n");
                for e in plist.iter() {
                    write!(format, "\t{}\n", e);
                }
                if *va {
                    write!(format, "\t...\n");
                }
                write!(format, ")")
            },
            FieldList(ref fieldlist) => {
                write!(format, "(fieldlist\n");
                for e in fieldlist.iter() {
                    write!(format, "\t{}\n", e);
                }
                write!(format, ")")
            },

            // Field
            FieldSingle(ref e) => write!(format, "(field {})", e),
            FieldAssign(ref n, ref e) => write!(format, "(field {} => {})", n, e),

            //Local
            Local(ref inner) => write!(format, "local {}", inner),

            //Var
            Var(ref name) => write!(format, "(var {})", name),
            VarPrefixed(ref pe, ref e) => write!(format, "{}[{}]", pe, e),
            VarListAccess(ref pe, ref n) => write!(format, "{}.{}", pe, n)
        }
    }

}

impl Display for ASTNode {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use ASTNode::*;
        match *self {
            //TODO: Check if the DOT format supports ()
            Integer(a) => write!(format, "Integer_{}_", a),
            Float(a) => write!(format, "Float_{}_", a),
            Bool(a) => write!(format, "Bool_{}_", a),
            // Dot does not allow spaces and a bunch of things in the names
            // we should change some of this stuff
            String(ref a) => write!(format, "String_{}_", a),
            Label(ref a) => write!(format, "Label_{}_", a),
            Name(ref a) => write!(format, "Name_{}_", a),
            Paren(_) => write!(format, "Paren"),
            Block(_, _) => write!(format, "Block"),
            EmptyStatement => write!(format, "EmptyStatement"),
            Break => write!(format, "Break"),
            Goto(_) => write!(format, "Goto"),
            RetStat(_) => write!(format, "RetStat"),
            Add(_, _) => write!(format, "Add"),
            Sub(_, _) => write!(format, "Sub"),
            Mul(_, _) => write!(format, "Mul"),
            Div(_, _) => write!(format, "Div"),
            Exp(_, _) => write!(format, "Exp"),
            FDiv(_, _) => write!(format, "FDiv"),
            Mod(_, _) => write!(format, "Mod"),
            And(_, _) => write!(format, "And"),
            Or(_, _) => write!(format, "Or"),
            Lt(_, _) => write!(format, "Lt"),
            Le(_, _) => write!(format, "Le"),
            Gt(_, _) => write!(format, "Gt"),
            Ge(_, _) => write!(format, "Ge"),
            Eq(_, _) => write!(format, "Eq"),
            Ne(_, _) => write!(format, "Ne"),
            BitOr(_, _) => write!(format, "BitOr"),
            BitAnd(_, _) => write!(format, "BitAnd"),
            BitXor(_, _) => write!(format, "BitXor"),
            Rsh(_, _) => write!(format, "Rsh"),
            Lsh(_, _) => write!(format, "Lsh"),
            BinNot(_) => write!(format, "BinNot"),
            Not(_) => write!(format, "Not"),
            Len(_) => write!(format, "Len"),
            UMin(_) => write!(format, "UMin"),
            Concat(_, _) => write!(format, "Concat"),
            PrefixExp(_) => write!(format, "PrefixExp"),
            Nil => write!(format, "Nil"),
            VarArg => write!(format, "VarArg"),
            TableConstructor(_) => write!(format, "TableConstructor"),
            Function(_) => write!(format, "Function"),
            FunctionBody(_, _) => write!(format, "FunctionBody"),
            FunctionName(_, _, _) => write!(format, "FunctionName"),
            NamedFunction(_, _) => write!(format, "NamedFunction"),
            ExpList(_) => write!(format, "ExpList"),
            VarList(_) => write!(format, "VarList"),
            NameList(_) => write!(format, "NameList"),
            FieldList(_) => write!(format, "FieldList"),
            ParameterList(_, _) => write!(format, "ParameterList"),
            FieldSingle(_) => write!(format, "FieldSingle"),
            FieldAssign(_, _) => write!(format, "FieldAssign"),
            Local(_) => write!(format, "Local"),
            Var(_) => write!(format, "Var"),
            VarPrefixed(_, _) => write!(format, "VarPrefixed"),
            VarListAccess(_, _) => write!(format, "VarListAccess"),
        }
    }
}

//TODO: There is a bunch of cloing here that might not be necessary
//TODO: Is there a better way to do this, or do we need 6 cfg blocks
#[cfg(feature="graphviz")]
type Node = ASTNode;
#[cfg(feature="graphviz")]
type Edge = (ASTNode, ASTNode);
#[cfg(feature="graphviz")]
struct Edges(Vec<Edge>);
#[cfg(feature="graphviz")]
use dot;
#[cfg(feature="graphviz")]
use std::borrow::Cow;
#[cfg(feature="graphviz")]
use std::io::Write;


#[cfg(feature="graphviz")]
impl<'a> dot::Labeller<'a, Node, Edge> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'a self, node: &Node) -> dot::Id<'a> {
        dot::Id::new(format!("{}", node)).unwrap()
    }
}


#[cfg(feature="graphviz")]
impl<'a> dot::GraphWalk<'a, Node, Edge> for Edges {
    fn nodes(&self) -> dot::Nodes<'a,Node> {
        // (assumes that |N| \approxeq |E|)
        let &Edges(ref v) = self;
        let mut node_vec = Vec::with_capacity(v.len());
        for n in v {
            let &(ref s, ref t) = n;
            node_vec.extend(s.sub_nodes());
            node_vec.extend(t.sub_nodes());
        }
        node_vec.dedup();
        Cow::Owned(node_vec)
    }

    fn edges(&'a self) -> dot::Edges<'a,Edge> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Edge) -> Node { let &(ref s,_) = e; s.clone() }

    fn target(&self, e: &Edge) -> Node { let &(_,ref t) = e; t.clone() }
}


#[cfg(feature="graphviz")]
impl ASTNode {
    fn generate_edges(&self) -> Vec<(ASTNode, ASTNode)> {
        use ASTNode::*;
        let mut node_vec = Vec::new();
        match (*self).clone() {

            Integer(_) |
            Float(_) |
            Bool(_) |
            String(_) |
            Label(_) |
            Name(_) |
            Nil |
            Break |
            VarArg |
            EmptyStatement => {},

            Paren(a) |
            Local(a) |
            Var(a) |
            Goto(a) |
            BinNot(a) |
            Not(a) |
            Len(a) |
            UMin(a) |
            PrefixExp(a) |
            Function(a) |
            FieldSingle(a) |
            PrefixExp(a) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.extend(a.generate_edges());
            },

            Add(a, b) |
            Sub(a, b) |
            Mul(a, b) |
            Div(a, b) |
            Exp(a, b) |
            FDiv(a, b) |
            Mod(a, b) |
            And(a, b) |
            Or(a, b) |
            Lt(a, b) |
            Le(a, b) |
            Gt(a, b) |
            Ge(a, b) |
            Eq(a, b) |
            Ne(a, b) |
            BitOr(a, b) |
            BitAnd(a, b) |
            BitXor(a, b) |
            Rsh(a, b) |
            Lsh(a, b) |
            Concat(a, b) |
            FieldAssign(a, b) |
            VarPrefixed(a, b) |
            VarListAccess(a, b) |
            NamedFunction(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },

            RetStat(a) |
            TableConstructor(a) |
            ParameterList(a, _) => if let Some(sa) = *a {
                node_vec.push(((*self).clone(), sa.clone()));
                node_vec.extend(sa.generate_edges());
            },

            ExpList(a) |
            VarList(a) |
            NameList(a) |
            FieldList(a) => {
                a.iter().map(|ae| {
                    node_vec.push(((*self).clone(), (*ae).clone()));
                    node_vec.extend(ae.generate_edges());
                });
            },

            Block(a, b) => {
                a.iter().map(|ae| {
                    node_vec.push(((*self).clone(), (*ae).clone()));
                    node_vec.extend(ae.generate_edges());
                });
                if let Some(sb) = *b {
                    node_vec.push(((*self).clone(), sb.clone()));
                    node_vec.extend(sb.generate_edges());
                };
            },
            FunctionBody(a, b) => {
                if let Some(sa) = *a {
                    node_vec.push(((*self).clone(), sa.clone()));
                    node_vec.extend(sa.generate_edges());
                };
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(b.generate_edges());
            },
            FunctionName(a, b, c) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.extend(a.generate_edges());
                if let Some(sb) = b {
                    sb.iter().map(|sbe| {
                        node_vec.push(((*self).clone(), (*sbe).clone()));
                        node_vec.extend(sbe.generate_edges());
                    });
                };
                if let Some(sc) = c {
                    node_vec.push(((*self).clone(), (*sc).clone()));
                    node_vec.extend(sc.generate_edges());
                };
            },
            _ => panic!("Unimplemented: GenEdges"),
        }
        node_vec
    }

    fn sub_nodes(&self) -> Vec<ASTNode> {
        use ASTNode::*;
        let mut node_vec = Vec::new();

        node_vec.push((*self).clone());

        match (*self).clone() {
            Nil |
            VarArg |
            Break |
            EmptyStatement |
            Float(_) |
            Bool(_) |
            String(_) |
            Label(_) |
            Name(_) |
            Integer(_) => {},

            Goto(a) |
            BinNot(a) |
            Not(a) |
            Len(a) |
            UMin(a) |
            PrefixExp(a) |
            FieldSingle(a) |
            Local(a) |
            Var(a) |
            Function(a) |
            PrefixExp(a) |
            Paren(a) => node_vec.extend(a.sub_nodes()),

            And(a, b) |
            Or(a, b) |
            Lt(a, b) |
            Le(a, b) |
            Gt(a, b) |
            Ge(a, b) |
            Eq(a, b) |
            Ne(a, b) |
            BitOr(a, b) |
            BitAnd(a, b) |
            BitXor(a, b) |
            Rsh(a, b) |
            Lsh(a, b) |
            FieldAssign(a, b) |
            VarPrefixed(a, b) |
            VarListAccess(a, b) |
            NamedFunction(a, b) |
            Concat(a, b) |
            Add(a, b) |
            Sub(a, b) |
            Mul(a, b) |
            Div(a, b) |
            Exp(a, b) |
            FDiv(a, b) |
            Mod(a, b) => {
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },

            ExpList(a) |
            VarList(a) |
            NameList(a) |
            FieldList(a) => { a.iter().map(|b| node_vec.extend(b.sub_nodes())); },

            RetStat(a) |
            ParameterList(a, _) |
            TableConstructor(a) => if let Some(sa) = *a {
                node_vec.extend(sa.sub_nodes());
            },

            Block(a, b) => {
                a.iter().map(|ae| node_vec.extend(ae.sub_nodes()));
                if let Some(sb) = *b {
                    node_vec.extend(sb.sub_nodes());
                }
            },

            FunctionBody(a, b) => {
                if let Some(sa) = *a {
                    node_vec.extend(sa.sub_nodes());
                }
                node_vec.extend(b.sub_nodes());
            },

            FunctionName(a, b, c) => {
                node_vec.extend(a.sub_nodes());
                if let Some(sb) = b {
                    sb.iter().map(|sbe| node_vec.extend(sbe.sub_nodes()));
                };
                if let Some(sc) = c {
                    node_vec.extend(sc.sub_nodes());
                }
            },
            _ => panic!("Unimplemented: SubNodes"),
        };
        node_vec
    }

    #[cfg(feature="graphviz")]
    pub fn graphviz_render<W: Write>(&self, output: &mut W) {
        let edges = Edges(self.generate_edges());
        dot::render(&edges, output).unwrap()
    }
}
