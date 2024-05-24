use std::{collections::HashSet, rc::Rc};

use crate::{
    l2_ir::{Bind, Body, If, Match, Name},
    types::Type,
    utils::Scope,
};

/// Move == drop/drop-reuse

impl Body {
    /// trivial
    pub fn insert_reuse_from_liveness(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        match self {
            Body::Bind(b) => {
                let (r, liveness) = b.insert_reuse_from_liveness(env);
                (Body::Bind(r), liveness)
            }
            Body::If(i) => {
                let (r, liveness) = i.insert_reuse_from_liveness(env);
                (Body::If(r), liveness)
            }
            Body::Match(m) => {
                let (r, liveness) = m.insert_reuse_from_liveness(env);
                (Body::Match(r), liveness)
            }
            Body::Compute(c) => {
                let liveness = c.free_vars();
                (Body::Compute(c), liveness)
            }
            Body::Dup(name, e) => {
                let (r, liveness) = e.insert_reuse_from_liveness(env);
                (Body::Dup(name, Box::new(r)), liveness)
            }
            Body::Drop(name, e) => {
                let (r, liveness) = e.insert_reuse_from_liveness(env);
                (Body::Drop(name, Box::new(r)), liveness)
            }
            Body::DropReuse(new_name, name, e) => {
                let (r, liveness) = e.insert_reuse_from_liveness(env);
                (Body::DropReuse(new_name, name, Box::new(r)), liveness)
            }
        }
    }
}

impl Bind {
    /// Notice
    pub fn insert_reuse_from_liveness(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        // add pattern to new env
        let env = self
            .0
            .type_binding(&self.1)
            .into_iter()
            .fold(env.clone(), |scope, (name, ty)| {
                Rc::new(Scope(name, ty, Some(scope)))
            });

        // run pass
        let (body, liveness) = self.3.insert_reuse_from_liveness(env.clone());
        let liveness: HashSet<Name> = liveness
            .difference(&self.0.defined_vars())
            .cloned()
            .collect();

        // get variable, liveness check, try rewrite
        let body = self.2.free_vars().into_iter().fold(body, |body, var|
            // is linear
            if !liveness.contains(&var) {
                // find var type
                let ty: &Type = env.find_variable(&var).unwrap();
                // try rewrite construct
                if let Some(new_body) = body.rewrite_construct(&var, ty) {
                    Body::DropReuse(format!("__reuse_{}", var), var, Box::new(new_body))
                } else {
                    Body::Drop(var, Box::new(body))
                }
            } else {
                body
            });

        // TODO: body insert DUP
        (Bind(self.0, self.1, self.2, Box::new(body)), liveness)
    }
}

impl If {
    /// Trivial
    pub fn insert_reuse_from_liveness(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        let borrowed_self = self;
        let (it1, mut liveness1) = borrowed_self.1.insert_reuse_from_liveness(env.clone());
        let (it2, liveness2) = borrowed_self.2.insert_reuse_from_liveness(env);
        liveness1.extend(liveness2);
        liveness1.insert(borrowed_self.0.clone());
        (
            If(borrowed_self.0.clone(), Box::new(it1), Box::new(it2)),
            liveness1,
        )
    }
}

impl Match {
    /// Notice
    pub fn insert_reuse_from_liveness(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        let ty: &Type = env.find_variable(self.0.as_str()).unwrap();

        let mut new_liveness = HashSet::new();
        let mut pairs = Vec::new();

        for (pat, body) in self.1.into_iter() {
            // add pattern to new env
            let env = pat
                .type_binding(ty)
                .into_iter()
                .fold(env.clone(), |scope, (name, ty)| {
                    Rc::new(Scope(name, ty, Some(scope)))
                });

            // run pass
            let (mut body, liveness) = body.insert_reuse_from_liveness(env);
            let liveness: HashSet<Name> =
                liveness.difference(&pat.defined_vars()).cloned().collect();

            // is linear
            if !liveness.contains(&self.0) {
                body = if let Some(new_body) = body.rewrite_construct(&self.0, ty) {
                    Body::DropReuse(
                        format!("__reuse_{}", self.0),
                        self.0.clone(),
                        Box::new(new_body),
                    )
                } else {
                    Body::Drop(self.0.clone(), Box::new(body))
                };
            }

            // TODO: body insert DUP
            new_liveness.extend(liveness);
            pairs.push((pat, body));
        }
        (Match(self.0.clone(), pairs), new_liveness)
    }
}
