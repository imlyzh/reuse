use std::collections::HashSet;

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
}
