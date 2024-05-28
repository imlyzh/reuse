use std::fmt;

use crate::{
    ir::l3_ir::*,
    types::{FunctionType, StructType, Type},
};

use super::{Compute, Function, Owned};

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .args
            .iter()
            .map(|(name, (ty, owned))| format!("{}{}: {}", owned, name, ty))
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "fn {}({}) -> {} {{\n{}\n}}\n",
            self.name, args, self.return_type, self.body
        )
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Compute(c) => c.fmt(f),
            Body::Bind(b) => b.fmt(f),
            Body::If(i) => i.fmt(f),
            Body::Match(m) => m.fmt(f),
            Body::Dup(dst, src, cont) => write!(f, "let {} = dup {};\n{}", dst, src, cont),
            Body::Drop(src, cont) => write!(f, "drop {};\n{}", src, cont),
            Body::DropReuse(reuse, src, cont) => {
                write!(f, "let {} = drop-reuse {};\n{}", reuse, src, cont)
            }
            Body::DupOnBind(src, cont) => write!(f, "dup {};\n{}", src, cont),
        }
    }
}

impl fmt::Display for Compute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Compute::Variable(v) => write!(f, "{}", v),
            Compute::Invoke(fun, args) => {
                let args = args.to_vec().join(", ");
                write!(f, "{}({})", fun, args)
            }
            // FIXME: fun_type, free_vars
            Compute::Closure {
                fun_type,
                free_vars: _,
                params,
                body,
            } => {
                let args = params
                    .iter()
                    .map(|(name, owned)| format!("{} {}", owned, name))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "fn({}) -> {} {{\n{}}}\n", args, fun_type, body)
            }
            Compute::Constructor(cname, _ty, reuse, args) => {
                let args = args.to_vec().join(", ");
                if let Some(reuse) = reuse {
                    write!(f, "{}@{}({})", cname, reuse, args)
                } else {
                    write!(f, "{}({})", cname, args)
                }
            }
        }
    }
}

impl fmt::Display for Bind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Owned::Linear = self.owned {
            write!(f, "let {} = {};\n{}", self.pat, self.value, self.cont)
        } else {
            write!(f, "let^ {} = {};\n{}", self.pat, self.value, self.cont)
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {} then {{\n{}\n}} else {{\n{}\n}}\n",
            self.cond, self.then, self.else_
        )
        // write!(f, "if {} then\n{}\nelse\n{}\nend\n", self.cond, self.then, self.else_)
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let matchs = self
            .matchs
            .iter()
            .map(|(pat, body)| format!("{} => {{\n{}\n}}", pat, body))
            .collect::<Vec<_>>()
            .join(",\n");
        if let Owned::Linear = self.owned {
            write!(f, "match {} {{\n{}}}\n", self.value, matchs)
        } else {
            write!(f, "match^ {} {{\n{}}}\n", self.value, matchs)
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Variable(v) => write!(f, "{}", v),
            Pattern::Constructor(cname, args) => {
                let args = args
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", cname, args)
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Struct(s) => s.fmt(f),
            Type::Function(fun) => fun.fmt(f),
        }
    }
}

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = self
            .fields
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}({})", self.name, fields)
    }
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self
            .params
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "({}) -> {}", params, self.ret_type)
    }
}

impl fmt::Display for Owned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Owned::Linear => write!(f, ""),
            Owned::Borrow => write!(f, "^"),
        }
    }
}
