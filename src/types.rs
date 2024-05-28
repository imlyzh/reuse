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
    // TODO: L2 Owned
    pub params: Vec<Type>,
    pub ret_type: Box<Type>,
}
