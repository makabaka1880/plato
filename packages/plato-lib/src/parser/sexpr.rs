/// An s-expression: either an atom (symbol) or a list of sub-expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

// ── Tokenizer ───────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ';' => {
                // line comment — skip to end of line
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\n' {
                        break;
                    }
                }
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                let mut sym = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '(' || c == ')' {
                        break;
                    }
                    sym.push(c);
                    chars.next();
                }
                tokens.push(Token::Symbol(sym));
            }
        }
    }
    tokens
}

// ── S-expression parser ─────────────────────────────────────────

pub fn parse_sexpr(tokens: &[Token], pos: &mut usize) -> Result<SExpr, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of input".into());
    }
    match &tokens[*pos] {
        Token::LParen => {
            *pos += 1;
            let mut items = Vec::new();
            while *pos < tokens.len() && tokens[*pos] != Token::RParen {
                items.push(parse_sexpr(tokens, pos)?);
            }
            if *pos >= tokens.len() {
                return Err("unclosed '('".into());
            }
            *pos += 1; // skip ')'
            Ok(SExpr::List(items))
        }
        Token::RParen => Err("unexpected ')'".into()),
        Token::Symbol(s) => {
            *pos += 1;
            Ok(SExpr::Atom(s.clone()))
        }
    }
}

pub fn parse_one(tokens: &[Token]) -> Result<SExpr, String> {
    let mut pos = 0;
    let expr = parse_sexpr(tokens, &mut pos)?;
    // trailing tokens — accept but ignore
    Ok(expr)
}

/// Parse a string to an SExpr, ignoring whitespace differences.
fn parse_str(s: &str) -> Result<SExpr, String> {
    let tokens = tokenize(s);
    if tokens.is_empty() {
        return Err("empty input".into());
    }
    parse_one(&tokens)
}

/// Compare two tactic strings for structural equality.
/// Normalises whitespace and parses to AST, so e.g.
/// `(assume I)` equals `(assume  I)`.
pub fn tactic_equals(a: &str, b: &str) -> bool {
    match (parse_str(a), parse_str(b)) {
        (Ok(ea), Ok(eb)) => ea == eb,
        _ => false,
    }
}
