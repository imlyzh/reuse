use std::{collections::HashMap, convert::identity};
// use std::rc::Rc;

use crate::{
    ir::l3_ir::{Bind, Body, Compute, If, Match, Name, Pattern},
    types::{StructType, Type},
    // l2_ir::{If, Match},
    // utils::Scope,
};

// /// insert drop-reuse

// impl Body {
//     /// Trivial
//     pub fn insert_drop_reuse(self, scope: Rc<Scope<Type>>) -> Self {
//         match self {
//             Body::Bind(b) => Body::Bind(b.insert_drop_reuse(scope)),
//             Body::If(i) => Body::If(i.insert_drop_reuse(scope)),
//             Body::Match(m) => Body::Match(m.insert_drop_reuse(scope)),
//             Body::Compute(c) => Body::Compute(c.insert_drop_reuse(scope)),
//             Body::Dup(name, e) => Body::Dup(name, Box::new(e.insert_drop_reuse(scope))),
//             Body::Drop(name, e) => Body::Drop(name, Box::new(e.insert_drop_reuse(scope))),
//             Body::DropReuse(new_name, name, e) => {
//                 Body::DropReuse(new_name, name, Box::new(e.insert_drop_reuse(scope)))
//             }
//         }
//     }
// }

// impl Compute {
//     /// Notice
//     pub fn insert_drop_reuse(self, scope: Rc<Scope<Type>>) -> Self {
//         match self {
//             Compute::Closure {
//                 free_vars,
//                 params,
//                 body,
//             } => {
//                 // TODO
//                 Compute::Closure {
//                     free_vars,
//                     params,
//                     body: Box::new(body.insert_drop_reuse(scope)),
//                 }
//             }
//             Compute::Variable(_) => self,
//             Compute::Invoke(_, _) => self,
//             Compute::Constructor(_, _, _, _) => self,
//         }
//     }
// }

// impl Bind {
//     /// Trivial
//     pub fn insert_drop_reuse(self, scope: Rc<Scope<Type>>) -> Self {
//         let scope = self
//             .0
//             .type_binding(&self.1)
//             .into_iter()
//             .fold(scope, |scope, (name, ty)| {
//                 Rc::new(Scope(name, ty, Some(scope)))
//             });
//         Bind(
//             self.0,
//             self.1,
//             Box::new(self.2.insert_drop_reuse(scope.clone())),
//             Box::new(self.3.insert_drop_reuse(scope)),
//         )
//     }
// }

// impl If {
//     /// Trivial
//     pub fn insert_drop_reuse(self, scope: Rc<Scope<Type>>) -> Self {
//         If(
//             self.0.clone(),
//             Box::new(self.1.insert_drop_reuse(scope.clone())),
//             Box::new(self.2.insert_drop_reuse(scope)),
//         )
//     }
// }

// impl Match {
//     /// Notice
//     pub fn insert_drop_reuse(mut self, scope: Rc<Scope<Type>>) -> Self {
//         //////////////// sequence problem
//         let ty = scope.find_variable(self.0.as_str()).unwrap();
//         let borrowed_self = &mut self;
//         for (_, body) in borrowed_self.1.iter_mut() {
//             if let Some(new_body) = body.rewrite_construct(&borrowed_self.0, ty) {
//                 *body = Body::DropReuse(
//                     format!("__reuse_{}", borrowed_self.0),
//                     borrowed_self.0.clone(),
//                     Box::new(new_body),
//                 );
//                 break;
//             }
//         }
//         ////////////////
//         Match(
//             self.0.clone(),
//             self.1
//                 .into_iter()
//                 .map(|(pat, body)| {
//                     let scope = pat
//                         .type_binding(ty)
//                         .into_iter()
//                         .fold(scope.clone(), |scope, (name, ty)| {
//                             Rc::new(Scope(name, ty, Some(scope)))
//                         });
//                     (pat, body.insert_drop_reuse(scope.clone()))
//                 })
//                 .collect(),
//         )
//     }
// }

/// rewrite_construct

impl Body {
    /// Trivial
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        match self {
            Body::Bind(b) => b.rewrite_construct(name, ty).map(Body::Bind),
            Body::Compute(c) => c.rewrite_construct(name, ty).map(Body::Compute),
            Body::Dup(dst_name, src_name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::Dup(dst_name.clone(), src_name.clone(), Box::new(e))),
            Body::Drop(src_name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::Drop(src_name.clone(), Box::new(e))),
            Body::DropReuse(new_name, src_name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::DropReuse(new_name.clone(), src_name.clone(), Box::new(e))),
            Body::If(i) => i.rewrite_construct(name, ty).map(Body::If),
            Body::Match(m) => m.rewrite_construct(name, ty).map(Body::Match),
            Body::DupOnBind(src_name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::DupOnBind(src_name.clone(), Box::new(e))),
        }
    }
}

impl Bind {
    /// Trivial
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        if let Some(cont) = self.cont.rewrite_construct(name, ty) {
            return Some(Bind {
                pat: self.pat.clone(),
                owned: self.owned,
                ty: self.ty.clone(),
                value: self.value.clone(),
                cont: Box::new(cont),
            });
        }
        if let Some(value) = self.value.rewrite_construct(name, ty) {
            return Some(Bind {
                pat: self.pat.clone(),
                owned: self.owned,
                ty: self.ty.clone(),
                value: Box::new(value),
                cont: self.cont.clone(),
            });
        }
        None
    }
}

impl If {
    /// Notice
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        let it1 = self.then.rewrite_construct(name, ty);
        let it2 = self.else_.rewrite_construct(name, ty);
        let r = match (it1, it2) {
            (Some(it1), None) => If {
                cond: self.cond.clone(),
                then: Box::new(it1),
                else_: Box::new(Body::Drop(name.clone(), self.else_.clone())),
            },
            (None, Some(it2)) => If {
                cond: self.cond.clone(),
                then: Box::new(Body::Drop(name.clone(), self.then.clone())),
                else_: Box::new(it2),
            },
            (Some(it1), Some(it2)) => If {
                cond: self.cond.clone(),
                then: Box::new(it1),
                else_: Box::new(it2),
            },
            (None, None) => return None,
        };
        Some(r)
    }
}

impl Match {
    /// Notice
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        let matchs: Vec<_> = self
            .matchs
            .iter()
            .map(|(pat, body)| {
                (
                    pat.clone(),
                    body.rewrite_construct(name, ty).ok_or(body.clone()),
                )
            })
            .collect();
        // return cond: not exist branch has been rewrited
        let exist_rewrited = matchs.iter().any(|(_, body)| body.is_ok());
        if !exist_rewrited {
            return None;
        }
        let matchs = matchs
            .into_iter()
            .map(|(pat, body)| match body {
                Ok(body) => (pat, body),
                Err(body) => (pat, Body::Drop(name.clone(), Box::new(body))),
            })
            .collect();
        Some(Match {
            value: self.value.clone(),
            owned: self.owned,
            matchs,
        })
    }
}

impl Compute {
    /// Notice
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        match self {
            Compute::Constructor(cname, cty, None, params) => {
                // reuse condition
                if ty != cty {
                    return None;
                }
                Some(Compute::Constructor(
                    cname.to_string(),
                    cty.clone(),
                    Some(format!("__reuse_{}", name)),
                    params.clone(),
                ))
            }
            // Compute::Closure { free_vars, params, body } => todo!(),
            _ => None,
        }
    }
}

/// binding type to pattern(equality type deconstruct)

impl Pattern {
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
