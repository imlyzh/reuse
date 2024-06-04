use std::collections::HashSet;

use crate::{
    ir::{
        l2_ir,
        l3_ir::{Bind, BindPattern, Body, Compute, Function, If, Match},
    },
    types::Owned,
};

use super::l2::use_check::Used;

impl l2_ir::Function {
    pub fn linearize(self, linears: &HashSet<String>) -> Function {
        let body = self.body.linearize(linears);
        Function {
            name: self.name,
            return_type: self.return_type,
            args: self.args,
            body,
        }
    }
}

impl l2_ir::Body {
    pub fn linearize(self, linears: &HashSet<String>) -> Body {
        match self {
            l2_ir::Body::Bind(b) => b.linearize(linears),
            l2_ir::Body::BindPattern(b) => b.linearize(linears),
            l2_ir::Body::If(i) => Body::If(i.linearize(linears)),
            l2_ir::Body::Match(m) => m.linearize(linears),
            l2_ir::Body::Variable(v) => {
                assert!(linears.contains(&v));
                Body::Move(v)
            }
        }
    }
}

impl l2_ir::Bind {
    pub fn linearize(self, linears: &HashSet<String>) -> Body {
        let compute_used_vars = self.value.free_vars();
        let cont_free_vars = self.cont.free_vars();
        let cont = self.cont.linearize(linears);
        let bind = Bind {
            var: self.var,
            ty: self.ty,
            value: Box::new(self.value.linearize()),
            cont: Box::new(cont),
        };
        compute_used_vars
            .into_iter()
            .filter(|name| linears.contains(name))
            .filter(|name| cont_free_vars.contains(name))
            .fold(Body::Bind(bind), |r, name| Body::Dup(name, Box::new(r)))
    }
}

impl l2_ir::Compute {
    /// Trivial
    pub fn linearize(self) -> Compute {
        match self {
            l2_ir::Compute::Invoke(f, args) => Compute::Invoke(f, args),
            l2_ir::Compute::Closure {
                fun_type,
                free_vars,
                params,
                body,
            } => {
                let mut linears = Used::new();
                // FIXME
                body.use_check(&mut linears).unwrap();
                let linears = linears.to_linears();
                Compute::Closure {
                    fun_type,
                    free_vars,
                    params,
                    body: Box::new(body.linearize(&linears)),
                }
            }
            l2_ir::Compute::Constructor(c, ty, args) => Compute::Constructor(c, ty, None, args),
        }
    }
}

impl l2_ir::BindPattern {
    pub fn linearize(self, linears: &HashSet<String>) -> Body {
        let cont_free_vars = self.cont.free_vars();
        let cont = self.cont.linearize(linears);

        if linears.contains(&self.value) {
            let bind = BindPattern {
                pat: self.pat,
                owned: Owned::Linear,
                ty: self.ty,
                value: self.value.clone(),
                cont: Box::new(cont),
            };
            let mut body = Body::BindPattern(bind);
            if cont_free_vars.contains(&self.value) {
                body = Body::Dup(self.value, Box::new(body));
            }
            body
        } else {
            let bind = BindPattern {
                pat: self.pat,
                owned: Owned::Borrow,
                ty: self.ty,
                value: self.value,
                cont: Box::new(cont),
            };
            Body::BindPattern(bind)
        }
    }
}

impl l2_ir::If {
    pub fn linearize(self, linears: &HashSet<String>) -> If {
        let l2_ir::If { cond, then, else_ } = self;
        let then = then.linearize(linears);
        let else_ = else_.linearize(linears);
        If {
            cond,
            then: Box::new(then),
            else_: Box::new(else_),
        }
    }
}

impl l2_ir::Match {
    pub fn linearize(self, linears: &HashSet<String>) -> Body {
        if !linears.contains(&self.value) {
            return Body::Match(Match {
                value: self.value,
                owned: Owned::Borrow,
                matchs: self
                    .matchs
                    .into_iter()
                    .map(|(pat, body)| (pat, body.linearize(linears)))
                    .collect(),
            });
        }
        let body = Body::Match(Match {
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
                return Body::Dup(self.value, Box::new(body));
            }
        }
        body
    }
}
