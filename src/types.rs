#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Bool,
    Int,
    Float,
    Struct(StructType),
    Function(FunctionType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<(Type, Owned)>,
    pub ret_type: Box<Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Owned {
    Linear = 1,
    Borrow = 0,
}

impl Owned {
    pub fn is_linear(&self) -> bool {
        match self {
            Owned::Linear => true,
            Owned::Borrow => false,
        }
    }
    pub fn is_borrow(&self) -> bool {
        match self {
            Owned::Borrow => true,
            Owned::Linear => false,
        }
    }
}
