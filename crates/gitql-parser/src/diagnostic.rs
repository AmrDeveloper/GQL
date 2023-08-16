use crate::tokenizer::Location;

pub struct GQLError {
    pub message: String,
    pub location: Location,
}
