use std::collections::{HashMap, HashSet};

use crate::ir::{common::Name, l2_ir::*};

impl Body {
    fn shadow_variable_rename(self, namespace: HashMap<Name, Name>, rename_counter: usize) -> Self {
        match self {
            Body::Variable(v) => namespace
                .get(&v)
                .map_or_else(|| Body::Variable(v), |v| Body::Variable(v.clone())),
            Body::Bind(e) => Body::Bind(e.shadow_variable_rename(namespace, rename_counter)),
            Body::BindPattern(e) => {
                Body::BindPattern(e.shadow_variable_rename(namespace, rename_counter))
            }
            Body::If(e) => Body::If(e.shadow_variable_rename(namespace, rename_counter)),
            Body::Match(e) => Body::Match(e.shadow_variable_rename(namespace, rename_counter)),
        }
    }
}

impl Bind {
    fn shadow_variable_rename(self, namespace: HashMap<Name, Name>, rename_counter: usize) -> Self {
        todo!()
    }
}

impl BindPattern {
    fn shadow_variable_rename(self, namespace: HashMap<Name, Name>, rename_counter: usize) -> Self {
        todo!()
    }
}

impl If {
    fn shadow_variable_rename(self, namespace: HashMap<Name, Name>, rename_counter: usize) -> Self {
        todo!()
    }
}

impl Match {
    fn shadow_variable_rename(self, namespace: HashMap<Name, Name>, rename_counter: usize) -> Self {
        todo!()
    }
}
