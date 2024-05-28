use std::rc::Rc;

#[derive(Debug)]
pub struct Scope<T>(pub String, pub T, pub NullableScope<T>);

pub type NullableScope<T> = Option<Rc<Scope<T>>>;

pub fn find_variable<T: Clone>(record: &NullableScope<T>, name: &str) -> Option<T> {
    record
        .clone()
        .and_then(|re| re.find_variable(name).cloned())
}

impl<T> Scope<T> {
    pub fn get_parent(&self) -> Option<&Rc<Scope<T>>> {
        self.2.as_ref()
    }

    pub fn find_variable(&self, name: &str) -> Option<&T> {
        if self.0 == name {
            return Some(&self.1);
        } else if let Some(parent) = self.get_parent() {
            return parent.find_variable(name);
        }
        None
    }
}
