use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use crate::types::*;

pub type Name = String;

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Variable(Name),
    Constructor(Name, Vec<Pattern>),
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

    /// binding type to pattern(equality type deconstruct)
    pub fn type_binding(&self, ty: &Type) -> HashMap<Name, Type> {
        match (self, ty) {
            (Pattern::Wildcard, _) => HashMap::new(),
            (Pattern::Variable(v), ty) => vec![(v.clone(), ty.clone())].into_iter().collect(),
            (Pattern::Constructor(c, params), Type::Struct(StructType { name, fields }))
                if c == name =>
            {
                params
                    .iter()
                    .zip(fields)
                    .map(|(pat, ty)| pat.type_binding(ty))
                    .reduce(|mut l, r| {
                        l.extend(r);
                        l
                    })
                    .map_or_else(HashMap::new, identity)
            }
            _ => panic!("type binding to pattern, not matched: {:?}, {:?}", self, ty),
        }
    }
}
