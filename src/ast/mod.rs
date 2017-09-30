// Copyright 2017 The nom-lua project developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod op;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use op::{BinOp, UnOp};

#[derive(Clone, PartialEq)]
pub enum ASTNode<'a> {
    // TODO: Should this be u64?
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Label(String),
    Name(String),
    Paren(Box<ASTNode<'a>>),

    Block(Vec<ASTNode<'a>>, Box<Option<ASTNode<'a>>>),

    UnOp(UnOp<'a>),
    BinOp(BinOp<'a>),

    //Statements
    EmptyStatement,
    Break,
    Goto(Box<ASTNode<'a>>),
    RetStat(Box<Option<ASTNode<'a>>>),

    // Expression
    /// Takes one of
    /// Var
    /// FunctionCall
    /// Exp
    PrefixExp(Box<ASTNode<'a>>),

    Nil,
    VarArg,
    TableConstructor(Box<Option<ASTNode<'a>>>),

    // Function
    /// Takes a FunctionBody
    Function(Box<ASTNode<'a>>),
    /// Takes a ParameterList and a Block
    FunctionBody(Box<Option<ASTNode<'a>>>, Box<ASTNode<'a>>),
    /// Has 3 parameters
    /// the example: log.ms:al
    /// would produce
    /// Name log
    /// Name ms
    /// Name al
    FunctionName(Box<ASTNode<'a>>, Option<Vec<ASTNode<'a>>>, Option<Box<ASTNode<'a>>>),
    /// Takes a Name and a FunctionBody
    NamedFunction(Box<ASTNode<'a>>, Box<ASTNode<'a>>),

    // Lists
    ExpList(Vec<ASTNode<'a>>),
    VarList(Vec<ASTNode<'a>>),
    NameList(Vec<ASTNode<'a>>),
    FieldList(Vec<ASTNode<'a>>),
    /// Takes a list of parameters and is vararg
    ParameterList(Box<Option<ASTNode<'a>>>, bool),

    // Field
    /// Contains an expr
    FieldSingle(Box<ASTNode<'a>>),
    /// The first node may be an expr to be resolved or a Name
    /// The second node is the assigned expr
    FieldAssign(Box<ASTNode<'a>>, Box<ASTNode<'a>>),

    // Local
    Local(Box<ASTNode<'a>>),

    // Var
    /// Takes a Name
    Var(Box<ASTNode<'a>>),
    /// Takes a prefixexp and a exp
    VarPrefixed(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    /// Takes a prefixexp and a Name
    VarListAccess(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
}

impl<'a> Debug for ASTNode<'a> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::ASTNode::*;
        match *self {
            Integer(val) => write!(format, "{}", val),
            Float(val) => write!(format, "{}f", val),
            Bool(val) => write!(format, "{}", val),
            String(ref val) => write!(format, "\"{}\"", val),

            // Holds a lua name, usually a function or variable name
            // Contains `ASTNode::String`
            Name(ref val) => write!(format, "(name {})", val),

            // Holds a lua label name
            // Contains `ASTNode::Name`
            Label(ref val) => write!(format, "::{}::", val),
            // Contains an expression
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


// TODO: Redo graphviz
/*

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
*/
