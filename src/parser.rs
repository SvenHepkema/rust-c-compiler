use crate::lexer::Token;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum UnaryOp {
    Minus,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
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

    pub fn print(&self, indent: usize) {
        println!("{}| {:?}", "  ".repeat(indent), self.entry);
        for child in self.children.iter() {
            child.print(indent + 1);
        }
    }

    pub fn get_child(&self, child: usize) -> &ParseNode {
        self.children.get(child).expect(&format!(
            "{:?} has no {} child, it has {} children",
            self,
            child,
            self.children.len()
        ))
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
        _ => Err(format!(
            "The token {:?} cannot be converted to a unary operation.",
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

fn parse_expression(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let token = &tokens[pos];

    match *token {
        Token::Minus | Token::Plus => match tokens[pos - 1] {
            Token::Integer(_) => {
                let left_child_node = match tokens[pos - 1] {
                    Token::Integer(x) => ParseNode {
                        entry: NodeType::Const(x),
                        children: vec![],
                    },
                    _ => {
                        panic!(
                            "Expected integer as left child but found {:?} at {}",
                            tokens.get(pos),
                            pos
                        );
                    }
                };

                let (right_child_node, pos) = parse_expression(tokens, pos + 1)?;

                return Ok((
                    ParseNode {
                        entry: NodeType::BinaryOp(convert_token_to_binary_op(token)?),
                        children: vec![left_child_node, right_child_node],
                    },
                    pos,
                ));
            }
            _ => {
                // Previous token is not an integer, so we are dealing with a unary operation
                let (child_node, pos) = parse_expression(tokens, pos + 1)?;

                return Ok((
                    ParseNode {
                        entry: NodeType::UnaryOp(convert_token_to_unary_op(token)?),
                        children: vec![child_node],
                    },
                    pos,
                ));
            }
        },
        Token::Integer(x) => match tokens[pos + 1] {
            Token::Minus | Token::Plus => Ok(parse_expression(tokens, pos + 1)?),
            _ => Ok((ParseNode::new_childless(NodeType::Const(x)), pos + 1)),
        },
        _ => {
            panic!("Expected integer but found {:?} at {}", tokens[pos], pos);
        }
    }
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
