use std::collections::HashMap;
// use std::rc::Rc;

use crate::{
    l2_ir::{Bind, Body, Compute, If, Match, Name, Pattern},
    types::Type,
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
            Body::Dup(name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::Dup(name.clone(), Box::new(e))),
            Body::Drop(name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::Drop(name.clone(), Box::new(e))),
            Body::DropReuse(new_name, name, e) => e
                .rewrite_construct(name, ty)
                .map(|e| Body::DropReuse(new_name.clone(), name.clone(), Box::new(e))),
            Body::If(i) => i.rewrite_construct(name, ty).map(|i| Body::If(i)),
            Body::Match(m) => m.rewrite_construct(name, ty).map(|m| Body::Match(m)),
        }
    }
}

impl Bind {
    /// Trivial
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        if let Some(t2) = self.2.rewrite_construct(name, ty) {
            return Some(Bind(
                self.0.clone(),
                self.1.clone(),
                Box::new(t2),
                self.3.clone(),
            ));
        }
        if let Some(t3) = self.3.rewrite_construct(name, ty) {
            return Some(Bind(
                self.0.clone(),
                self.1.clone(),
                self.2.clone(),
                Box::new(t3),
            ));
        }
        None
    }
}

impl If {
    /// Notice
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        let it1 = self.1.rewrite_construct(name, ty);
        let it2 = self.2.rewrite_construct(name, ty);
        let r = match (it1, it2) {
            (Some(it1), None) => If(
                self.0.clone(),
                Box::new(it1),
                Box::new(Body::Drop(name.clone(), self.2.clone())),
            ),
            (None, Some(it2)) => If(
                self.0.clone(),
                Box::new(Body::Drop(name.clone(), self.1.clone())),
                Box::new(it2),
            ),
            (Some(it1), Some(it2)) => If(self.0.clone(), Box::new(it1), Box::new(it2)),
            (None, None) => return None,
        };
        Some(r)
    }
}

impl Match {
    /// Notice
    pub fn rewrite_construct(&self, name: &Name, ty: &Type) -> Option<Self> {
        let matchs: Vec<_> = self
            .1
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
        Some(Match(self.0.clone(), matchs))
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
        todo!()
    }
}
