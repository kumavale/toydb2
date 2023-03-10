use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Keyword = &'static str;
pub const SELECT: Keyword = "select";
pub const FROM:   Keyword = "from";
pub const AS:     Keyword = "as";
pub const TABLE:  Keyword = "table";
pub const CREATE: Keyword = "create";
pub const INSERT: Keyword = "insert";
pub const INTO:   Keyword = "into";
pub const VALUES: Keyword = "values";
pub const INT:    Keyword = "int";
pub const TEXT:   Keyword = "text";
pub const NONE:   Keyword = "none";

pub type Symbol = &'static str;
pub const SEMICOLON: Symbol = ";";
pub const ASTERISK:  Symbol = "*";
pub const COMMA:     Symbol = ",";
pub const LPAREN:    Symbol = "(";
pub const RPAREN:    Symbol = ")";

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    String(String), // Expression
    Number(i64),    // Expression
    Illegal(String),
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Token {
        Token { kind, line }
    }

    pub fn expect(&self, expect: TokenKind) -> Result<(), String> {
        if self.kind == expect {
            Ok(())
        } else {
            Err(format!(
                "{}: error: expect={:?}. got={:?}",
                self.line, expect, self.kind
            ))
        }
    }

    pub fn expect_expression(&self) -> bool {
        matches!(self.kind, TokenKind::String(_) | TokenKind::Number(_))
    }
}

lazy_static! {
    static ref VALID_KEYWORDS: HashMap<String, Keyword> = {
        let mut m = HashMap::new();
        m.insert("select".to_string(), SELECT);
        m.insert("from".to_string(),   FROM);
        m.insert("as".to_string(),     AS);
        m.insert("table".to_string(),  TABLE);
        m.insert("create".to_string(), CREATE);
        m.insert("insert".to_string(), INSERT);
        m.insert("into".to_string(),   INTO);
        m.insert("values".to_string(), VALUES);
        m.insert("int".to_string(),    INT);
        m.insert("text".to_string(),   TEXT);
        m.insert("none".to_string(),   NONE);
        m
    };
}

pub fn is_keyword(s: &str) -> Option<Keyword> {
    let lower_s: String = s.to_ascii_lowercase();
    if let Some(keyword) = VALID_KEYWORDS.get(&lower_s) {
        Some(keyword)
    } else {
        None
    }
}
