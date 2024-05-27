#[derive(Debug)]
pub enum Expr {
    Variable(String),
    Lit(i32),
    Invoke(Box<Expr>, Vec<Expr>),
    Lambda(Vec<String>, Block),
    // StaticInvoke(String, Vec<Expr>),
    Constructor(String, Option<Box<Expr>>, Vec<Expr>),
    If(Box<Expr>, Block, Option<Block>),
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Dup(Expr),
    Drop(Expr),
    Assign(String, Expr),
    // While(Expr, Body),
    // Return(Option<Expr>),
}

#[derive(Debug)]
pub struct Block(pub Vec<Stmt>);

pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Block,
}

// pub struct Module {
//   pub functions: Vec<Function>,
// }

pub fn merge_to_blocks(a: Expr, b: Expr) -> Block {
    Block(vec![Stmt::Expr(a), Stmt::Expr(b)])
}