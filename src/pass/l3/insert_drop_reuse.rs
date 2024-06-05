use std::collections::{HashMap, HashSet};

use crate::{
    ir::l3_ir::{Bind, BindPattern, Body, Compute, Function, If, Match, Name},
    types::{Owned, Type},
};

use super::use_check::Used;

impl Function {
    pub fn insert_drop_reuse(self) -> Self {
        let mut reuse_types = HashMap::new();

        for (name, (ty, owned)) in self.args.iter() {
            if let Owned::Linear = owned {
                reuse_types.insert(name.clone(), ty.clone());
            }
        }

        let mut used_record = Used::new();
        self.body.use_check(&mut used_record).unwrap();
        let mut linears: HashMap<String, usize> = used_record
            .to_linears()
            .into_iter()
            .map(|name| (name, 1))
            .collect();

        let body = self.body.insert_drop_reuse(&mut linears, &mut reuse_types);

        Function { body, ..self }
    }
}

impl Body {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        // order problem
        let free_vars = self.free_vars();
        let drop_vars: HashSet<String> = linears
            .iter()
            .filter(|(_, ref_count)| **ref_count == 1)
            .filter(|(var, _)| !free_vars.contains(*var))
            .map(|(name, _)| name.to_string())
            // .map(|(name, _)| (name.clone(), reuse_types.get(name).unwrap().clone()))
            .collect();

        drop_vars.iter().for_each(|name| {
            linears.remove(name).unwrap();
        });

        let mut body = self.process_match_raw(linears, reuse_types);

        for var in drop_vars {
            let ty = reuse_types.get(&var).unwrap();

            body = if let Some(new_body) = body.rewrite_construct(&var, ty) {
                Body::DropReuse(format!("__reuse_{}", &var), var.clone(), Box::new(new_body))
            } else {
                Body::Drop(var, Box::new(body))
            };
        }
        body
    }

    // pub fn insert_drop_reuse(
    pub fn process_match_raw(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        match self {
            Body::Bind(e) => Body::Bind(e.insert_drop_reuse(linears, reuse_types)),
            Body::BindPattern(e) => Body::BindPattern(e.insert_drop_reuse(linears, reuse_types)),
            Body::If(e) => Body::If(e.insert_drop_reuse(linears, reuse_types)),
            Body::Match(e) => Body::Match(e.insert_drop_reuse(linears, reuse_types)),
            Body::Move(_) => self,
            Body::Dup(name, e) => {
                linears.insert(name.clone(), linears.get(&name).unwrap() + 1);
                Body::Dup(name, Box::new(e.insert_drop_reuse(linears, reuse_types)))
            }
            Body::Drop(name, e) => {
                if *linears.get(&name).unwrap() != 1 {
                    linears.insert(name.clone(), linears.get(&name).unwrap() + 1);
                } else {
                    linears.remove(&name);
                }
                Body::Drop(name, Box::new(e.insert_drop_reuse(linears, reuse_types)))
            }
            Body::DropReuse(new_name, name, e) => Body::DropReuse(
                new_name,
                name,
                Box::new(e.insert_drop_reuse(linears, reuse_types)),
            ),
        }
    }
}

impl Bind {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        // add pattern to new env
        reuse_types.insert(self.var.clone(), self.ty.clone());

        // run pass
        let value = self.value.insert_drop_reuse(linears, reuse_types);
        let cont = self.cont.insert_drop_reuse(linears, reuse_types);

        Bind {
            var: self.var,
            ty: self.ty,
            value: Box::new(value),
            cont: Box::new(cont),
        }
    }
}

impl BindPattern {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        // add pattern to new env
        if let Owned::Linear = self.owned {
            for (name, ty) in self.pat.type_binding(&self.ty).into_iter() {
                reuse_types.insert(name, ty);
            }
        }

        let pat_defined_vars = &self.pat.defined_vars();

        let cont = self.cont;

        // run pass
        let cont = cont.insert_drop_reuse(linears, reuse_types);

        // insert DUP to Pattern Bind after
        let cont = pat_defined_vars
            .iter()
            .filter(|name| linears.contains_key(*name))
            .fold(cont, |body, var| Body::Dup(var.clone(), Box::new(body)));

        BindPattern {
            pat: self.pat,
            owned: self.owned,
            ty: self.ty,
            value: self.value,
            cont: Box::new(cont),
        }
    }
}

impl Compute {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        match self {
            Compute::Closure {
                fun_type,
                free_vars,
                params,
                body,
            } => {
                for name in free_vars.clone() {
                    if *linears.get(&name).unwrap() != 1 {
                        linears.insert(name.clone(), linears.get(&name).unwrap() + 1);
                    } else {
                        linears.remove(&name);
                    }
                }

                let mut new_reuse_types = HashMap::new();

                // add free_vars to linear
                for name in free_vars.iter() {
                    let ty = reuse_types.get(name).unwrap().clone();
                    new_reuse_types.insert(name.clone(), ty);
                }

                // add params to linear and borrow
                for (index, name) in params.iter().enumerate() {
                    let (ty, owned) = fun_type.params.get(index).unwrap();
                    if let Owned::Linear = owned {
                        new_reuse_types.insert(name.to_string(), ty.clone());
                    }
                }

                let mut used_record = Used::new();
                body.use_check(&mut used_record).unwrap();
                let mut new_linears: HashMap<String, usize> = used_record
                    .to_linears()
                    .into_iter()
                    .map(|name| (name, 1))
                    .collect();

                let body = body.insert_drop_reuse(&mut new_linears, &mut new_reuse_types);

                Compute::Closure {
                    fun_type,
                    free_vars: free_vars.clone(),
                    params,
                    body: Box::new(body),
                }
            }
            Compute::Invoke(ref f, ref args) => {
                if let Some(1) = linears.get(f) {
                    linears.insert(f.clone(), linears.get(f).unwrap() + 1);
                } else {
                    linears.remove(f);
                }

                for name in args
                    .iter()
                    .filter(|(_, owned)| matches!(owned, Owned::Linear))
                    .map(|(name, _)| name)
                {
                    if *linears.get(name).unwrap() != 1 {
                        linears.insert(name.clone(), linears.get(name).unwrap() + 1);
                    } else {
                        linears.remove(name);
                    }
                }
                self
            }
            Compute::Constructor(_, _, _, ref args) => {
                for name in args.iter() {
                    if *linears.get(name).unwrap() != 1 {
                        linears.insert(name.clone(), linears.get(name).unwrap() + 1);
                    } else {
                        linears.remove(name);
                    }
                }
                self
            }
        }
    }
}

impl If {
    /// Trivial
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        let borrowed_self = self;
        let it1 = borrowed_self.then.insert_drop_reuse(linears, reuse_types);
        let it2 = borrowed_self.else_.insert_drop_reuse(linears, reuse_types);
        If {
            cond: borrowed_self.cond,
            then: Box::new(it1),
            else_: Box::new(it2),
        }
    }
}

impl Match {
    /// Notice
    pub fn insert_drop_reuse(
        self,
        linears: &mut HashMap<String, usize>,
        reuse_types: &mut HashMap<Name, Type>,
    ) -> Self {
        let ty: Type = reuse_types.get(&self.value).unwrap().clone();

        let mut new_liveness = HashSet::new();
        let mut pairs = Vec::new();

        for (pat, body) in self.matchs.into_iter() {
            // add pattern to new env
            if let Owned::Linear = self.owned {
                for (name, ty) in pat.type_binding(&ty).into_iter() {
                    reuse_types.insert(name, ty);
                }
            }

            let pat_defined_vars = pat.defined_vars();

            // run pass
            let body = body.insert_drop_reuse(linears, reuse_types);

            // insert DUP to Pattern Bind after
            let body = pat_defined_vars
                .into_iter()
                .filter(|name| linears.contains_key(name))
                .fold(body, |body, var| Body::Dup(var.clone(), Box::new(body)));

            pairs.push((pat, body));
        }
        new_liveness.insert(self.value.clone());
        Match {
            value: self.value,
            owned: self.owned,
            matchs: pairs,
        }
    }
}
