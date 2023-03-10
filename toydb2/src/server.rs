use crate::ast::AstKind;
use crate::display;
use crate::parser;
use crate::table::Tables;

use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn run() {
    let listener = TcpListener::bind("localhost:5432").unwrap();
    let mut tables: Tables = Default::default();

    println!("Server listening on port 5432");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut tables);
    }

    drop(listener);
}

fn handle_connection(mut stream: TcpStream, tables: &mut Tables) {
    let mut data = [0; 256];

    while match stream.read(&mut data) {
        Ok(size) => {
            if size < 2 {
                return;
            }

            let input = String::from_utf8_lossy(&data[0..size]);

            let ast = match parser::parse(&input) {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let mut ret = String::new();

            for stmt in ast.statements {
                match stmt.kind {
                    AstKind::Select => match tables.select(&stmt.select.unwrap()) {
                        Ok(virtual_table) => ret += &display::html(&virtual_table),
                        Err(e) => ret += &e.to_string(),
                    },
                    AstKind::Insert => match tables.insert(&stmt.insert.unwrap()) {
                        Ok(_) => ret += "ok<br />",
                        Err(e) => ret += &e.to_string(),
                    },
                    AstKind::Create => match tables.create_table(&stmt.create.unwrap()) {
                        Ok(_) => ret += "ok<br />",
                        Err(e) => ret += &e.to_string(),
                    },
                }
            }

            stream.write_all(ret.as_bytes()).unwrap();

            true
        }
        Err(_) => {
            eprintln!(
                "Server error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
