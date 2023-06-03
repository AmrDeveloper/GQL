use crate::tokenizer::Location;

const PORPOT_LENGTH: usize = 6;

pub struct GQLError {
    pub message: String,
    pub location: Location,
}

pub fn report_gql_error(error: GQLError) {
    println!("{}^", "-".repeat(PORPOT_LENGTH + error.location.start));
    println!(
        "Error({}:{}) -> {}",
        error.location.start, error.location.end, error.message
    );
}
