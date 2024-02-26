use crate::lexer::Token;
use crate::parser::{NodeType, ParseNode};

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
        NodeType::Exp(n) => {
            format!("mov eax, {}", n)
        }
        NodeType::UnExp(t) => match t {
            Token::Minus => {
                format!(
                    "{}
neg eax",
                    generate(tree.children.first().expect("Unary operator has no child."))
                )
            }
            _ => {
                // FIX this is ugly should be done by typesystme, it should not be possible
                // to create a unary node with a non unary operator token at all
                panic!("Invalid unary operator type: {:?}", t);
            }
        },
    }
}
