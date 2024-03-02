use crate::parser::{BinaryOp, NodeType, ParseNode, UnaryOp};
use crate::constants::QUADWORD_LENGTH;


#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Param {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsp,
    Rbp,
    Constant(i32),
    BpMinus(usize),
}

impl Param {
    fn as_string(&self) -> String {
        match self {
            Param::Rax => "rax".to_string(),
            Param::Rbx => "rbx".to_string(),
            Param::Rcx => "rcx".to_string(),
            Param::Rdx => "rdx".to_string(),
            Param::Rsp => "rsp".to_string(),
            Param::Rbp => "rbp".to_string(),
            Param::Constant(x) => x.to_string(),
            Param::BpMinus(x) => format!("[rbp - {}]", x),
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

trait AddSelfToAsmVec {
    fn add_self_to(&self, assembly: &mut Vec<Asm>);
}

impl AddSelfToAsmVec for Asm {
    fn add_self_to(&self, assembly: &mut Vec<Asm>) {
        assembly.push(self.clone());
    }
}

impl AddSelfToAsmVec for Vec<Asm> {
    fn add_self_to(&self, assembly: &mut Vec<Asm>) {
        for asm in self {
            asm.add_self_to(assembly);
        }
    }
}

impl AddSelfToAsmVec for &[Asm] {
    fn add_self_to(&self, assembly: &mut Vec<Asm>) {
        for asm in *self {
            asm.add_self_to(assembly);
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
                $x.add_self_to(&mut result);
            )*
            result
        }
    };
}

pub fn generate_operations(node: &ParseNode) -> Vec<Asm> {
    match &node.node_type {
        NodeType::Fn(name, n_variables) => {
            let function_name = match name.as_str() {
                "main" => "_start".to_string(),
                _ => name.clone(),
            };
            let mut statements = vec![];

            for child in node.children.iter() {
                statements.append(&mut generate_operations(child));
            }

            if matches!(node.children.last().unwrap().node_type, NodeType::Return) {
                let split_statements = statements.split_last().unwrap();

                join_asm!(
                    Asm::FunctionRef(function_name.clone()),
                    Asm::Push(Param::Rbp),
                    Asm::Mov(Param::Rbp, Param::Rsp),
                    Asm::Sub(Param::Rsp, Param::Constant((n_variables * QUADWORD_LENGTH) as i32)),
                    split_statements.1,
                    Asm::Add(Param::Rsp, Param::Constant((n_variables * QUADWORD_LENGTH) as i32)),
                    Asm::Pop(Param::Rbp),
                    split_statements.0
                )
            } else {
                join_asm!(
                    Asm::FunctionRef(function_name.clone()),
                    Asm::Push(Param::Rbp),
                    Asm::Mov(Param::Rbp, Param::Rsp),
                    Asm::Sub(Param::Rsp, Param::Constant((n_variables * QUADWORD_LENGTH) as i32)),
                    statements,
                    Asm::Add(Param::Rsp, Param::Constant((n_variables * QUADWORD_LENGTH) as i32)),
                    Asm::Pop(Param::Rbp)
                )
            }
        }
        NodeType::Return => join_asm!(
            generate_operations(node.get_child(0)),
            Asm::Mov(Param::Rbx, Param::Rax),
            Asm::Mov(Param::Rax, Param::Constant(1)),
            Asm::OSInterrupt
        ),
        NodeType::VarDecl(_, offset) => {
            join_asm!(
                generate_operations(node.get_child(0)),
                Asm::Mov(Param::BpMinus(*offset as usize), Param::Rax)
            )
        }
        NodeType::Var(_, offset) => {
            vec![Asm::Mov(
                Param::Rax,
                Param::BpMinus(*offset as usize),
            )]
        }
        NodeType::Const(x) => {
            vec![Asm::Mov(Param::Rax, Param::Constant(*x))]
        }
        NodeType::UnaryOp(t) => match t {
            UnaryOp::Minus => {
                join_asm!(generate_operations(node.get_child(0)), Asm::Neg(Param::Rax))
            }
            UnaryOp::Function(_) => {
                // TODO: Implement
                panic!("Function generation not implemented yet.")
            }
        },
        NodeType::BinaryOp(t) => match t {
            BinaryOp::Minus => join_asm!(
                generate_operations(node.get_child(1)),
                Asm::Push(Param::Rax),
                generate_operations(node.get_child(0)),
                Asm::Pop(Param::Rbx),
                Asm::Sub(Param::Rax, Param::Rbx)
            ),
            BinaryOp::Plus => join_asm!(
                generate_operations(node.get_child(1)),
                Asm::Push(Param::Rax),
                generate_operations(node.get_child(0)),
                Asm::Pop(Param::Rbx),
                Asm::Add(Param::Rax, Param::Rbx)
            ),
            BinaryOp::Multiplication => join_asm!(
                generate_operations(node.get_child(1)),
                Asm::Push(Param::Rax),
                generate_operations(node.get_child(0)),
                Asm::Pop(Param::Rbx),
                Asm::Mul(Param::Rax, Param::Rbx)
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
