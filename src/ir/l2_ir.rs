use crate::types::{FunctionType, Owned, Type};

use super::common::{Name, Pattern};

#[derive(Debug, Clone)]
pub enum Body {
    Variable(Name),
    Bind(Bind),
    BindPattern(BindPattern),
    If(If),
    Match(Match),
}

#[derive(Debug, Clone)]
pub enum Compute {
    // Variable(Name),
    Invoke(Name, Vec<(Name, Owned)>),
    Closure {
        fun_type: FunctionType,
        free_vars: Vec<Name>,
        params: Vec<Name>,
        body: Box<Body>,
    },
    Constructor(String, Type, Vec<Name>),
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub var: Name,
    pub ty: Type,
    pub value: Box<Compute>,
    pub cont: Box<Body>,
}

#[derive(Debug, Clone)]
pub struct BindPattern {
    pub pat: Pattern,
    pub ty: Type,
    pub value: Name,
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
    pub matchs: Vec<(Pattern, Body)>,
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
