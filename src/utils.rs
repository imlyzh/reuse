use std::rc::Rc;

#[derive(Debug)]
pub struct Scope<T>(pub String, pub T, pub Option<Rc<Scope<T>>>);

impl<T> Scope<T> {
    pub fn get_parent(&self) -> Option<&Rc<Scope<T>>> {
        self.2.as_ref()
    }

    pub fn find_variable(&self, name: &str) -> Option<&T> {
        if self.0 == name {
            return Some(&self.1);
        } else if let Some(parent) = self.2.as_ref() {
            return parent.find_variable(name);
        }
        None
    }
}
