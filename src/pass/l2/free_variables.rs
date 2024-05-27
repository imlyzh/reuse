use std::collections::HashSet;

use crate::ir::l2_ir::{Bind, Body, Compute, If, Match, Pattern};

impl Body {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Body::Bind(b) => b.free_vars(),
            Body::If(i) => i.free_vars(),
            Body::Match(m) => m.free_vars(),
            Body::Compute(c) => c.free_vars(),
            Body::Dup(_, e) => e.free_vars(),
            Body::Drop(_, e) => e.free_vars(),
            Body::DropReuse(_, _, e) => e.free_vars(),
        }
    }
}

impl Compute {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Compute::Variable(v) => vec![v.clone()].into_iter().collect(),
            Compute::Invoke(f, args) => {
                let mut r: HashSet<String> = HashSet::new();
                r.insert(f.clone());
                r.extend(args.clone());
                r.into_iter().collect()
            }
            Compute::Closure { free_vars, .. } => free_vars.iter().cloned().collect(),
            // Compute::Constructor(c, _ty, _reuse, params) => {
            Compute::Constructor(c, _ty, None, params) => {
                let mut r: HashSet<String> = params.iter().cloned().collect();
                r.insert(c.clone());
                r
            }
            Compute::Constructor(..) => HashSet::new(),
        }
    }
}

impl Bind {
    pub fn free_vars(&self) -> HashSet<String> {
        let r: HashSet<String> = self.3.free_vars();
        let r: HashSet<String> = r.difference(&self.0.defined_vars()).cloned().collect();
        r.union(&self.2.free_vars()).cloned().collect()
    }
}

impl If {
    pub fn free_vars(&self) -> HashSet<String> {
        let If(c, t, e) = self;
        let mut r: HashSet<String> = HashSet::new();
        r.insert(c.clone());
        r.extend(t.free_vars());
        r.extend(e.free_vars());
        r
    }
}

impl Match {
    pub fn free_vars(&self) -> HashSet<String> {
        let mut r = self
            .1
            .iter()
            .map(|(pat, expr)| {
                expr.free_vars()
                    .difference(&pat.defined_vars())
                    .cloned()
                    .collect::<HashSet<String>>()
            })
            .reduce(|l, r| HashSet::union(&l, &r).cloned().collect())
            .unwrap_or(HashSet::new());
        r.insert(self.0.clone());
        r
    }
}

impl Pattern {
    pub fn defined_vars(&self) -> HashSet<String> {
        match self {
            Pattern::Wildcard => HashSet::new(),
            Pattern::Variable(v) => vec![v.clone()].into_iter().collect(),
            Pattern::Constructor(_, v) => v
                .iter()
                .map(Pattern::defined_vars)
                .reduce(|l, r| HashSet::union(&l, &r).cloned().collect())
                .unwrap_or(HashSet::new()),
        }
    }
}
