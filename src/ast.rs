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
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Label(String),
    Name(String),
    Paren(Box<ASTNode>),

    Block(Box<Vec<ASTNode>>, Box<Option<ASTNode>>),

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
    FunctionName(Box<ASTNode>, Option<Box<Vec<ASTNode>>>, Option<Box<ASTNode>>),
    /// Takes a Name and a FunctionBody
    NamedFunction(Box<ASTNode>, Box<ASTNode>),

    // Lists
    ExpList(Box<Vec<ASTNode>>),
    VarList(Box<Vec<ASTNode>>),
    NameList(Box<Vec<ASTNode>>),
    FieldList(Box<Vec<ASTNode>>),
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

impl ASTNode {
    fn get_name(&self) -> String {
        use ASTNode::*;
        match *self {
            Integer(a) => format!("Integer_{}", a).to_owned(),
            Float(a) => format!("Float_{}", a).to_owned(),
            Bool(a) => format!("Bool_{}", a).to_owned(),
            String(ref a) => format!("String_{}", a).to_owned(),
            Label(ref a) => format!("Label_{}", a).to_owned(),
            Name(ref a) => format!("Name_{}", a).to_owned(),
            Paren(_) => "Paren".to_owned(),
            Add(_, _) => "Add".to_owned(),
            Sub(_, _) => "Sub".to_owned(),
            Mul(_, _) => "Mul".to_owned(),
            Div(_, _) => "Div".to_owned(),
            Exp(_, _) => "Exp".to_owned(),
            FDiv(_, _) => "FDiv".to_owned(),
            Mod(_, _) => "Mod".to_owned(),
            PrefixExp(_) => "PrefixExp".to_owned(),
            _ => unimplemented!(),
        }
    }
}

impl Display for ASTNode {
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

#[cfg(feature="graphviz")]
type Node = ASTNode;
#[cfg(feature="graphviz")]
type Edge = (ASTNode, ASTNode);
#[cfg(feature="graphviz")]
struct Edges(Vec<Edge>);

#[cfg(feature="graphviz")]
impl<'a> dot::Labeller<'a, Node, Edge> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'a self, node: &Node) -> dot::Id<'a> {
        let name = node.get_name();
        dot::Id::new(name).unwrap()
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
use dot;
#[cfg(feature="graphviz")]
use std::borrow::Cow;
#[cfg(feature="graphviz")]
use std::io::Write;


#[cfg(feature="graphviz")]
impl ASTNode {
    fn generate_edges(&self) -> Vec<(ASTNode, ASTNode)> {
        use ASTNode::*;
        let mut node_vec = Vec::new();
        match (*self).clone() {
            // When we have a full implementation we can move this onto _ => {}
            Integer(_) => {},
            Float(_) => {},
            Bool(_) => {},
            String(_) => {},
            Label(_) => {},
            Name(_) => {},

            Paren(a) => node_vec.push(((*self).clone(), (*a).clone())),

            // ArithmeticOps
            Add(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            Sub(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            Mul(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            Div(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            Exp(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            FDiv(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            Mod(a, b) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.push(((*self).clone(), (*b).clone()));
                node_vec.extend(a.generate_edges());
                node_vec.extend(b.generate_edges());
            },
            PrefixExp(a) => {
                node_vec.push(((*self).clone(), (*a).clone()));
                node_vec.extend(a.generate_edges());
            },
            _ => panic!("Unimplemented: GenEdges: {:?}", (*self).clone()),
        }
        node_vec
    }

    fn sub_nodes(&self) -> Vec<ASTNode> {
        use ASTNode::*;
        let mut node_vec = Vec::new();
        match (*self).clone() {
            Integer(_) => node_vec.push((*self).clone()),
            Float(_) => node_vec.push((*self).clone()),
            Bool(_) => node_vec.push((*self).clone()),
            String(_) => node_vec.push((*self).clone()),
            Label(_) => node_vec.push((*self).clone()),
            Name(_) => node_vec.push((*self).clone()),

            Paren(a) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
            },

            // ArithmeticOps
            Add(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            Sub(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            Mul(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            Div(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            Exp(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            FDiv(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            Mod(a, b) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
                node_vec.extend(b.sub_nodes());
            },
            PrefixExp(a) => {
                node_vec.push((*self).clone());
                node_vec.extend(a.sub_nodes());
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
