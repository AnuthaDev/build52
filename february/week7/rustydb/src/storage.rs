use std::collections::HashMap;

pub struct Row(Vec<String>);

impl Row {
    pub fn new(data: Vec<String>) -> Self {
        Self(data)
    }

    pub fn get_inner_vec(&self) -> &Vec<String> {
        &self.0
    }
}

pub struct Table {
    pub columns: Vec<String>,
    rows: Vec<Row>,
}

pub struct Database {
    tables: HashMap<String, Table>,
}

impl Table {
    pub fn insert(&mut self, row: Row) {
        if self.columns.len() == row.0.len() {
            self.rows.push(row);
        } else {
            panic!(
                "Incorrect number of values: expected {}, got {}",
                self.columns.len(),
                row.0.len()
            );
        }
    }

    pub fn select_all(&self) -> &Vec<Row> {
        &self.rows
    }

    /// Return rows where `column_name = value` (string comparison).
    /// Returns an empty vec if the column doesn't exist.
    pub fn select_where(&self, column_name: &str, value: &str) -> Vec<&Row> {
        let col_idx = self.columns.iter().position(|c| c == column_name);
        match col_idx {
            None => vec![],
            Some(idx) => self
                .rows
                .iter()
                .filter(|row| row.0.get(idx).map(|v| v == value).unwrap_or(false))
                .collect(),
        }
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, table_name: &str, columns: Vec<String>) {
        let table = Table {
            columns,
            rows: Vec::new(),
        };
        self.tables.insert(table_name.to_owned(), table);
    }

    pub fn insert_into_table(&mut self, table_name: &str, row: Row) {
        let table = self
            .tables
            .get_mut(table_name)
            .unwrap_or_else(|| panic!("Table '{}' not found", table_name));
        table.insert(row);
    }

    pub fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    /// Select all rows from a table.
    pub fn select_all(&self, table_name: &str) -> &Vec<Row> {
        let table = self
            .tables
            .get(table_name)
            .unwrap_or_else(|| panic!("Table '{}' not found", table_name));
        table.select_all()
    }

    /// Select rows matching `column = value`.
    pub fn select_where<'a>(&'a self, table_name: &str, column: &str, value: &str) -> Vec<&'a Row> {
        let table = self
            .tables
            .get(table_name)
            .unwrap_or_else(|| panic!("Table '{}' not found", table_name));
        table.select_where(column, value)
    }
}
