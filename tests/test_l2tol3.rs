use reuse::ir::common::Pattern;
use reuse::ir::l2_ir::*;
use reuse::pass::l2::use_check::Used;
use reuse::types::{Owned, StructType, Type};

#[test]
pub fn test_l2tol3_move() {
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
            ty: input_output_type.clone(),
            value: "a".to_string(),
            cont: Box::new(Body::Bind(Bind {
                var: "t".to_owned(),
                ty: input_output_type.clone(),
                value: Box::new(Compute::Constructor(
                    "Cons".to_owned(),
                    input_output_type,
                    vec!["x".to_string(), "xx".to_string()],
                )),
                cont: Box::new(Body::Variable("t".to_owned())),
            })),
        }),
    };
    let mut used_record = Used::new();
    f.use_check(&mut used_record).unwrap();
    let linears = used_record.to_linears();
    let gen_f = f.linearize(&linears);
    println!("gen result:\n{}", gen_f);
}

#[test]
pub fn test_l2tol3_share() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    /* */
    let f = Function {
        name: "test".to_string(),
        return_type: input_output_type.clone(),
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::Bind(Bind {
            var: "_".to_owned(),
            ty: Type::Bool,
            value: Box::new(Compute::Invoke(
                "take".to_owned(),
                vec![("a".to_string(), Owned::Linear)],
            )),
            cont: Box::new(Body::Bind(Bind {
                var: "_".to_owned(),
                ty: Type::Bool,
                value: Box::new(Compute::Invoke(
                    "identity".to_owned(),
                    vec![("a".to_string(), Owned::Borrow)],
                )),
                cont: Box::new(Body::Bind(Bind {
                    var: "r".to_owned(),
                    ty: Type::Bool,
                    value: Box::new(Compute::Invoke(
                        "take".to_owned(),
                        vec![("a".to_string(), Owned::Linear)],
                    )),
                    cont: Box::new(Body::Variable("r".to_owned())),
                })),
            })),
        }),
    };
    let mut used_record = Used::new();
    f.use_check(&mut used_record).unwrap();
    let linears = used_record.to_linears();
    let gen_f = f.linearize(&linears);
    println!("gen result:\n{}", gen_f);
}

#[test]
pub fn test_l2tol3_if() {
    let input_output_type = Type::Struct(StructType {
        name: "Cons".to_owned(),
        fields: vec![Type::Int, Type::Int],
    });
    /* */
    let f = Function {
        name: "test".to_string(),
        return_type: input_output_type.clone(),
        args: vec![("a".to_string(), (input_output_type.clone(), Owned::Linear))],
        body: Body::Bind(Bind {
            var: "_".to_owned(),
            ty: Type::Bool,
            value: Box::new(Compute::Invoke(
                "take".to_owned(),
                vec![("a".to_string(), Owned::Linear)],
            )),
            cont: Box::new(Body::If(If {
                cond: "a".to_owned(),
                then: Box::new(Body::Bind(Bind {
                    var: "r".to_owned(),
                    ty: Type::Bool,
                    value: Box::new(Compute::Invoke(
                        "take".to_owned(),
                        vec![("a".to_string(), Owned::Linear)],
                    )),
                    cont: Box::new(Body::Bind(Bind {
                        var: "_".to_owned(),
                        ty: Type::Bool,
                        value: Box::new(Compute::Invoke(
                            "identity".to_owned(),
                            vec![("a".to_string(), Owned::Linear)],
                        )),
                        cont: Box::new(Body::Variable("r".to_owned())),
                    })),
                })),
                else_: Box::new(Body::Variable("a".to_owned())),
            })),
        }),
    };
    let mut used_record = Used::new();
    f.use_check(&mut used_record).unwrap();
    let linears = used_record.to_linears();
    let gen_f = f.linearize(&linears);
    println!("gen result:\n{}", gen_f);
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
                ty: input_output_type.clone(),
                value: "b".to_string(),
                cont: Box::new(Body::If(If {
                    cond: "x".to_owned(),
                    then: Box::new(Body::If(If {
                        cond: "y".to_owned(),
                        then: Box::new(Body::Variable("x".to_owned())),
                        else_: Box::new(Body::Variable("y".to_owned())),
                    })),
                    else_: Box::new(Body::If(If {
                        cond: "y".to_owned(),
                        then: Box::new(Body::Variable("xx".to_owned())),
                        else_: Box::new(Body::Variable("yy".to_owned())),
                    })),
                })),
            })),
        }),
    };

    let mut used_record = Used::new();
    f.use_check(&mut used_record).unwrap();
    let linears = used_record.to_linears();
    let gen_f = f.linearize(&linears);
    println!("gen result:\n{}", gen_f);
}
