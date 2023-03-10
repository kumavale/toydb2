use crate::token;

pub type Expression = token::Token;

#[derive(Debug, Default, PartialEq)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn push_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

#[derive(Debug, PartialEq)]
pub enum AstKind {
    Select,
    Create,
    Insert,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub select: Option<SelectStatement>,      //
    pub create: Option<CreateTableStatement>, // Golang no interface tekinano
    pub insert: Option<InsertStatement>,      //
    pub kind: AstKind,
}

impl Statement {
    pub fn new_select(stmt: SelectStatement) -> Self {
        Self {
            select: Some(stmt),
            create: None,
            insert: None,
            kind: AstKind::Select,
        }
    }

    pub fn new_create(stmt: CreateTableStatement) -> Self {
        Self {
            select: None,
            create: Some(stmt),
            insert: None,
            kind: AstKind::Create,
        }
    }

    pub fn new_insert(stmt: InsertStatement) -> Self {
        Self {
            select: None,
            create: None,
            insert: Some(stmt),
            kind: AstKind::Insert,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub item: Vec<Expression>,
    pub from: Expression, // A String is better
}

impl SelectStatement {
    pub fn new(item: Vec<Expression>, from: Expression) -> Self {
        Self { item, from }
    }
}

#[derive(Debug, PartialEq)]
pub struct InsertStatement {
    pub table: token::Token, // A String is better
    pub values: Vec<Expression>,
}

impl InsertStatement {
    pub fn new(table: token::Token, values: Vec<Expression>) -> Self {
        Self { table, values }
    }
}

#[derive(Debug, PartialEq)]
pub struct ColumnDefinition {
    pub name: token::Token,
    pub data_type: token::Token,
}

impl ColumnDefinition {
    pub fn new(name: token::Token, data_type: token::Token) -> Self {
        Self { name, data_type }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateTableStatement {
    pub name: token::Token, // A String is better
    pub cols: Vec<ColumnDefinition>,
}

impl CreateTableStatement {
    pub fn new(name: token::Token, cols: Vec<ColumnDefinition>) -> Self {
        Self { name, cols }
    }
}
