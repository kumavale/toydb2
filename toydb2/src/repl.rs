use crate::ast::AstKind;
use crate::display;
use crate::parser;
use crate::table;

use std::io::{stdout, Write};

pub fn run() {
    let prompt = ">> ";
    let mut tables: table::Tables = Default::default();

    println!("Welcome to toydb2");
    println!("Use `exit` to exit\n");

    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();

        let input = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).ok();
            s
        };

        match &*input {
            "\n" => continue,
            "exit\n" => break,
            _ => (),
        }

        let ast = match parser::parse(&input) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        for stmt in ast.statements {
            match stmt.kind {
                AstKind::Select => match tables.select(&stmt.select.unwrap()) {
                    Ok(virtual_table) => display::view(&virtual_table),
                    Err(e) => eprintln!("{}", e),
                },
                AstKind::Insert => match tables.insert(&stmt.insert.unwrap()) {
                    Ok(_) => println!("ok"),
                    Err(e) => eprintln!("{}", e),
                },
                AstKind::Create => match tables.create_table(&stmt.create.unwrap()) {
                    Ok(_) => println!("ok"),
                    Err(e) => eprintln!("{}", e),
                },
            }
        }
    }
}
