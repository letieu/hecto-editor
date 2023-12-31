use std::{fs, io};

use crate::row::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(file_path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(file_path)?;
        let rows = content.lines().map(Row::from).collect();

        Ok(Self {
            rows,
            file_name: Some(file_path.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
