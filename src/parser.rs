use crate::lexer::Token;
use crate::{verify_token, verify_next_token};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum UnaryOp {
    Minus,
    Function(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiplication,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum NodeType {
    Prog(String),
    Fn(String, usize),      // name, number of variables
    VarDecl(String, usize), // name, offset
    Var(String, usize),     // name, offset
    Return,
    Const(i32),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseNode {
    pub node_type: NodeType,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new(node_type: NodeType) -> ParseNode {
        ParseNode {
            node_type,
            children: Vec::new(),
        }
    }

    pub fn get_child(&self, child: usize) -> &ParseNode {
        self.children.get(child).unwrap_or_else(|| {
            panic!(
                "{:?} has no {} child, it has {} children",
                self,
                child,
                self.children.len()
            )
        })
    }

    pub fn is_operation(&self) -> bool {
        matches!(self.node_type, NodeType::UnaryOp(_) | NodeType::BinaryOp(_))
    }
}

fn print_parse_node_tree(node: &ParseNode, indent: usize) {
    println!("{}| {:?}", "  ".repeat(indent), node.node_type);
    for child in node.children.iter() {
        print_parse_node_tree(child, indent + 1);
    }
}

pub fn print_ast(node: &ParseNode) {
    print_parse_node_tree(node, 0)
}

trait GetTokenOrPrintErr {
    fn get_token(&self, pos: usize) -> Result<&Token, String>;
    fn get_next_token(&self, pos: &mut usize) -> Result<&Token, String>;
}

impl GetTokenOrPrintErr for &[Token] {
    fn get_token(&self, pos: usize) -> Result<&Token, String> {
        match self.get(pos) {
            Some(x) => Ok(x),
            None => Err(format!(
                "No token exists at {}, the last token is {:?} at {}. ",
                pos,
                self.last(),
                self.len() - 1
            )),
        }
    }

    fn get_next_token(&self, pos: &mut usize) -> Result<&Token, String> {
        *pos += 1;
        let token = &self.get_token(*pos)?;
        Ok(token)
    }
}

fn parse_function(tokens: &[Token], pos: &mut usize) -> Result<ParseNode, String> {
    verify_token!(tokens, pos, Token::Int)?;
    verify_next_token!(tokens, pos, Token::Identifier { .. })?;
    verify_next_token!(tokens, pos, Token::LParenthesis)?;
    verify_next_token!(tokens, pos, Token::RParenthesis)?;
    verify_next_token!(tokens, pos, Token::LCurly)?;

    let mut func_node = ParseNode::new(NodeType::Fn("main".to_string(), 0));

    while !matches!(tokens.get_token(*pos + 1)?, Token::RCurly) {
        let stmt_node: ParseNode;
        stmt_node = parse_statement(tokens, pos)?;
        func_node.children.push(stmt_node);
    }

    verify_next_token!(tokens, pos, Token::RCurly)?;

    Ok(func_node)
}

fn parse_statement(tokens: &[Token], pos: &mut usize) -> Result<ParseNode, String> {
    let token = tokens.get_next_token(pos)?;

    match *token {
        Token::Return => {
            let exp_node = parse_expression(tokens, pos)?;

            Ok(ParseNode {
                node_type: NodeType::Return,
                children: vec![exp_node],
            })
        }
        Token::Int => match tokens.get_next_token(pos)? {
            Token::Identifier(x) => {
                verify_next_token!(tokens, pos, Token::Assignment)?;

                let exp_node = parse_expression(tokens, pos)?;

                Ok(ParseNode {
                    node_type: NodeType::VarDecl(x.clone(), 0),
                    children: vec![exp_node],
                })
            }
            _ => Err(format!("No identifier found at {}.", pos)),
        },
        _ => Err(format!(
            "Found statement with invalid starting token: {:?}",
            token
        )),
    }
}

fn get_precedence(token: &ParseNode) -> u32 {
    match &token.node_type {
        NodeType::UnaryOp(_) => 10,
        NodeType::BinaryOp(x) => match x {
            BinaryOp::Multiplication => 2,
            BinaryOp::Plus | BinaryOp::Minus => 1,
        },
        _ => panic!("Could not get precedence of a non operator token"),
    }
}

fn parse_output_stack(output_stack: &mut [ParseNode]) -> Result<ParseNode, String> {
    let mut stack: Vec<ParseNode> = vec![];

    for node in output_stack {
        if matches!(node.node_type, NodeType::UnaryOp(_)) {
            node.children.push(stack.remove(stack.len() - 1));
        } else if node.is_operation() {
            node.children.push(stack.remove(stack.len() - 2));
            node.children.push(stack.remove(stack.len() - 1));
        }

        stack.push(node.clone())
    }

    Ok(stack.remove(0))
}

fn convert_token_to_value(token: &Token) -> Result<NodeType, String> {
    match token {
        Token::Integer(x) => Ok(NodeType::Const(*x)),
        _ => Err(format!(
            "The token {:?} cannot be converted to a unary operation.",
            token
        )),
    }
}

fn get_unary_op_from_token(token: &Token) -> Result<UnaryOp, String> {
    match token {
        Token::Minus => Ok(UnaryOp::Minus),
        Token::Identifier(x) => Ok(UnaryOp::Function(x.clone())),
        _ => Err(format!(
            "The token {:?} cannot be converted to a unary operation.",
            token
        )),
    }
}

fn get_binary_op_from_token(token: &Token) -> Result<BinaryOp, String> {
    match token {
        Token::Minus => Ok(BinaryOp::Minus),
        Token::Plus => Ok(BinaryOp::Plus),
        Token::Multiplication => Ok(BinaryOp::Multiplication),
        _ => Err(format!(
            "The token {:?} cannot be converted to a binary operation.",
            token
        )),
    }
}

fn process_operation(
    token: &Token,
    output_stack: &mut Vec<ParseNode>,
    operator_stack: &mut Vec<ParseNode>,
    previous_was_value: &mut bool,
) -> Result<(), String> {
    let node: ParseNode;

    if !*previous_was_value {
        node = ParseNode::new(NodeType::UnaryOp(get_unary_op_from_token(token)?));
    } else {
        node = ParseNode::new(NodeType::BinaryOp(get_binary_op_from_token(token)?));

        let previous_operator_precedence = match operator_stack.first() {
            Some(x) => get_precedence(x),
            None => 0,
        };
        if get_precedence(&node) < previous_operator_precedence {
            // TODO FIX: left associative should take precedence if precedence is equal
            output_stack.append(operator_stack);
        }
    }

    (*operator_stack).insert(0, node); // Change vec to stack
    *previous_was_value = false;

    Ok(())
}

fn process_value(
    token: &Token,
    output_stack: &mut Vec<ParseNode>,
    previous_was_value: &mut bool,
) -> Result<(), String> {
    let node: ParseNode;

    if matches!(token, Token::Identifier { .. }) {
        node = ParseNode::new(NodeType::Var(token.get_string_value().clone(), 0));
    } else {
        node = ParseNode::new(convert_token_to_value(&token.clone())?);
    }

    output_stack.push(node);
    *previous_was_value = true;

    Ok(())
}

// Source: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<ParseNode, String> {
    let mut output_stack: Vec<ParseNode> = vec![];
    let mut operator_stack: Vec<ParseNode> = vec![];
    let mut previous_was_value: bool = false;

    let mut token = tokens.get_next_token(pos)?;
    while !matches!(token, Token::SemiColon) {
        let mut is_function: bool = false;

        if matches!(token, Token::Identifier { .. }) {
            is_function = matches!(tokens.get_token(*pos + 1)?, Token::LParenthesis);
        }

        if is_function || token.is_operation() {
            process_operation(
                token,
                &mut output_stack,
                &mut operator_stack,
                &mut previous_was_value,
            )?;
        } else {
            process_value(token, &mut output_stack, &mut previous_was_value)?;
        }

        token = tokens.get_next_token(pos)?;
    }

    output_stack.append(&mut operator_stack);

    Ok(parse_output_stack(&mut output_stack)?)
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<ParseNode, String> {
    let mut pos = 0;
    parse_function(&tokens, &mut pos).and_then(|node| {
        if (1 + pos) == tokens.len() {
            Ok(node)
        } else {
            Err(format!(
                "Parsing Error: Expected end of input, found {:?} at {}",
                &tokens[pos], pos
            ))
        }
    })
}
