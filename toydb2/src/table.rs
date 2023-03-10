use crate::ast::*;
use crate::token::TokenKind;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Tables {
    pub table: HashMap<String, Table>, // <table-name, &Table>
}

impl Tables {
    pub fn create_table(&mut self, cts: &CreateTableStatement) -> Result<(), String> {
        let name = match pop_string(&cts.name.kind) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let mut columns: HashMap<String, TokenKind> = HashMap::new();
        let mut col_names: Vec<String> = Vec::new();

        for cd in &cts.cols {
            let name = pop_string(&cd.name.kind)?;
            col_names.push(name.to_string());
            columns.insert(name, cd.data_type.kind.clone());
        }

        self.table.insert(
            name.to_string(),
            Table {
                name,
                columns,
                col_names,
                rows: Vec::new(),
            },
        );

        Ok(())
    }

    // Daibukitanai
    pub fn insert(&mut self, is: &InsertStatement) -> Result<(), String> {
        let mut column: HashMap<String, TokenKind> = HashMap::new();
        let table: &mut Table;

        match pop_string(&is.table.kind) {
            Ok(name) => match self.table.get_mut(&name) {
                Some(t) => table = t,
                None => return Err("error: invalid table name".to_string()),
            },
            Err(e) => return Err(e),
        }

        for (i, col) in table.col_names.iter().enumerate() {
            column.insert(col.to_string(), is.values[i].kind.clone());
        }

        table.rows.push(column);
        Ok(())
    }

    pub fn select(&mut self, ss: &SelectStatement) -> Result<Table, String> {
        let table: &mut Table;

        match pop_string(&ss.from.kind) {
            Ok(name) => match self.table.get_mut(&name) {
                Some(t) => table = t,
                None => return Err("error: invalid table name".to_string()),
            },
            Err(e) => return Err(e),
        }

        if pop_string(&ss.from.kind)? != table.name {
            return Err("error: invalid table name".to_string());
        }

        let mut columns: HashMap<String, TokenKind> = HashMap::new();
        let mut col_names = Vec::new();
        let mut rows = Vec::new();

        for item in &ss.item {
            let name = pop_string(&item.kind)?;
            col_names.push(name.to_string());
            columns.insert(name.to_string(), table.columns.get(&name).unwrap().clone());
        }

        for row in &table.rows {
            let mut hm = HashMap::new();
            for name in &col_names {
                let kind = row.get(name).unwrap().clone(); // toriaezu era- syori nashi
                hm.insert(name.to_string(), kind);
            }
            rows.push(hm);
        }

        Ok(Table {
            name: pop_string(&ss.from.kind)?,
            columns,
            col_names,
            rows,
        })
    }
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: HashMap<String, TokenKind>, // [INT|TEXT]
    pub col_names: Vec<String>,
    pub rows: Vec<HashMap<String, TokenKind>>,
}

fn pop_string(tk: &TokenKind) -> Result<String, String> {
    match tk {
        TokenKind::String(s) => Ok(s.to_string()),
        TokenKind::Identifier(i) => Ok(i.to_string()),
        _ => Err("create table error: failed to get table name.".to_string()),
    }
}
