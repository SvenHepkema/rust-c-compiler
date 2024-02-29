use crate::lexer::Token;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum UnaryOp {
    Minus,
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
    Fn(String),
    Stmt,
    Const(i32),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseNode {
    pub entry: NodeType,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new(node_type: NodeType) -> ParseNode {
        ParseNode {
            entry: node_type,
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
        matches!(self.entry, NodeType::UnaryOp(_) | NodeType::BinaryOp(_))
    }
}

fn print_parse_node_tree(node: &ParseNode, indent: usize) {
    println!("{}| {:?}", "  ".repeat(indent), node.entry);
    for child in node.children.iter() {
        print_parse_node_tree(child, indent + 1);
    }
}

pub fn print_ast(node: &ParseNode) {
    print_parse_node_tree(node, 0)
}

fn assert_next(expected_token: Token, tokens: &[Token], pos: usize) -> Result<usize, String> {
    let token = match tokens.get(pos) {
        Some(x) => x,
        None => return Err("Next token does not exist.".to_string()),
    };

    if *token != expected_token {
        return Err(format!(
            "Expected {:?}, but found {:?} at {}",
            expected_token, *token, pos
        ));
    }

    Ok(pos + 1)
}

fn parse_function(tokens: &[Token], mut pos: usize) -> Result<(ParseNode, usize), String> {
    pos = assert_next(Token::Int, tokens, pos)?;
    pos = assert_next(Token::Identifier("main".to_string()), tokens, pos)?;
    pos = assert_next(Token::LParenthesis, tokens, pos)?;
    pos = assert_next(Token::RParenthesis, tokens, pos)?;
    pos = assert_next(Token::LCurly, tokens, pos)?;

    let (stmt_node, mut pos) = parse_statement(tokens, pos)?;

    pos = assert_next(Token::RCurly, tokens, pos)?;

    Ok((
        ParseNode {
            entry: NodeType::Fn("main".to_string()),
            children: vec![stmt_node],
        },
        pos,
    ))
}

fn parse_statement(tokens: &[Token], mut pos: usize) -> Result<(ParseNode, usize), String> {
    pos = assert_next(Token::Return, tokens, pos)?;

    let (exp_node, mut pos) = parse_expression(tokens, pos)?;

    pos = assert_next(Token::SemiColon, tokens, pos)?;

    Ok((
        ParseNode {
            entry: NodeType::Stmt,
            children: vec![exp_node],
        },
        pos,
    ))
}

fn get_precedence(token: &ParseNode) -> u32 {
    match &token.entry {
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
        if matches!(node.entry, NodeType::UnaryOp(_)) {
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

fn convert_token_to_unary_op(token: &Token) -> Result<UnaryOp, String> {
    match token {
        Token::Minus => Ok(UnaryOp::Minus),
        _ => Err(format!(
            "The token {:?} cannot be converted to a unary operation.",
            token
        )),
    }
}

fn convert_token_to_binary_op(token: &Token) -> Result<BinaryOp, String> {
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

// Source: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn parse_expression(tokens: &[Token], mut pos: usize) -> Result<(ParseNode, usize), String> {
    let mut token = &tokens[pos];

    let mut output_stack: Vec<ParseNode> = vec![];
    let mut operator_stack: Vec<ParseNode> = vec![];

    let mut previous_was_value: bool = false;

    while *token != Token::SemiColon {
        let node: ParseNode;
        if token.is_operation() {
            if !previous_was_value && token.is_operation() {
                node = ParseNode::new(NodeType::UnaryOp(convert_token_to_unary_op(token)?));
            } else {
                node = ParseNode::new(NodeType::BinaryOp(convert_token_to_binary_op(token)?));

                let precedence = get_precedence(&node);
                if precedence
                    < match operator_stack.first() {
                        Some(x) => get_precedence(x),
                        None => 0,
                    }
                {
                    // TODO FIX: left associative should take precedence if precedence is equal
                    output_stack.append(&mut operator_stack);
                }
            }

            operator_stack.insert(0, node); // Change vec to stack
            previous_was_value = false;
        } else {
            node = ParseNode::new(convert_token_to_value(&token.clone())?);
            output_stack.push(node);
            previous_was_value = true;
        }

        pos += 1;
        token = &tokens[pos];
    }

    output_stack.append(&mut operator_stack);

    Ok((parse_output_stack(&mut output_stack)?, pos))
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<ParseNode, String> {
    parse_function(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Parsing Error: Expected end of input, found {:?} at {}",
                &tokens[i], i
            ))
        }
    })
}
