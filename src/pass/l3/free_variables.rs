use std::collections::HashSet;

use crate::ir::l3_ir::{Bind, BindPattern, Body, Compute, If, Match};

impl Body {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Body::Bind(b) => b.free_vars(),
            Body::BindPattern(b) => b.free_vars(),
            Body::If(i) => i.free_vars(),
            Body::Match(m) => m.free_vars(),
            // Body::Compute(c) => c.free_vars(),
            Body::Move(var) => vec![var.clone()].into_iter().collect(),
            Body::Dup(_, e) => e.free_vars(),
            // Body::Dup(_, src_value, e) => {
            //     let mut r = e.free_vars();
            //     r.insert(src_value.clone());
            //     r
            // }
            Body::Drop(_, e) => e.free_vars(),
            Body::DropReuse(_, _, e) => e.free_vars(),
            // Body::DupOnBind(_, _) => unreachable!(),
            // Body::DupOnBind(_, e) => e.free_vars(),
        }
    }
}

impl Compute {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Compute::Invoke(f, args) => {
                let mut r: HashSet<String> = HashSet::new();
                r.insert(f.clone());
                r.extend(args.iter().map(|(name, _)| name.to_string()).clone());
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
        let r: HashSet<String> = self.cont.free_vars();
        // let r: HashSet<String> = r.difference(&self.pat.defined_vars()).cloned().collect();
        r.union(&self.value.free_vars()).cloned().collect()
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
