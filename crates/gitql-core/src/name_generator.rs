use std::sync::Mutex;

static NAME_GENERATOR_PREFIX: &str = "column";

/// Generate Unique name for columns each time you call this funcion
pub fn generate_column_name() -> String {
    static GENERATOR_NUMBER: Mutex<usize> = Mutex::new(0);
    let mut count = GENERATOR_NUMBER.lock().unwrap();
    *count += 1;
    format!("{}_{}", NAME_GENERATOR_PREFIX, count)
}
