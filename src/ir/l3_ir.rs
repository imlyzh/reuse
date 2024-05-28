pub mod display;

use crate::types::{FunctionType, Type};

pub type Name = String;

#[derive(Debug, Clone)]
pub enum Body {
    // Bind(Pattern, Box<Expr>, Box<Expr>),
    Compute(Compute),
    Bind(Bind),
    If(If),
    Match(Match),
    Dup(Name, Name, Box<Body>),
    Drop(Name, Box<Body>),
    DropReuse(Name, Name, Box<Body>),
}

#[derive(Debug, Clone)]
pub enum Compute {
    Variable(Name),
    Invoke(Name, Vec<Name>),
    // Lambda(params, body)
    // Lambda(Vec<String>, Box<Expr>),
    Closure {
        fun_type: FunctionType,
        free_vars: Vec<Name>,
        params: Vec<(Name, Owned)>,
        body: Box<Body>,
    },
    Constructor(String, Type, Option<String>, Vec<Name>),
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub pat: Pattern,
    pub owned: Owned,
    pub ty: Type,
    pub value: Box<Compute>,
    pub cont: Box<Body>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Name,
    pub then: Box<Body>,
    pub else_: Box<Body>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub value: Name,
    pub owned: Owned,
    pub matchs: Vec<(Pattern, Body)>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Variable(Name),
    Constructor(Name, Vec<Pattern>),
}

#[derive(Debug, Clone, Copy)]
pub enum Owned {
    Linear,
    Borrow,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Name,
    pub return_type: Type,
    pub args: Vec<(Name, (Type, Owned))>,
    pub body: Body,
}

// pub struct Module {
//   pub functions: Vec<Function>,
// }
