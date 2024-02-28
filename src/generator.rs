use crate::parser::{NodeType, ParseNode, UnaryOp, BinaryOp};

pub fn generate(tree: &ParseNode) -> String {
    match &tree.entry {
        NodeType::Prog(name) => name.to_string(),
        NodeType::Fn(name) => {
            let function_name = match name.as_str() {
                "main" => "_start",
                _ => name,
            };
            format!(
                "
section     .text
global      {}
{}:
{}
",
                function_name,
                function_name,
                generate(tree.children.first().expect("Function has no child"))
            )
        }
        NodeType::Stmt => {
            format!(
                "
{}
mov ebx, eax
mov eax, 1
int 0x80",
                generate(tree.children.first().expect("Statement has no child"))
            )
        }
        NodeType::Const(n) => {
            format!("mov eax, {}", n)
        }
        NodeType::UnaryOp(t) => match t {
            UnaryOp::Minus => {
                format!(
                    "{}
neg eax",
                    generate(tree.children.first().expect("Unary operator has no child."))
                )
            }
        },
        NodeType::BinaryOp(t) => match t {
            BinaryOp::Plus => {
                format!(
                    "{}
mov ebx, eax
{}
add eax, ebx",
                    generate(
                        tree.children
                            .get(1)
                            .expect("Binary operator '+' has no second child.")
                    ),
                    generate(
                        tree.children
                            .first()
                            .expect("Binary operator '+' has no childs.")
                    )
                )
            }
            BinaryOp::Minus => {
                format!(
                    "{}
mov ebx, eax
{}
sub eax, ebx",
                    generate(
                        tree.children
                            .get(1)
                            .expect("Binary operator '-' has no second child.")
                    ),
                    generate(
                        tree.children
                            .first()
                            .expect("Binary operator '-' has no childs.")
                    )
                )
            }
        },
    }
}
