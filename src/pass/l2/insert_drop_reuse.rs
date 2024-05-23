use std::rc::Rc;

use crate::{
    l2_ir::{Bind, Body, Compute, If, Match},
    types::Type,
    utils::Scope,
};

impl Body {
    pub fn insert_drop_reuse(&mut self, scope: Rc<Scope<Type>>) {
        match self {
            Body::Bind(b) => b.insert_drop_reuse(scope),
            Body::If(i) => i.insert_drop_reuse(scope),
            Body::Match(m) => m.insert_drop_reuse(scope),
            Body::Compute(c) => c.insert_drop_reuse(scope),
            Body::Dup(_, e) => e.insert_drop_reuse(scope),
            Body::Drop(_, e) => e.insert_drop_reuse(scope),
        }
    }
}

impl Compute {
    pub fn insert_drop_reuse(&mut self, scope: Rc<Scope<Type>>) {
        match self {
            Compute::Closure {
                free_vars: _,
                params: _,
                body,
            } => body.insert_drop_reuse(scope),
            Compute::Variable(_) => {}
            Compute::Invoke(_, _) => {}
            Compute::Constructor(_, _, _, _) => {}
        }
    }
}

impl Bind {
    pub fn insert_drop_reuse(&mut self, scope: Rc<Scope<Type>>) {
        self.2.insert_drop_reuse(scope.clone());
        self.3.insert_drop_reuse(scope);
    }
}

impl If {
    pub fn insert_drop_reuse(&mut self, scope: Rc<Scope<Type>>) {
        self.1.insert_drop_reuse(scope.clone());
        self.2.insert_drop_reuse(scope);
    }
}

impl Match {
    pub fn insert_drop_reuse(&mut self, scope: Rc<Scope<Type>>) {
        for (_, body) in self.1.iter_mut() {
            body.insert_drop_reuse(scope.clone());
        }
    }
}
