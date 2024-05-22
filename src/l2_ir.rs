use crate::types::Type;

#[derive(Debug)]
pub enum Expr {
    Invoke(Value, Vec<Value>),
    // Bind(Pattern, Box<Expr>, Box<Expr>),
    Bind(Bind),
    Dup(String, String),
    Drop(String, String),
    Match(Match),
    If(Value, Box<Expr>, Option<Box<Expr>>),
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
pub struct Match(pub Value, pub Vec<(Pattern, Expr)>);

#[derive(Debug)]
pub enum Pattern {
    Wildcard,
    Variable(String),
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
