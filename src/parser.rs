use crate::lexer::Token;

// Copied (and edited) from https://github.com/onehr/crust/commit/a708fbc04bf395425197a11ca1f0fde0ed49d865#diff-4a04259da480a6b794a2e947e4cc03eff4d1aa9330836f5b91cac68c5398193f

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum NodeType {
    Prog(String),
    Fn(String),
    Stmt,
    Exp(i64),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseNode {
    pub entry: NodeType,
    pub children: Vec<ParseNode>,
}

impl Default for ParseNode {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: NodeType::Prog("root".to_string()),
        }
    }

    pub fn print(&self, indent: usize) {
        println!("{}| {:?}", "  ".repeat(indent), self.entry);
        for child in self.children.iter() {
            child.print(indent + 1);
        }
    }
}

fn parse_function(toks: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Int {
        // TODO: Chnage this to Token::Keyword::Type and then check if int
        // float etc.
        return Err(format!("Expected `int`, found {:?} at {}", toks[pos], pos));
    }
    let mut pos = pos + 1;

    let tok = &toks[pos];
    if *tok != Token::Identifier("main".to_string()) {
        return Err(format!("Expected `main`, found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let tok = &toks[pos];
    if *tok != Token::LParenthesis {
        return Err(format!("Expected `(`, found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let tok = &toks[pos];
    if *tok != Token::RParenthesis {
        return Err(format!("Expected `)`, found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let tok = &toks[pos];
    if *tok != Token::LCurly {
        return Err(format!("Expected `{{`, found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let mut stmt_node = ParseNode::new();
    let tmp = parse_statement(toks, pos);
    if let Ok((a, b)) = tmp {
        stmt_node = a;
        pos = b;
    }

    let tok = &toks[pos];
    if *tok != Token::RCurly {
        return Err(format!("Expected `}}`, found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let mut fn_node = ParseNode::new();
    fn_node.entry = NodeType::Fn("main".to_string());
    fn_node.children.push(stmt_node);

    Ok((fn_node, pos))
}

fn parse_statement(toks: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Return {
        return Err(format!(
            "Expected 'return', found {:?} at {}",
            toks[pos], pos
        ));
    }
    let mut pos = pos + 1;

    let mut exp_node = ParseNode::new();
    let tmp = parse_expression(toks, pos);
    if let Ok((a, b)) = tmp {
        exp_node = a;
        pos = b;
    }

    let tok = &toks[pos];
    if *tok != Token::SemiColon {
        return Err(format!("Expected ';', found {:?} at {}", toks[pos], pos));
    }
    pos += 1;

    let mut stmt_node = ParseNode::new();
    stmt_node.entry = NodeType::Stmt;
    stmt_node.children.push(exp_node);

    Ok((stmt_node, pos))
}

fn parse_expression(toks: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Integer(10) {
        panic!("Expected 'Integer(10)`, found {:?} at {}", toks[pos], pos);
    }
    let pos = pos + 1;

    let mut exp_node = ParseNode::new();
    exp_node.entry = NodeType::Exp(10);

    Ok((exp_node, pos))
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
