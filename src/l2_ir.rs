use crate::types::Type;

#[derive(Debug)]
pub enum Expr {
    Invoke(String, Vec<String>),
    // Bind(Pattern, Box<Expr>, Box<Expr>),
    Bind(Bind),
    Dup(String, Box<Expr>),
    Drop(String, Box<Expr>),
    Match(Match),
    If(String, Box<Expr>, Option<Box<Expr>>),
}

#[derive(Debug)]
pub enum Value {
    Variable(String),
    // Lambda(params, body)
    // Lambda(Vec<String>, Box<Expr>),
    // Closure(free_vars, params, body)
    Closure(Vec<String>, Vec<String>, Box<Expr>),
    Constructor(String, Type, Option<String>, Vec<Value>),
}

#[derive(Debug)]
pub struct Bind(pub Pattern, pub Type, pub Box<Value>, pub Box<Expr>);

#[derive(Debug)]
pub struct Match(pub Vec<(Pattern, Expr)>);

#[derive(Debug)]
pub enum Pattern {
    Wildcard,
    Variable(String),
    Value(Value),
    Constructor(String, Vec<Pattern>),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expr,
}

// pub struct Module {
//   pub functions: Vec<Function>,
// }
