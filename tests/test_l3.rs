use reuse::ir::l3_ir::*;
use reuse::types::{StructType, Type};

#[test]
pub fn test_l3() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    let f = Function {
        name: "test".to_string(),
        return_type: input_output_type.clone(),
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::Bind(Bind {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: Box::new(Compute::Variable("a".to_string())),
            cont: Box::new(Body::Compute(Compute::Constructor(
                "Cons".to_owned(),
                input_output_type,
                None,
                vec!["x".to_string(), "xx".to_string()],
            ))),
        }),
    };
    let (r, liveness) = f.insert_drop_reuse();
    println!("liveness: {:?}", liveness);
    println!("return:\n{}", r);
}

#[test]
pub fn test_l3_1() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    let f = Function {
        name: "test".to_string(),
        return_type: Type::Int,
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::Bind(Bind {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: Box::new(Compute::Variable("a".to_string())),
            cont: Box::new(Body::Compute(Compute::Variable("xx".to_owned()))),
        }),
    };
    let (r, liveness) = f.insert_drop_reuse();
    println!("liveness: {:?}", liveness);
    println!("return:\n{}", r);
}

#[test]
pub fn test_l3_if() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    let f = Function {
        name: "test".to_string(),
        return_type: Type::Int,
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::Bind(Bind {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: Box::new(Compute::Variable("a".to_string())),
            cont: Box::new(Body::If(If {
                cond: "x".to_owned(),
                then: Box::new(Body::Compute(Compute::Variable("x".to_owned()))),
                else_: Box::new(Body::Compute(Compute::Variable("xx".to_owned()))),
            })),
        }),
    };
    let (r, liveness) = f.insert_drop_reuse();
    println!("liveness: {:?}", liveness);
    println!("return:\n{}", r);
}
