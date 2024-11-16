static COLUMN_NAME_PREFIX: &str = "column_";
static HIDDEN_NAME_PREFIX: &str = "_@temp_";

/// Component to generate name for visible and hidden columns with number as prefix started from 0
#[derive(Default)]
pub struct NameGenerator {
    column_name_number: usize,
    temp_name_number: usize,
}

impl NameGenerator {
    /// Generate name for visible column
    pub fn generate_column_name(&mut self) -> String {
        let name = format!("{}{}", COLUMN_NAME_PREFIX, self.column_name_number);
        self.column_name_number += 1;
        name
    }

    pub fn generate_temp_name(&mut self) -> String {
        let name = format!("{}{}", HIDDEN_NAME_PREFIX, self.temp_name_number);
        self.temp_name_number += 1;
        name
    }

    /// Reset the name counter to start from 0 in new session
    pub fn reset_numbers(&mut self) {
        self.column_name_number = 0;
        self.temp_name_number = 0;
    }
}
