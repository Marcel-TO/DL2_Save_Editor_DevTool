use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub long_arg: String,
    pub short_arg: String,
    pub description: String,
    pub is_required: bool,
    pub type_of: String,
}

impl Argument {
    pub fn new(
        long_arg: &str, 
        short_arg: &str, 
        description: &str, 
        is_required: bool, 
        type_of: &str) -> Self {
        Argument {
            long_arg: long_arg.to_string(),
            short_arg: short_arg.to_string(),
            description: description.to_string(),
            is_required: is_required,
            type_of: type_of.to_string(),
        }
    }
}