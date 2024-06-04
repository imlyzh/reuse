use std::collections::{HashMap, HashSet};

use crate::{
    ir::l2_ir::{Bind, BindPattern, Body, Compute, Function, If, Match},
    types::Owned,
};

#[derive(Debug, Default)]
pub struct Used(HashMap<String, Option<Owned>>);

impl Used {
    pub fn new() -> Self {
        Used::default()
    }
    pub fn update(&mut self, name: &str, owned: Option<Owned>) {
        // If variable is borrow or empty, it will be overwritten.
        if let Some(Some(record)) = self.0.get(name) {
            if let Some(owned) = owned {
                self.0.insert(name.to_string(), Some(*record.max(&owned)));
            }
            return;
        }
        self.0.insert(name.to_string(), owned);
    }
    pub fn update_define(&mut self, name: &str, owned: Owned) -> Result<(), String> {
        // owned state update order
        if owned.is_borrow() {
            if let Some(Some(Owned::Linear)) = self.0.get(name) {
                return Err(name.to_string());
            }
        }
        self.0.insert(name.to_string(), Some(owned));
        Ok(())
    }
    pub fn find(&self, k: &str) -> Option<Option<Owned>> {
        self.0.get(k).cloned()
    }

    pub fn to_linears(self) -> HashSet<String> {
        self.0
            .into_iter()
            .filter_map(|(name, owned)| if owned?.is_linear() { Some(name) } else { None })
            .collect()
    }
}

impl Function {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        self.body.use_check(used_record)?;
        for (name, (_ty, owned)) in self.args.iter() {
            used_record.update_define(name, *owned)?;
        }
        Ok(())
    }
}

impl Body {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        match self {
            Body::Bind(b) => b.use_check(used_record),
            Body::BindPattern(b) => b.use_check(used_record),
            Body::If(i) => i.use_check(used_record),
            Body::Match(m) => m.use_check(used_record),
            Body::Variable(v) => {
                used_record.update(v, Some(Owned::Linear));
                Ok(())
            }
        }
    }
}

impl Compute {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        match self {
            // Compute::Variable(v) => {},
            Compute::Invoke(f, args) => {
                used_record.update(f, None);
                for (name, owned) in args {
                    used_record.update(name, Some(*owned));
                }
                Ok(())
            }
            Compute::Closure { free_vars, .. } => {
                for name in free_vars {
                    used_record.update(name, Some(Owned::Linear));
                }
                Ok(())
            }
            // Compute::Constructor(c, _ty, _reuse, params) => {
            Compute::Constructor(c, _ty, params) => {
                used_record.update(c, None);
                for name in params {
                    used_record.update(name, Some(Owned::Linear));
                }
                Ok(())
            }
        }
    }
}

impl Bind {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        self.cont.use_check(used_record)?;
        used_record.update_define(&self.var, Owned::Linear)?;
        self.value.use_check(used_record)
    }
}

impl BindPattern {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        self.cont.use_check(used_record)?;
        let is_linear = self
            .pat
            .defined_vars()
            .into_iter()
            .filter_map(|name| used_record.find(&name))
            .any(|x| matches!(x, Some(Owned::Linear)));
        if is_linear {
            used_record.update(&self.value, Some(Owned::Linear));
        } else {
            used_record.update(&self.value, Some(Owned::Borrow));
        }
        Ok(())
    }
}

impl If {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        let If { cond, then, else_ } = self;
        used_record.update(cond, Some(Owned::Borrow));
        // Notice: variable name is unique
        then.use_check(used_record)?;
        else_.use_check(used_record)?;
        Ok(())
    }
}

impl Match {
    pub fn use_check(&self, used_record: &mut Used) -> Result<(), String> {
        let mut is_linear = false;
        for (pat, expr) in self.matchs.iter() {
            // Notice: variable name is unique
            expr.use_check(used_record)?;
            let expr_is_linear = pat
                .defined_vars()
                .into_iter()
                .filter_map(|name| used_record.find(&name))
                .any(|x| matches!(x, Some(Owned::Linear)));
            is_linear = is_linear || expr_is_linear;
        }
        if is_linear {
            used_record.update(&self.value, Some(Owned::Linear));
        } else {
            used_record.update(&self.value, Some(Owned::Borrow));
        }
        Ok(())
    }
}
