use crate::types::Type;

pub type Name = String;

#[derive(Debug)]
pub enum Body {
    // Bind(Pattern, Box<Expr>, Box<Expr>),
    Compute(Compute),
    Bind(Bind),
    Dup(Name, Box<Body>),
    Drop(Name, Box<Body>),
    If(If),
    Match(Match),
}

#[derive(Debug)]
pub enum Compute {
    Variable(Name),
    Invoke(Name, Vec<Name>),
    // Lambda(params, body)
    // Lambda(Vec<String>, Box<Expr>),
    Closure {
        free_vars: Vec<Name>,
        params: Vec<Name>,
        body: Box<Body>,
    },
    Constructor(String, Type, Option<String>, Vec<Name>),
}

#[derive(Debug)]
pub struct Bind(pub Pattern, pub Type, pub Box<Compute>, pub Box<Body>);

#[derive(Debug)]
pub struct If(pub Name, pub Box<Body>, pub Box<Body>);

#[derive(Debug)]
pub struct Match(pub Name, pub Vec<(Pattern, Body)>);

#[derive(Debug)]
pub enum Pattern {
    Wildcard,
    Variable(Name),
    Constructor(Name, Vec<Pattern>),
}

#[derive(Debug)]
pub struct Function {
    pub name: Name,
    pub args: Vec<Name>,
    pub body: Body,
}

// pub struct Module {
//   pub functions: Vec<Function>,
// }
