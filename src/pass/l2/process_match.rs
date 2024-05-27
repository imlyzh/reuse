use std::{collections::HashSet, rc::Rc};

use crate::{
    ir::l2_ir::{Bind, Body, Compute, If, Match, Name},
    types::Type,
    utils::Scope,
};

impl Body {
    /// Notice
    pub fn process_match(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        // order problem
        let (mut body, liveness) = self.process_match_raw(env.clone());
        let mut env = Some(env);
        while let Some(var) = env {
            // if variable is not live
            if !liveness.contains(&var.0) {
                body = if let Some(new_body) = body.rewrite_construct(&var.0, &var.1) {
                    Body::DropReuse(
                        format!("__reuse_{}", &var.0),
                        var.0.clone(),
                        Box::new(new_body),
                    )
                } else {
                    Body::Drop(var.0.clone(), Box::new(body))
                };
            }
            env = var.2.clone();
        }
        (body, liveness)
    }

    /// trivial
    pub fn process_match_raw(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        match self {
            Body::Bind(b) => {
                let (r, liveness) = b.process_match(env);
                (Body::Bind(r), liveness)
            }
            Body::If(i) => {
                let (r, liveness) = i.process_match(env);
                (Body::If(r), liveness)
            }
            Body::Match(m) => {
                let (r, liveness) = m.process_match(env);
                (Body::Match(r), liveness)
            }
            Body::Compute(c) => {
                let liveness = c.free_vars();
                (Body::Compute(c), liveness)
            }
            Body::Dup(name, e) => {
                let (r, liveness) = e.process_match(env);
                (Body::Dup(name, Box::new(r)), liveness)
            }
            Body::Drop(name, e) => {
                let (r, liveness) = e.process_match(env);
                (Body::Drop(name, Box::new(r)), liveness)
            }
            Body::DropReuse(new_name, name, e) => {
                let (r, liveness) = e.process_match(env);
                (Body::DropReuse(new_name, name, Box::new(r)), liveness)
            }
        }
    }
}

impl Bind {
    /// Notice
    pub fn process_match(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        // add pattern to new env
        let env = self
            .0
            .type_binding(&self.1)
            .into_iter()
            .fold(env.clone(), |env, (name, ty)| {
                Rc::new(Scope(name, ty, Some(env)))
            });

        let pat_deefined_vars = self.0.defined_vars();

        // run pass
        let (body, liveness) = self.3.process_match(env.clone());
        let liveness: HashSet<Name> = liveness.difference(&pat_deefined_vars).cloned().collect();

        let (compute, compute_used_vars) = self.2.process_match(env.clone());
        let liveness: HashSet<Name> = liveness.union(&compute_used_vars).cloned().collect();

        // insert DUP to Pattern Bind after
        let body = pat_deefined_vars
            .into_iter()
            .fold(body, |body, var| Body::Dup(var, Box::new(body)));

        (
            Bind(self.0, self.1, Box::new(compute), Box::new(body)),
            liveness,
        )
    }
}

impl Compute {
    /// Notice
    pub fn process_match(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        match self {
            Compute::Closure {
                fun_type,
                free_vars,
                params,
                body,
            } => {
                // add free_vars to env
                let env = free_vars.iter().fold(env, |env, name| {
                    let ty = env.find_variable(name).unwrap();
                    Rc::new(Scope(name.clone(), ty.clone(), Some(env)))
                });
                // add params to env
                let env = params.iter().enumerate().fold(env, |env, (index, name)| {
                    let ty = fun_type.params.get(index).unwrap();
                    Rc::new(Scope(name.clone(), ty.clone(), Some(env)))
                });
                let (body, _) = body.process_match(env);
                // notice: closure_liveness equal to free_vars
                // debug_assert_eq!(closure_liveness, free_vars.iter().cloned().collect());
                (
                    Compute::Closure {
                        fun_type,
                        free_vars: free_vars.clone(),
                        params,
                        body: Box::new(body),
                    },
                    free_vars.into_iter().collect(),
                )
            }
            _ => {
                let free_vars = self.free_vars();
                (self, free_vars)
            }
        }
    }
}

impl If {
    /// Trivial
    pub fn process_match(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
        let borrowed_self = self;
        let (it1, mut liveness1) = borrowed_self.1.process_match(env.clone());
        let (it2, liveness2) = borrowed_self.2.process_match(env);
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
    pub fn process_match(self, env: Rc<Scope<Type>>) -> (Self, HashSet<Name>) {
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

            let pat_deefined_vars = pat.defined_vars();

            // run pass
            let (body, liveness) = body.process_match(env);
            let liveness: HashSet<Name> =
                liveness.difference(&pat_deefined_vars).cloned().collect();

            // insert DUP to Pattern Bind after
            let body = pat_deefined_vars
                .into_iter()
                .fold(body, |body, var| Body::Dup(var, Box::new(body)));

            new_liveness.extend(liveness);
            pairs.push((pat, body));
        }
        (Match(self.0.clone(), pairs), new_liveness)
    }
}
