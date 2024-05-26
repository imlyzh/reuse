use std::collections::HashMap;

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
    pub fields: HashMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret_type: Box<Type>,
}
