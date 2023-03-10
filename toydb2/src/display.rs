use crate::table;
use crate::token::TokenKind;

pub fn view(table: &table::Table) {
    // header
    let mut len = 0;
    for name in &table.col_names {
        len += name.len() + 3;
        print!("| {} ", name);
    }
    println!("|");
    println!("={:=<width$}", "=", width = len);

    // body
    for row in &table.rows {
        for name in &table.col_names {
            let col = row.get(name);
            match col {
                Some(TokenKind::Number(n)) => print!("| {} ", n),
                Some(TokenKind::String(s)) => print!("| {} ", s),
                _ => print!("| NONE "),
            }
        }
        println!("|");
    }
}

pub fn html(table: &table::Table) -> String {
    let mut ret = String::new();

    // header
    ret += r#"<table border="1"><tr>"#;
    for name in &table.col_names {
        ret += &format!("<th>{}</th>", name);
    }
    ret += "</tr>";

    // body
    for row in &table.rows {
        ret += "</tr>";
        for name in &table.col_names {
            let col = row.get(name);
            match col {
                Some(TokenKind::Number(n)) => ret += &format!("<td>{}</td>", n),
                Some(TokenKind::String(s)) => ret += &format!("<td>{}</td>", s),
                _ => print!("<td>NONE</td>"),
            }
        }
        ret += "</tr>";
    }
    ret += "</table>";

    ret
}
