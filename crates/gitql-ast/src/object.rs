use std::error::Error;

use csv::Writer;

use crate::value::Value;

/// In memory representation of the list of [`Value`] in one Row
#[derive(Default)]
pub struct Row {
    pub values: Vec<Value>,
}

/// In memory representation of the Rows of one [`Group`]
#[derive(Default)]
pub struct Group {
    pub rows: Vec<Row>,
}

impl Group {
    /// Returns true of this group has no rows
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns the number of rows in this group
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// In memory representation of the GitQL Object which has titles and groups
#[derive(Default)]
pub struct GitQLObject {
    pub titles: Vec<String>,
    pub groups: Vec<Group>,
}

impl GitQLObject {
    /// Flat the list of current groups into one main group
    pub fn flat(&mut self) {
        let mut rows: Vec<Row> = vec![];
        for group in &mut self.groups {
            rows.append(&mut group.rows);
        }

        self.groups.clear();
        self.groups.push(Group { rows })
    }

    /// Returns true of there is no groups
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns the number of groups in this Object
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Export the GitQLObject as CSV String
    pub fn as_csv(&self) -> Result<String, Box<dyn Error>> {
        let mut writer = Writer::from_writer(vec![]);
        writer.write_record(self.titles.clone())?;
        let row_len = self.titles.len();
        if let Some(group) = self.groups.get(0) {
            for row in &group.rows {
                let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                for value in &row.values {
                    values_row.push(value.to_string());
                }
                writer.write_record(values_row)?;
            }
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }
}
