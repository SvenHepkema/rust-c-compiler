use std::collections::HashMap;

use crate::constants::QUADWORD_LENGTH;
use crate::parser::{NodeType, ParseNode};

#[derive(Clone, Debug)]
struct Variable {
    name: String,
    offset: usize,
}

#[derive(Clone, Debug)]
struct SymbolTable {
    variables: HashMap<String, Variable>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    fn add(&mut self, name: &String, offset: usize) {
        self.variables.insert(
            name.to_owned(),
            Variable {
                name: name.to_owned(),
                offset,
            },
        );
    }

    fn get_var(&self, name: &String) -> Variable {
        self.variables[name].clone()
    }

    fn len(&self) -> usize {
        self.variables.len()
    }
}

fn scan_for_symbols(node: &mut ParseNode, symbols: &SymbolTable) {
    match &node.node_type {
        NodeType::Var(name, _) => {
            let var = symbols.get_var(&name);
            node.node_type = NodeType::Var(var.name, var.offset)
        }
        _ => {
            for child in node.children.iter_mut() {
                scan_for_symbols(child, symbols)
            }
        }
    }
}

pub fn symbolpass(node: &mut ParseNode) {
    match &node.node_type {
        NodeType::Fn(fn_name, _) => {
            let mut offset = QUADWORD_LENGTH;
            let mut symbols = SymbolTable::new();

            for child in node.children.iter_mut() {
                scan_for_symbols(child, &symbols);
                match &child.node_type {
                    NodeType::VarDecl(name, _) => {
                        symbols.add(&name, offset);
                        child.node_type = NodeType::VarDecl(name.clone(), offset);
                        offset += QUADWORD_LENGTH;
                    }
                    _ => {}
                }
            }

            node.node_type = NodeType::Fn(fn_name.clone(), symbols.len())
        }
        _ => {}
    }
}
