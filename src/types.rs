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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owned {
    Linear,
    Borrow,
}
