use reuse::ir::common::Pattern;
use reuse::ir::l3_ir::*;
use reuse::types::{Owned, StructType, Type};

#[test]
pub fn test_l3() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    /* */
    let f = Function {
        name: "test".to_string(),
        return_type: input_output_type.clone(),
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::BindPattern(BindPattern {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: "a".to_string(),
            cont: Box::new(Body::Bind(Bind {
                var: "t".to_owned(),
                ty: input_output_type.clone(),
                value: Box::new(Compute::Constructor(
                    "Cons".to_owned(),
                    input_output_type,
                    None,
                    vec!["x".to_string(), "xx".to_string()],
                )),
                cont: Box::new(Body::Move("t".to_owned())),
            })),
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
        body: Body::BindPattern(BindPattern {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: "a".to_string(),
            cont: Box::new(Body::Move("xx".to_owned())),
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
        body: Body::BindPattern(BindPattern {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: "a".to_string(),
            cont: Box::new(Body::If(If {
                cond: "x".to_owned(),
                then: Box::new(Body::Move("x".to_owned())),
                else_: Box::new(Body::Move("xx".to_owned())),
            })),
        }),
    };
    let (r, liveness) = f.insert_drop_reuse();
    println!("liveness: {:?}", liveness);
    println!("return:\n{}", r);
}

#[test]
pub fn test_l3_nested_if() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    let f = Function {
        name: "test".to_string(),
        return_type: Type::Int,
        args: vec![
            ("a".to_string(), (input_output_type.clone(), Owned::Linear)),
            ("b".to_string(), (input_output_type.clone(), Owned::Linear)),
        ],
        body: Body::BindPattern(BindPattern {
            pat: Pattern::Constructor(
                "Cons".to_owned(),
                vec![
                    Pattern::Variable("x".to_string()),
                    Pattern::Variable("xx".to_string()),
                ],
            ),
            owned: Owned::Linear,
            ty: input_output_type.clone(),
            value: "a".to_string(),
            cont: Box::new(Body::BindPattern(BindPattern {
                pat: Pattern::Constructor(
                    "Cons".to_owned(),
                    vec![
                        Pattern::Variable("y".to_string()),
                        Pattern::Variable("yy".to_string()),
                    ],
                ),
                owned: Owned::Linear,
                ty: input_output_type.clone(),
                value: "b".to_string(),
                cont: Box::new(Body::If(If {
                    cond: "x".to_owned(),
                    then: Box::new(Body::If(If {
                        cond: "y".to_owned(),
                        then: Box::new(Body::Move("x".to_owned())),
                        else_: Box::new(Body::Move("y".to_owned())),
                    })),
                    else_: Box::new(Body::If(If {
                        cond: "y".to_owned(),
                        then: Box::new(Body::Move("xx".to_owned())),
                        else_: Box::new(Body::Move("yy".to_owned())),
                    })),
                })),
            })),
        }),
    };
    let (r, liveness) = f.insert_drop_reuse();
    println!("liveness: {:?}", liveness);
    println!("return:\n{}", r);
}
