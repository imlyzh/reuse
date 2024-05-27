use crate::types::{FunctionType, Type};

pub type Name = String;

#[derive(Debug, Clone)]
pub enum Body {
    // Bind(Pattern, Box<Expr>, Box<Expr>),
    Compute(Compute),
    Bind(Bind),
    Dup(Name, Box<Body>),
    Drop(Name, Box<Body>),
    DropReuse(Name, Name, Box<Body>),
    If(If),
    BMatch(Match),
    DMatch(Match),
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
        params: Vec<Name>,
        body: Box<Body>,
    },
    Constructor(String, Type, Option<String>, Vec<Name>),
}

#[derive(Debug, Clone)]
pub struct Bind(pub Pattern, pub Type, pub Box<Compute>, pub Box<Body>);

#[derive(Debug, Clone)]
pub struct If(pub Name, pub Box<Body>, pub Box<Body>);

#[derive(Debug, Clone)]
pub struct Match(pub Name, pub Vec<(Pattern, Body)>);

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Variable(Name),
    Constructor(Name, Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Name,
    pub args: Vec<(Name, Owned)>,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub enum Owned {
    // duplication
    Normal,
    Linear,
    Borrow,
}

// pub struct Module {
//   pub functions: Vec<Function>,
// }