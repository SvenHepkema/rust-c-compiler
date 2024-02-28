use crate::parser::{BinaryOp, NodeType, ParseNode, UnaryOp};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Param {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Constant(i32),
}

impl Param {
    fn as_string(&self) -> String {
        match self {
            Param::Eax => "eax".to_string(),
            Param::Ebx => "ebx".to_string(),
            Param::Ecx => "ecx".to_string(),
            Param::Edx => "edx".to_string(),
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
            Asm::Neg(param) => {
                format!("neg {}", param.as_string())
            }
            Asm::Add(first, second) => {
                format!("add {}, {}", first.as_string(), second.as_string())
            }
            Asm::Sub(first, second) => {
                format!("sub {}, {}", first.as_string(), second.as_string())
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

fn join_two(mut first: Vec<Asm>, second: &mut Vec<Asm>) -> Vec<Asm> {
    first.append(second);
    first
}

fn join_three(mut first: Vec<Asm>, second: &mut Vec<Asm>, third: &mut Vec<Asm>) -> Vec<Asm> {
    first.append(second);
    first.append(third);
    first
}

fn join_four(
    mut first: Vec<Asm>,
    second: &mut Vec<Asm>,
    third: &mut Vec<Asm>,
    fourth: &mut Vec<Asm>,
) -> Vec<Asm> {
    first.append(second);
    first.append(third);
    first.append(fourth);
    first
}

pub fn generate_operations(node: &ParseNode) -> Vec<Asm> {
    match &node.entry {
        NodeType::Fn(name) => {
            let function_name = match name.as_str() {
                "main" => "_start".to_string(),
                _ => name.clone(),
            };
            join_two(
                vec![Asm::FunctionRef(function_name)],
                &mut generate_operations(node.get_child(0)),
            )
        }
        NodeType::Stmt => join_two(
            generate_operations(node.get_child(0)),
            &mut vec![
                Asm::Mov(Param::Ebx, Param::Eax),
                Asm::Mov(Param::Eax, Param::Constant(1)),
                Asm::OSInterrupt,
            ],
        ),
        NodeType::Const(x) => {
            vec![Asm::Mov(Param::Eax, Param::Constant(x.clone()))]
        }
        NodeType::UnaryOp(t) => match t {
            UnaryOp::Minus => join_two(
                generate_operations(node.get_child(0)),
                &mut vec![Asm::Neg(Param::Eax)],
            ),
        },
        NodeType::BinaryOp(t) => match t {
            BinaryOp::Minus => join_four(
                generate_operations(node.get_child(1)),
                &mut vec![Asm::Mov(Param::Ebx, Param::Eax)],
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Sub(Param::Eax, Param::Ebx)],
            ),
            BinaryOp::Plus => join_four(
                generate_operations(node.get_child(1)),
                &mut vec![Asm::Mov(Param::Ebx, Param::Eax)],
                &mut generate_operations(node.get_child(0)),
                &mut vec![Asm::Add(Param::Eax, Param::Ebx)],
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

