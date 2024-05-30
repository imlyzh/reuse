use std::collections::{HashMap, HashSet};

use crate::{
    ir::l3_ir::{Bind, BindPattern, Body, Compute, Function, If, Match, Name, Owned},
    types::Type,
};

impl Function {
    pub fn insert_drop_reuse(self) -> (Self, HashSet<Name>) {
        let mut linear = HashMap::new();
        let mut borrow = HashMap::new();
        for (name, (ty, owned)) in self.args.iter() {
            if let Owned::Linear = owned {
                linear.insert(name.clone(), ty.clone());
            } else {
                borrow.insert(name.clone(), ty.clone());
            }
        }
        let (body, liveness) = self.body.insert_drop_reuse(linear, borrow);
        // TODO: liveness difference params
        (Function { body, ..self }, liveness)
    }
}

impl Body {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linear: HashMap<Name, Type>,
        borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        // order problem
        // let mut body = self.clone();
        // let mut team_linear = linar.clone();
        // let linear_set = linear.iter().map(|(k, _)| k.clone()).collect();
        let free_vars = self.free_vars();
        let drop_vars: HashMap<String, Type> = linear
            .clone()
            .into_iter()
            .filter(|(var, _)| !free_vars.contains(var))
            .collect();
        let new_linear: HashMap<String, Type> = linear
            .into_iter()
            .filter(|(var, _)| free_vars.contains(var))
            .collect();
        let (mut body, liveness) = self.process_match_raw(new_linear, borrow.clone());
        for (var, ty) in drop_vars {
            // if variable is not live
            // if !liveness.contains(&var.0) {
            // FIXME
            if !body.free_vars().contains(&var) {
                body = if let Some(new_body) = body.rewrite_construct(&var, &ty) {
                    Body::DropReuse(format!("__reuse_{}", &var), var.clone(), Box::new(new_body))
                } else {
                    Body::Drop(var, Box::new(body))
                };
                // body = Body::Drop(var.0.clone(), Box::new(body));
            }
        }
        (body, liveness)
    }

    // pub fn insert_drop_reuse(
    pub fn process_match_raw(
        self,
        linear: HashMap<Name, Type>,
        borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        match self {
            Body::Bind(b) => {
                let (r, liveness) = b.insert_drop_reuse(linear, borrow);
                (Body::Bind(r), liveness)
            }
            Body::BindPattern(b) => {
                let (r, liveness) = b.insert_drop_reuse(linear, borrow);
                (Body::BindPattern(r), liveness)
            }
            Body::If(i) => {
                let (r, liveness) = i.insert_drop_reuse(linear, borrow);
                (Body::If(r), liveness)
            }
            Body::Match(m) => {
                let (r, liveness) = m.insert_drop_reuse(linear, borrow);
                (Body::Match(r), liveness)
            }
            Body::Compute(c) => {
                let liveness = c.free_vars();
                (Body::Compute(c), liveness)
            }
            Body::Dup(name, e) => {
                let (r, liveness) = e.insert_drop_reuse(linear, borrow);
                (Body::Dup(name, Box::new(r)), liveness)
            }
            // Body::Dup(dst_name, src_name, e) => {
            //     linear.insert(dst_name.clone(), linear.get(&src_name).unwrap().clone());
            //     let (r, mut liveness) = e.insert_drop_reuse(linear, borrow);
            //     liveness.insert(src_name.clone());
            //     (Body::Dup(src_name, dst_name, Box::new(r)), liveness)
            // }
            Body::Drop(name, e) => {
                let (r, liveness) = e.insert_drop_reuse(linear, borrow);
                (Body::Drop(name, Box::new(r)), liveness)
            }
            Body::DropReuse(new_name, name, e) => {
                let (r, liveness) = e.insert_drop_reuse(linear, borrow);
                (Body::DropReuse(new_name, name, Box::new(r)), liveness)
            } // Body::DupOnBind(_, _) => unreachable!(),
        }
    }
}

impl Bind {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        mut linear: HashMap<Name, Type>,
        mut borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        // add pattern to new env
        if let Owned::Linear = self.owned {
            linear.insert(self.var.clone(), self.ty.clone());
        } else {
            borrow.insert(self.var.clone(), self.ty.clone());
        }
        // let pat_deefined_vars = &self.pat.defined_vars();

        // run pass
        let (cont, mut liveness) = self.cont.insert_drop_reuse(linear.clone(), borrow.clone());
        liveness.remove(&self.var);

        let (value, it2_free_vars) = self.value.insert_drop_reuse(linear.clone(), borrow.clone());
        let liveness = liveness.union(&it2_free_vars).cloned().collect();

        // TODO
        // get bind used variable, liveness check, try rewrite
        /* Disable option
        let mut cont = cont;
        if let Owned::Linear = self.owned {
            cont = it2_free_vars.into_iter().fold(cont, |body, var|
            // is linear
            if !liveness.contains(&var) {
                // find var type
                let ty: Type = find_variable(&linear, &var).unwrap();
                // try rewrite construct
                if let Some(new_body) = body.rewrite_construct(&var, &ty) {
                    Body::DropReuse(format!("__reuse_{}", var), var, Box::new(new_body))
                } else {
                    Body::Drop(var, Box::new(body))
                }
            } else {
                body
            });
        } else {
            cont
        };
        // */

        // insert DUP to Pattern Bind after
        /* Disable option
        let cont = pat_deefined_vars.iter().fold(cont, |body, var| {
            Body::DupOnBind(var.clone(), Box::new(body))
        });
        // */
        (
            Bind {
                var: self.var,
                owned: self.owned,
                ty: self.ty,
                value: Box::new(value),
                cont: Box::new(cont),
            },
            liveness,
        )
    }
}

impl BindPattern {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        mut linear: HashMap<Name, Type>,
        mut borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        // add pattern to new env
        if let Owned::Linear = self.owned {
            for (name, ty) in self.pat.type_binding(&self.ty).into_iter() {
                linear.insert(name, ty);
            }
        } else {
            for (name, ty) in self.pat.type_binding(&self.ty).into_iter() {
                borrow.insert(name, ty);
            }
        }

        let pat_deefined_vars = &self.pat.defined_vars();

        // run pass
        let (cont, liveness) = self.cont.insert_drop_reuse(linear.clone(), borrow.clone());
        let mut liveness: HashSet<Name> = liveness.difference(pat_deefined_vars).cloned().collect();

        liveness.insert(self.value.clone());

        // TODO
        // get bind used variable, liveness check, try rewrite
        /* Disable option
        let mut cont = cont;
        if let Owned::Linear = self.owned {
            cont = it2_free_vars.into_iter().fold(cont, |body, var|
            // is linear
            if !liveness.contains(&var) {
                // find var type
                let ty: Type = find_variable(&linear, &var).unwrap();
                // try rewrite construct
                if let Some(new_body) = body.rewrite_construct(&var, &ty) {
                    Body::DropReuse(format!("__reuse_{}", var), var, Box::new(new_body))
                } else {
                    Body::Drop(var, Box::new(body))
                }
            } else {
                body
            });
        } else {
            cont
        };
        // */

        // insert DUP to Pattern Bind after
        // /* Disable option
        let cont = pat_deefined_vars
            .iter()
            .fold(cont, |body, var| Body::Dup(var.clone(), Box::new(body)));
        // */
        (
            BindPattern {
                pat: self.pat,
                owned: self.owned,
                ty: self.ty,
                value: self.value,
                cont: Box::new(cont),
            },
            liveness,
        )
    }
}

impl Compute {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        mut linear: HashMap<Name, Type>,
        mut borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        match self {
            Compute::Closure {
                fun_type,
                free_vars,
                params,
                body,
            } => {
                // add free_vars to linear
                for name in free_vars.iter() {
                    let ty = linear.get(name).unwrap().clone();
                    linear.insert(name.clone(), ty);
                }

                // add params to linear and borrow
                for (index, (name, owned)) in params.iter().enumerate() {
                    let ty = fun_type.params.get(index).unwrap();
                    if let Owned::Linear = owned {
                        linear.insert(name.to_string(), ty.clone());
                    } else {
                        borrow.insert(name.to_string(), ty.clone());
                    }
                }

                let (body, _) = body.insert_drop_reuse(linear, borrow);
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
    pub fn insert_drop_reuse(
        self,
        linear: HashMap<Name, Type>,
        borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        let borrowed_self = self;
        let (it1, mut liveness1) = borrowed_self
            .then
            .insert_drop_reuse(linear.clone(), borrow.clone());
        let (it2, liveness2) = borrowed_self.else_.insert_drop_reuse(linear, borrow);
        liveness1.extend(liveness2);
        liveness1.insert(borrowed_self.cond.clone());
        (
            If {
                cond: borrowed_self.cond,
                then: Box::new(it1),
                else_: Box::new(it2),
            },
            liveness1,
        )
    }
}

impl Match {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linear: HashMap<Name, Type>,
        borrow: HashMap<Name, Type>,
    ) -> (Self, HashSet<Name>) {
        let ty: &Type = linear.get(&self.value).unwrap();

        let mut new_liveness = HashSet::new();
        let mut pairs = Vec::new();

        for (pat, body) in self.matchs.into_iter() {
            let mut new_linear = linear.clone();
            let mut new_borrow = borrow.clone();
            // add pattern to new env
            if let Owned::Linear = self.owned {
                for (name, ty) in pat.type_binding(ty).into_iter() {
                    new_linear.insert(name, ty);
                }
            } else {
                for (name, ty) in pat.type_binding(ty).into_iter() {
                    new_borrow.insert(name, ty);
                }
            }

            let pat_deefined_vars = pat.defined_vars();

            // run pass
            let (mut body, liveness) = body.insert_drop_reuse(new_linear, new_borrow);
            let liveness: HashSet<Name> =
                liveness.difference(&pat_deefined_vars).cloned().collect();

            // bind is linear
            /* Disable option
            if let Owned::Linear = self.owned {
                if !liveness.contains(&self.value) {
                    body = if let Some(new_body) = body.rewrite_construct(&self.value, &ty) {
                        Body::DropReuse(
                            format!("__reuse_{}", self.value.clone()),
                            self.value.clone(),
                            Box::new(new_body),
                        )
                    } else {
                        Body::Drop(self.value.clone(), Box::new(body))
                    };
                }
            }
            // */

            // insert DUP to Pattern Bind after
            // /* Disable option
            body = pat_deefined_vars
                .into_iter()
                .fold(body, |body, var| Body::Dup(var.clone(), Box::new(body)));
            // */
            new_liveness.extend(liveness);
            pairs.push((pat, body));
        }
        new_liveness.insert(self.value.clone());
        (
            Match {
                value: self.value,
                owned: self.owned,
                matchs: pairs,
            },
            new_liveness,
        )
    }
}
