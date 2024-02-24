use crate::parser::{NodeType, ParseNode};

pub fn generate(tree: &ParseNode) -> String {
    match &tree.entry {
        NodeType::Prog(name) => {
            name.to_string()
        }
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
mov eax, 1
mov ebx, {}
int 0x80",
                generate(tree.children.first().expect("Statement has no child"))
            )
        }
        NodeType::Exp(n) => {
            format!("{}", n)
        }
    }
}
