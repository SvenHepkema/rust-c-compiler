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
    pub fn new_childless(node_type: NodeType) -> ParseNode {
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
        match self.entry {
            NodeType::UnaryOp(_) | NodeType::BinaryOp(_) => true,
            _ => false,
        }
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

fn parse_literal(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let token = &tokens[pos];

    match *token {
        Token::Integer(x) => Ok((
            ParseNode {
                entry: NodeType::Const(x),
                children: vec![],
            },
            pos,
        )),
        _ => Err(format!(
            "Expected token to convert to literal but found {:?} at {}",
            tokens.get(pos),
            pos
        )),
    }
}

//fn parse_expression(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
//    let token = &tokens[pos];
//
//    match *token {
//        Token::Minus | Token::Plus | Token::Multiplication => match tokens[pos - 1] {
//            Token::Integer(_) => {
//                let left_child_node = match tokens[pos - 1] {
//                    Token::Integer(x) => ParseNode {
//                        entry: NodeType::Const(x),
//                        children: vec![],
//                    },
//                    _ => {
//                        return Err(format!(
//                            "Expected integer as left child but found {:?} at {}",
//                            tokens.get(pos),
//                            pos
//                        ));
//                    }
//                };
//
//                let (right_child_node, pos) = parse_expression(tokens, pos + 1)?;
//
//                Ok((
//                    ParseNode {
//                        entry: NodeType::BinaryOp(convert_token_to_binary_op(token)?),
//                        children: vec![left_child_node, right_child_node],
//                    },
//                    pos,
//                ))
//            }
//            _ => {
//                // Previous token is not an integer, so we are dealing with a unary operation
//                let (child_node, pos) = parse_expression(tokens, pos + 1)?;
//
//                Ok((
//                    ParseNode {
//                        entry: NodeType::UnaryOp(convert_token_to_unary_op(token)?),
//                        children: vec![child_node],
//                    },
//                    pos,
//                ))
//            }
//        },
//        Token::Integer(x) => match tokens[pos + 1] {
//            Token::Minus | Token::Plus | Token::Multiplication => {
//                Ok(parse_literal(tokens, pos + 1)?)
//            }
//            _ => Ok((ParseNode::new_childless(NodeType::Const(x)), pos + 1)),
//        },
//        _ => {
//            return Err(format!(
//                "Unexpected token found during expression parsing at {:?} at {}",
//                tokens[pos], pos
//            ));
//        }
//    }
//}

fn get_precedence(token: &Token) -> u32 {
    match token {
        Token::Multiplication => 2,
        Token::Plus | Token::Minus => 1,
        _ => panic!("Could not get precedence of a non operator token"),
    }
}

fn parse_expr_token(token: &Token) -> ParseNode {
    match token {
        Token::Integer(x) => ParseNode {
            entry: NodeType::Const(*x),
            children: vec![],
        },
        Token::Multiplication => ParseNode {
            entry: NodeType::BinaryOp(BinaryOp::Multiplication),
            children: vec![],
        },
        Token::Plus => ParseNode {
            entry: NodeType::BinaryOp(BinaryOp::Plus),
            children: vec![],
        },
        Token::Minus => ParseNode {
            entry: NodeType::BinaryOp(BinaryOp::Minus),
            children: vec![],
        },
        _ => panic!("Parse expr token function could not parse token.",),
    }
}

fn parse_output_stack(output_stack: &[&Token]) -> Result<ParseNode, String> {
    let mut stack: Vec<ParseNode> = vec![];

    println!("Parse output stack: {:?}", output_stack);

    for token in output_stack {
        let mut parsed = parse_expr_token(token);

        if parsed.is_operation() {
            parsed.children.push(stack.remove(stack.len() - 2));
            parsed.children.push(stack.remove(stack.len() - 1));
            //parsed.children.extend_from_slice(&mut stack.drain(stack.len()-2..).as_slice());
        }

        stack.push(parsed)
    }

    Ok(stack.remove(0))
}

// Source: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn parse_expression(tokens: &[Token], mut pos: usize) -> Result<(ParseNode, usize), String> {
    let mut token = &tokens[pos];

    let mut output_stack: Vec<&Token> = vec![];
    let mut operator_stack: Vec<&Token> = vec![];

    while *token != Token::SemiColon {
        match *token {
            Token::Minus | Token::Plus | Token::Multiplication => {
                let precedence = get_precedence(token);
                if precedence
                    < match operator_stack.first() {
                        Some(x) => get_precedence(x),
                        None => 0,
                    }
                {
                    // TODO FIX: left associative should take precedence if precedence is equal
                    output_stack.append(&mut operator_stack);
                }
                operator_stack.insert(0, token); // Change vec to stack
            }
            Token::Integer(_) => {
                output_stack.push(token);
            }
            _ => {
                return Err(format!(
                    "Unexpected token found during expression parsing at {:?} at {}",
                    tokens[pos], pos
                ));
            }
        }

        pos += 1;
        token = &tokens[pos];
    }

    output_stack.append(&mut operator_stack);

    Ok((parse_output_stack(&output_stack)?, pos))
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
