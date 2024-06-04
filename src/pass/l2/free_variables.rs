use std::collections::HashSet;

use crate::{
    ir::l2_ir::{Bind, BindPattern, Body, Compute, If, Match},
    types::Owned,
};

impl Body {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Body::Bind(b) => b.free_vars(),
            Body::BindPattern(b) => b.free_vars(),
            Body::If(i) => i.free_vars(),
            Body::Match(m) => m.free_vars(),
            Body::Variable(var) => vec![var.clone()].into_iter().collect(),
        }
    }
}

impl Bind {
    pub fn free_vars(&self) -> HashSet<String> {
        let r: HashSet<String> = self.cont.free_vars();
        // let r: HashSet<String> = r.difference(&self.pat.defined_vars()).cloned().collect();
        r.union(&self.value.free_vars()).cloned().collect()
    }
}

impl Compute {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Compute::Invoke(f, args) => {
                let mut r: HashSet<String> = HashSet::new();
                r.insert(f.clone());
                r.extend(args.iter().map(|(name, _)| name.to_string()));
                r.into_iter().collect()
            }
            Compute::Closure { free_vars, .. } => free_vars.iter().cloned().collect(),
            // Compute::Constructor(c, _ty, _reuse, params) => {
            Compute::Constructor(c, _ty, params) => {
                let mut r: HashSet<String> = params.iter().cloned().collect();
                r.insert(c.clone());
                r
            }
        }
    }
}

impl Compute {
    pub fn free_linear_vars(&self, linears: &HashSet<String>) -> HashSet<String> {
        match self {
            Compute::Invoke(f, args) => {
                let mut r: HashSet<String> = HashSet::new();
                if linears.contains(f) {
                    r.insert(f.clone());
                }
                r.extend(args.iter().filter_map(|(name, owned)| {
                    if let Owned::Linear = owned {
                        Some(name.to_string())
                    } else {
                        None
                    }
                }));
                r.into_iter().collect()
            }
            Compute::Closure { free_vars, .. } => free_vars.iter().cloned().collect(),
            // Compute::Constructor(c, _ty, _reuse, params) => {
            Compute::Constructor(c, _ty, params) => {
                let mut r: HashSet<String> = params.iter().cloned().collect();
                r.insert(c.clone());
                r
            }
        }
    }
}

impl BindPattern {
    pub fn free_vars(&self) -> HashSet<String> {
        let mut r: HashSet<String> = self.cont.free_vars();
        // let r: HashSet<String> = r.difference(&self.pat.defined_vars()).cloned().collect();
        r.insert(self.value.clone());
        r
    }
}

impl If {
    pub fn free_vars(&self) -> HashSet<String> {
        let If { cond, then, else_ } = self;
        let mut r: HashSet<String> = HashSet::new();
        r.insert(cond.clone());
        r.extend(then.free_vars());
        r.extend(else_.free_vars());
        r
    }
}

impl Match {
    pub fn free_vars(&self) -> HashSet<String> {
        let mut r = self
            .matchs
            .iter()
            .map(|(pat, expr)| {
                expr.free_vars()
                    .difference(&pat.defined_vars())
                    .cloned()
                    .collect::<HashSet<String>>()
            })
            .reduce(|l, r| HashSet::union(&l, &r).cloned().collect())
            .unwrap_or(HashSet::new());
        r.insert(self.value.clone());
        r
    }
}
