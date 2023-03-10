use crate::ast::*;
use crate::lexer;
use crate::token;
use crate::token::TokenKind;

pub fn parse(source: &str) -> Result<Ast, String> {
    let mut tokens = lexer::Lexer::new(source);
    let mut node: Ast = Default::default();

    tokens.next_token();
    while tokens.token().kind != TokenKind::EOF {
        let stmt = match parse_statement(&mut tokens) {
            Ok(stmt) => stmt,
            Err(e) => return Err(e),
        };

        node.push_statement(stmt);

        tokens.next_token();
        tokens.token().expect(TokenKind::Symbol(token::SEMICOLON))?;

        tokens.next_token();
    }

    Ok(node)
}

fn parse_statement(tokens: &mut lexer::Lexer) -> Result<Statement, String> {
    match tokens.token().kind {
        // SELECT
        TokenKind::Keyword(token::SELECT) => parse_select_statement(tokens),
        // CREATE TABLE
        TokenKind::Keyword(token::CREATE) => parse_create_table_statement(tokens),
        // INSERT
        TokenKind::Keyword(token::INSERT) => parse_insert_statement(tokens),

        _ => Err(error_message("invalid keyword", tokens.token())),
    }
}

fn parse_select_statement(tokens: &mut lexer::Lexer) -> Result<Statement, String> {
    tokens.next_token();
    let mut item: Vec<Expression> = Vec::new();

    // column name
    if let TokenKind::Identifier(_) = tokens.token().kind {
        item.push(tokens.token().clone())
    } else {
        return Err(error_message("expect column name", tokens.token()));
    }

    // until FROM
    tokens.next_token();
    while tokens.token().kind != TokenKind::Keyword(token::FROM) {
        // ,
        tokens.token().expect(TokenKind::Symbol(token::COMMA))?;

        // column
        tokens.next_token();
        if let TokenKind::Identifier(_) = tokens.token().kind {
            item.push(tokens.token().clone())
        } else {
            return Err(error_message("expect column name", tokens.token()));
        }
        tokens.next_token();
    }

    // Table name
    tokens.next_token();
    let table = if let TokenKind::Identifier(_) = tokens.token().kind {
        tokens.token().clone()
    } else {
        return Err(error_message("expect table name", tokens.token()));
    };

    let stmt = SelectStatement::new(item, table);
    Ok(Statement::new_select(stmt))
}

fn parse_create_table_statement(tokens: &mut lexer::Lexer) -> Result<Statement, String> {
    // TABLE
    tokens.next_token();
    tokens.token().expect(TokenKind::Keyword(token::TABLE))?;

    // Table name
    tokens.next_token();
    let table = if let TokenKind::Identifier(_) = tokens.token().kind {
        tokens.token().clone()
    } else {
        return Err(error_message("expect table name", tokens.token()));
    };

    // (
    tokens.next_token();
    tokens.token().expect(TokenKind::Symbol(token::LPAREN))?;

    // [column-name column-type [, ...]]
    let mut cols: Vec<ColumnDefinition> = Vec::new();
    tokens.next_token();
    if let TokenKind::Identifier(_) = tokens.token().kind {
        let col_name = tokens.token().clone();
        tokens.next_token();
        if let TokenKind::Keyword(_) = tokens.token().kind {
            let col_type = tokens.token().clone();
            cols.push(ColumnDefinition::new(col_name, col_type));
        } else {
            return Err(error_message("expect column data-type", tokens.token()));
        }
    } else {
        return Err(error_message("expect column name", tokens.token()));
    }

    // until ')'
    tokens.next_token();
    while tokens.token().kind != TokenKind::Symbol(token::RPAREN) {
        tokens.token().expect(TokenKind::Symbol(token::COMMA))?;

        tokens.next_token();
        if let TokenKind::Identifier(_) = tokens.token().kind {
            let col_name = tokens.token().clone();
            tokens.next_token();
            if let TokenKind::Keyword(_) = tokens.token().kind {
                let col_type = tokens.token().clone();
                cols.push(ColumnDefinition::new(col_name, col_type));
            } else {
                return Err(error_message("expect column data-type", tokens.token()));
            }
        } else {
            return Err(error_message("expect column name", tokens.token()));
        }
        tokens.next_token();
    }

    let stmt = CreateTableStatement::new(table, cols);
    Ok(Statement::new_create(stmt))
}

fn parse_insert_statement(tokens: &mut lexer::Lexer) -> Result<Statement, String> {
    // INTO
    tokens.next_token();
    tokens.token().expect(TokenKind::Keyword(token::INTO))?;

    // Table name
    tokens.next_token();
    let table = if let TokenKind::Identifier(_) = tokens.token().kind {
        tokens.token().clone()
    } else {
        return Err(error_message("expect table name", tokens.token()));
    };

    // VALUES
    tokens.next_token();
    tokens.token().expect(TokenKind::Keyword(token::VALUES))?;

    // (
    tokens.next_token();
    tokens.token().expect(TokenKind::Symbol(token::LPAREN))?;

    // Expression
    let mut values: Vec<Expression> = Vec::new();
    tokens.next_token();
    if tokens.token().expect_expression() {
        values.push(tokens.token().clone());
    } else {
        return Err(error_message("expect expression", tokens.token()));
    }

    // until ')'
    tokens.next_token();
    while tokens.token().kind != TokenKind::Symbol(token::RPAREN) {
        tokens.token().expect(TokenKind::Symbol(token::COMMA))?;

        tokens.next_token();
        if tokens.token().expect_expression() {
            values.push(tokens.token().clone());
        } else {
            return Err(error_message("expect expression", tokens.token()));
        }
        tokens.next_token();
    }

    let stmt = InsertStatement::new(table, values);
    Ok(Statement::new_insert(stmt))
}

fn error_message(emsg: &str, actual: &token::Token) -> String {
    format!("{}: error: {}. got={:?}", actual.line, emsg, actual.kind)
}

#[cfg(test)]
mod tests {
    use super::TokenKind;
    use super::*;

    #[test]
    fn test_error_message() {
        assert_eq!(
            &error_message(
                &format!("expect={:?}", TokenKind::Symbol(token::SEMICOLON)),
                &token::Token::new(TokenKind::Symbol(token::LPAREN), 1)
            ),
            r#"1: error: expect=Symbol(";"). got=Symbol("(")"#
        );
        assert_eq!(
            &error_message(
                &format!("expect={:?}", TokenKind::Keyword(token::SELECT)),
                &token::Token::new(TokenKind::Number(42), 99)
            ),
            r#"99: error: expect=Keyword("select"). got=Number(42)"#
        );
        assert_eq!(
            &error_message(
                &format!("expect={:?}", TokenKind::Symbol(token::SEMICOLON)),
                &token::Token::new(TokenKind::EOF, 100)
            ),
            r#"100: error: expect=Symbol(";"). got=EOF"#
        );
    }

    #[test]
    fn test_parse_statement() {
        struct Test {
            input: &'static str,
            expect: Result<Statement, String>,
        }
        let tests = vec![
            Test {
                input: "",
                expect: Err(r#"1: error: invalid keyword. got=EOF"#.to_string()),
            },
            Test {
                input: "invalidkeyword",
                expect: Err(
                    r#"1: error: invalid keyword. got=Identifier("invalidkeyword")"#.to_string(),
                ),
            },
            Test {
                input: "select id, name from users;",
                expect: Ok(Statement {
                    select: Some(SelectStatement {
                        item: vec![
                            token::Token {
                                kind: TokenKind::Identifier("id".to_string()),
                                line: 1,
                            },
                            token::Token {
                                kind: TokenKind::Identifier("name".to_string()),
                                line: 1,
                            },
                        ],
                        from: token::Token {
                            kind: TokenKind::Identifier("users".to_string()),
                            line: 1,
                        },
                    }),
                    create: None,
                    insert: None,
                    kind: AstKind::Select,
                }),
            },
            Test {
                input: "select from users;",
                expect: Err(r#"1: error: expect column name. got=Keyword("from")"#.to_string()),
            },
            Test {
                input: "select id name from users;",
                expect: Err(r#"1: error: expect=Symbol(","). got=Identifier("name")"#.to_string()),
            },
            Test {
                input: "select id, from users;",
                expect: Err(r#"1: error: expect column name. got=Keyword("from")"#.to_string()),
            },
            Test {
                input: "select id, name from ;",
                expect: Err(r#"1: error: expect table name. got=Symbol(";")"#.to_string()),
            },
            Test {
                input: "create table users (id INT, name TEXT));",
                expect: Ok(Statement {
                    select: None,
                    create: Some(CreateTableStatement {
                        name: token::Token {
                            kind: TokenKind::Identifier("users".to_string()),
                            line: 1,
                        },
                        cols: vec![
                            ColumnDefinition {
                                name: token::Token {
                                    kind: TokenKind::Identifier("id".to_string()),
                                    line: 1,
                                },
                                data_type: token::Token {
                                    kind: TokenKind::Keyword(token::INT),
                                    line: 1,
                                },
                            },
                            ColumnDefinition {
                                name: token::Token {
                                    kind: TokenKind::Identifier("name".to_string()),
                                    line: 1,
                                },
                                data_type: token::Token {
                                    kind: TokenKind::Keyword(token::TEXT),
                                    line: 1,
                                },
                            },
                        ],
                    }),
                    insert: None,
                    kind: AstKind::Create,
                }),
            },
            Test {
                input: "create users (id INT);",
                expect: Err(
                    r#"1: error: expect=Keyword("table"). got=Identifier("users")"#.to_string(),
                ),
            },
            Test {
                input: "create table (id INT);",
                expect: Err(r#"1: error: expect table name. got=Symbol("(")"#.to_string()),
            },
            Test {
                input: "create table users id INT);",
                expect: Err(r#"1: error: expect=Symbol("("). got=Identifier("id")"#.to_string()),
            },
            Test {
                input: "create table users (INT);",
                expect: Err(r#"1: error: expect column name. got=Keyword("int")"#.to_string()),
            },
            Test {
                input: "create table users (id);",
                expect: Err(r#"1: error: expect column data-type. got=Symbol(")")"#.to_string()),
            },
            Test {
                input: "create table users (id INT name TEXT);",
                expect: Err(r#"1: error: expect=Symbol(","). got=Identifier("name")"#.to_string()),
            },
            Test {
                input: "insert into users values (1, 'Alice');",
                expect: Ok(Statement {
                    select: None,
                    create: None,
                    insert: Some(InsertStatement {
                        table: token::Token {
                            kind: TokenKind::Identifier("users".to_string()),
                            line: 1,
                        },
                        values: vec![
                            token::Token {
                                kind: TokenKind::Number(1),
                                line: 1,
                            },
                            token::Token {
                                kind: TokenKind::String("Alice".to_string()),
                                line: 1,
                            },
                        ],
                    }),
                    kind: AstKind::Insert,
                }),
            },
            Test {
                input: "insert users values (1, 'Alice');",
                expect: Err(
                    r#"1: error: expect=Keyword("into"). got=Identifier("users")"#.to_string(),
                ),
            },
            Test {
                input: "insert into values (1, 'Alice');",
                expect: Err(r#"1: error: expect table name. got=Keyword("values")"#.to_string()),
            },
            Test {
                input: "insert into users (1, 'Alice');",
                expect: Err(r#"1: error: expect=Keyword("values"). got=Symbol("(")"#.to_string()),
            },
            Test {
                input: "insert into users values 1, 'Alice');",
                expect: Err(r#"1: error: expect=Symbol("("). got=Number(1)"#.to_string()),
            },
            Test {
                input: "insert into users values ( , 'Alice');",
                expect: Err(r#"1: error: expect expression. got=Symbol(",")"#.to_string()),
            },
            Test {
                input: "insert into users values (1 'Alice');",
                expect: Err(r#"1: error: expect=Symbol(","). got=String("Alice")"#.to_string()),
            },
        ];

        for test in tests.iter() {
            let mut l = lexer::Lexer::new(&test.input);
            l.next_token();
            assert_eq!(parse_statement(&mut l), test.expect);
        }
    }
    #[test]
    fn test_parse() {
        struct Test {
            input: &'static str,
            expect: Result<Ast, String>,
        }
        let tests = vec![Test {
            input: "create table users (id INT, name TEXT);
                        insert into users values (1, 'Alice');
                        select id, name from users;",
            expect: Ok(Ast {
                statements: vec![
                    Statement {
                        select: None,
                        create: Some(CreateTableStatement {
                            name: token::Token {
                                kind: TokenKind::Identifier("users".to_string()),
                                line: 1,
                            },
                            cols: vec![
                                ColumnDefinition {
                                    name: token::Token {
                                        kind: TokenKind::Identifier("id".to_string()),
                                        line: 1,
                                    },
                                    data_type: token::Token {
                                        kind: TokenKind::Keyword(token::INT),
                                        line: 1,
                                    },
                                },
                                ColumnDefinition {
                                    name: token::Token {
                                        kind: TokenKind::Identifier("name".to_string()),
                                        line: 1,
                                    },
                                    data_type: token::Token {
                                        kind: TokenKind::Keyword(token::TEXT),
                                        line: 1,
                                    },
                                },
                            ],
                        }),
                        insert: None,
                        kind: AstKind::Create,
                    },
                    Statement {
                        select: None,
                        create: None,
                        insert: Some(InsertStatement {
                            table: token::Token {
                                kind: TokenKind::Identifier("users".to_string()),
                                line: 2,
                            },
                            values: vec![
                                token::Token {
                                    kind: TokenKind::Number(1),
                                    line: 2,
                                },
                                token::Token {
                                    kind: TokenKind::String("Alice".to_string()),
                                    line: 2,
                                },
                            ],
                        }),
                        kind: AstKind::Insert,
                    },
                    Statement {
                        select: Some(SelectStatement {
                            item: vec![
                                token::Token {
                                    kind: TokenKind::Identifier("id".to_string()),
                                    line: 3,
                                },
                                token::Token {
                                    kind: TokenKind::Identifier("name".to_string()),
                                    line: 3,
                                },
                            ],
                            from: token::Token {
                                kind: TokenKind::Identifier("users".to_string()),
                                line: 3,
                            },
                        }),
                        create: None,
                        insert: None,
                        kind: AstKind::Select,
                    },
                ],
            }),
        }];

        for test in tests.iter() {
            assert_eq!(parse(&test.input), test.expect);
        }
    }
}
