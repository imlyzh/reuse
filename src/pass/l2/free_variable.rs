use std::collections::HashSet;

use crate::l2_ir::{Bind, Expr, Match, Pattern, Value};

impl Expr {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Expr::Invoke(f, args) => {
                let mut r: HashSet<String> = f.free_vars();
                let args = args
                    .iter()
                    .map(Value::free_vars)
                    .reduce(|l, r| HashSet::union(&l, &r).cloned().collect())
                    .unwrap_or(HashSet::new());
                r.extend(args);
                r.into_iter().collect()
            }
            Expr::Bind(b) => b.free_vars(),
            Expr::Match(m) => m.free_vars(),
            Expr::If(c, t, e) => {
                let r: HashSet<String> = c.free_vars();
                let r: HashSet<String> = r.union(&t.free_vars()).cloned().collect();
                if let Some(e) = e {
                    r.union(&e.free_vars()).cloned().collect()
                } else {
                    r
                }
            }
            Expr::Dup(_, _) => unreachable!(),
            Expr::Drop(_, _) => unreachable!(),
        }
    }
}

impl Value {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Value::Variable(v) => vec![v.clone()].into_iter().collect(),
            Value::Closure(c, _, _) => c.iter().cloned().collect(),
            Value::Constructor(c, _ty, _reuse, params) => {
                let mut r: HashSet<String> = params
                    .iter()
                    .map(Value::free_vars)
                    .reduce(|l, r| HashSet::union(&l, &r).cloned().collect())
                    .unwrap_or(HashSet::new());
                r.insert(c.clone());
                r
            }
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

impl Match {
    pub fn free_vars(&self) -> HashSet<String> {
        let r = self
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
        r.union(&self.0.free_vars()).cloned().collect()
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
