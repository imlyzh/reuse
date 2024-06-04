use std::collections::HashSet;

use crate::{
    ir::{
        l2_ir::{Bind, BindPattern, Body, Compute, If, Match},
        l3_ir,
    },
    types::Owned,
};

use super::l2::use_check::Used;

impl Body {
    pub fn linearize(self, linears: &HashSet<String>) -> l3_ir::Body {
        match self {
            Body::Bind(b) => b.linearize(linears),
            Body::BindPattern(b) => b.linearize(linears),
            Body::If(i) => l3_ir::Body::If(i.linearize(linears)),
            Body::Match(m) => m.linearize(linears),
            Body::Variable(v) => {
                assert!(linears.contains(&v));
                l3_ir::Body::Move(v)
            }
        }
    }
}

impl Bind {
    pub fn linearize(self, linears: &HashSet<String>) -> l3_ir::Body {
        let compute_used_vars = self.value.free_vars();
        let cont_free_vars = self.cont.free_vars();
        let cont = self.cont.linearize(linears);
        let bind = l3_ir::Bind {
            var: self.var,
            ty: self.ty,
            value: Box::new(self.value.linearize()),
            cont: Box::new(cont),
        };
        compute_used_vars
            .into_iter()
            .filter(|name| linears.contains(name))
            .filter(|name| cont_free_vars.contains(name))
            .fold(l3_ir::Body::Bind(bind), |r, name| {
                l3_ir::Body::Dup(name, Box::new(r))
            })
    }
}

impl Compute {
    /// Trivial
    pub fn linearize(self) -> l3_ir::Compute {
        match self {
            Compute::Invoke(f, args) => l3_ir::Compute::Invoke(f, args),
            Compute::Closure {
                fun_type,
                free_vars,
                params,
                body,
            } => {
                let mut linears = Used::new();
                // FIXME
                body.use_check(&mut linears).unwrap();
                let linears = linears.to_linears();
                l3_ir::Compute::Closure {
                    fun_type,
                    free_vars,
                    params,
                    body: Box::new(body.linearize(&linears)),
                }
            }
            Compute::Constructor(c, ty, args) => l3_ir::Compute::Constructor(c, ty, None, args),
        }
    }
}

impl BindPattern {
    pub fn linearize(self, linears: &HashSet<String>) -> l3_ir::Body {
        let cont_free_vars = self.cont.free_vars();
        let cont = self.cont.linearize(linears);

        if linears.contains(&self.value) {
            let bind = l3_ir::BindPattern {
                pat: self.pat,
                owned: Owned::Linear,
                ty: self.ty,
                value: self.value.clone(),
                cont: Box::new(cont),
            };
            let mut body = l3_ir::Body::BindPattern(bind);
            if cont_free_vars.contains(&self.value) {
                body = l3_ir::Body::Dup(self.value, Box::new(body));
            }
            body
        } else {
            let bind = l3_ir::BindPattern {
                pat: self.pat,
                owned: Owned::Borrow,
                ty: self.ty,
                value: self.value,
                cont: Box::new(cont),
            };
            l3_ir::Body::BindPattern(bind)
        }
    }
}

impl If {
    pub fn linearize(self, linears: &HashSet<String>) -> l3_ir::If {
        let If { cond, then, else_ } = self;
        let then = then.linearize(linears);
        let else_ = else_.linearize(linears);
        l3_ir::If {
            cond,
            then: Box::new(then),
            else_: Box::new(else_),
        }
    }
}

impl Match {
    pub fn linearize(self, linears: &HashSet<String>) -> l3_ir::Body {
        if !linears.contains(&self.value) {
            return l3_ir::Body::Match(l3_ir::Match {
                value: self.value,
                owned: Owned::Borrow,
                matchs: self
                    .matchs
                    .into_iter()
                    .map(|(pat, body)| (pat, body.linearize(linears)))
                    .collect(),
            });
        }
        let body = l3_ir::Body::Match(l3_ir::Match {
            value: self.value.clone(),
            owned: Owned::Borrow,
            matchs: self
                .matchs
                .clone()
                .into_iter()
                .map(|(pat, body)| (pat, body.linearize(linears)))
                .collect(),
        });
        for (_pat, cont) in self.matchs.iter() {
            if cont.free_vars().contains(&self.value) {
                return l3_ir::Body::Dup(self.value, Box::new(body));
            }
        }
        body
    }
}
