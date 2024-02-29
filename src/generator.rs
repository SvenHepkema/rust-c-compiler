use crate::parser::{BinaryOp, NodeType, ParseNode, UnaryOp};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Param {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Constant(i32),
}

impl Param {
    fn as_string(&self) -> String {
        match self {
            Param::Rax => "rax".to_string(),
            Param::Rbx => "rbx".to_string(),
            Param::Rcx => "rcx".to_string(),
            Param::Rdx => "rdx".to_string(),
            Param::Constant(x) => x.to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Asm {
    FunctionRef(String),
    OSInterrupt, // Linux: always 0x80
    Mov(Param, Param),
    Neg(Param),
    Add(Param, Param),
    Sub(Param, Param),
    Mul(Param, Param),
    Push(Param),
    Pop(Param),
}

impl Asm {
    fn as_string(&self) -> String {
        match self {
            Asm::FunctionRef(name) => {
                let mut line = name.clone();
                line.push(':');
                line.to_owned()
            }
            Asm::OSInterrupt => "int 0x80".to_string(),
            Asm::Mov(first, second) => {
                format!("mov {}, {}", first.as_string(), second.as_string())
            }
            Asm::Push(param) => {
                format!("push {}", param.as_string())
            }
            Asm::Pop(param) => {
                format!("pop {}", param.as_string())
            }
            Asm::Neg(param) => {
                format!("neg {}", param.as_string())
            }
            Asm::Add(first, second) => {
                format!("add {}, {}", first.as_string(), second.as_string())
            }
            Asm::Sub(first, second) => {
                format!("sub {}, {}", first.as_string(), second.as_string())
            }
            Asm::Mul(first, second) => {
                format!("imul {}, {}", first.as_string(), second.as_string())
            }
        }
    }
}

pub fn print_operations(operations: &Vec<Asm>) {
    println!("\nGenerated Operations:");
    for operation in operations {
        println!("{:?}", operation)
    }
}

macro_rules! join_asm  {
    ( $( $x:expr ),* ) => {
        {
            let mut result: Vec<Asm> = Vec::new();
            $ (
                result.append($x);
            )*
            result
        }
    };
}

pub fn generate_operations(node: &ParseNode) -> Vec<Asm> {
    match &node.entry {
        NodeType::Fn(name) => {
            let function_name = match name.as_str() {
                "main" => "_start".to_string(),
                _ => name.clone(),
            };
            join_asm!(
                &mut vec![Asm::FunctionRef(function_name)],
                &mut generate_operations(node.get_child(0))
            )
        }
        NodeType::Stmt => join_asm!(
            &mut generate_operations(node.get_child(0)),
            &mut vec![
                Asm::Mov(Param::Rbx, Param::Rax),
                Asm::Mov(Param::Rax, Param::Constant(1)),
                Asm::OSInterrupt
            ]
        ),
        NodeType::Const(x) => {
            vec![Asm::Mov(Param::Rax, Param::Constant(*x))]
        }
        NodeType::UnaryOp(t) => match t {
            UnaryOp::Minus => join_asm!(
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Neg(Param::Rax)]
            ),
        },
        NodeType::BinaryOp(t) => match t {
            BinaryOp::Minus => join_asm!(
                &mut generate_operations(node.get_child(1)),
                &mut vec![Asm::Push(Param::Rax)],
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Pop(Param::Rbx), Asm::Sub(Param::Rax, Param::Rbx)]
            ),
            BinaryOp::Plus => join_asm!(
                &mut generate_operations(node.get_child(1)),
                &mut vec![Asm::Push(Param::Rax)],
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Pop(Param::Rbx), Asm::Add(Param::Rax, Param::Rbx)]
            ),
            BinaryOp::Multiplication => join_asm!(
                &mut generate_operations(node.get_child(1)),
                &mut vec![Asm::Push(Param::Rax)],
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Pop(Param::Rbx), Asm::Mul(Param::Rax, Param::Rbx)]
            ),
        },
        _ => {
            vec![]
        }
    }
}

pub fn generate_assembly(operations: &Vec<Asm>) -> String {
    let mut strings: Vec<String> = vec![
        " section     .text".to_string(),
        "global      _start".to_string(),
    ];

    for op in operations {
        strings.push(op.as_string());
    }

    strings.join("\n")
}
