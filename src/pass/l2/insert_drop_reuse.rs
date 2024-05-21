use std::borrow::Borrow;

use crate::{
    l2_ir::{Bind, Expr, Match, Pattern, Value},
    types::Type,
};

impl Expr {
    pub fn insert_drop_reuse(self) -> Expr {
        match self {
            Expr::Bind(b) => Expr::Bind(b.insert_drop_reuse()),
            Expr::Match(m) => Expr::Match(m.insert_drop_reuse()),
            Expr::If(c, t, e) => Expr::If(
                c,
                Box::new(t.insert_drop_reuse()),
                e.map(|e| Box::new(e.insert_drop_reuse())),
            ),
            Expr::Invoke(_, _) => self,
            Expr::Dup(s, e) => Expr::Dup(s, Box::new(e.insert_drop_reuse())),
            Expr::Drop(s, e) => Expr::Drop(s, Box::new(e.insert_drop_reuse())),
        }
    }
}

impl Bind {
    pub fn insert_drop_reuse(self) -> Bind {
        let ty = &self.1;
        if let Some(expr) = self.3.match_and_rewrite_constructor(ty) {
            Bind(self.0, self.1, self.2, Box::new(expr))
        } else {
            self
        }
    }
}

impl Expr {
    pub fn match_and_rewrite_constructor(&self, ty: &Type) -> Option<Expr> {
        match self {
            Expr::Invoke(_, _) => None,
            Expr::Bind(bind) => todo!(),
            Expr::Dup(_, e) | Expr::Drop(_, e) => e.match_and_rewrite_constructor(ty),
            Expr::Match(m) => todo!(),
            Expr::If(_, t, e) => {
                if let Some(r) = t.match_and_rewrite_constructor(ty) {
                    Some(r)
                } else {
                    if let Some(e) = e {
                        e.match_and_rewrite_constructor(ty)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl Match {
    pub fn insert_drop_reuse(&self) -> Match {
        todo!()
    }
}

impl Pattern {
    // pub fn use_variable(&self) -> Vec<String> {
    //     match self {
    //         Pattern::Wildcard |
    //         Pattern::Variable(_) => vec![],
    //         Pattern::Value(v) => todo!(),
    //         Pattern::Constructor(_, _) => todo!(),
    //     }
    // }

    pub fn def_variable(&self) -> Vec<String> {
        match self {
            Pattern::Wildcard => todo!(),
            Pattern::Variable(_) => todo!(),
            Pattern::Value(_) => todo!(),
            Pattern::Constructor(_, _) => todo!(),
        }
    }
}

impl Value {
    // pub fn use_variable(&self) -> Vec<String> {
    //     match self {
    //         Value::Variable(_) => vec![],
    //         Value::Closure(free_vars, _, _) => free_vars.to_owned(),
    //         Value::Constructor(_, a, b) => todo!(),
    //     }
    // }
}
