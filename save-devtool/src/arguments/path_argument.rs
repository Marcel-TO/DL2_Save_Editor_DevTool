use super::base_argument::Argument;

pub struct PathArgument;

impl PathArgument {
    pub fn new() -> Argument {
        Argument::new(
            "--path",
            "-p",
            "The full path of the save you want to analyse.",
            true,
            "String",
        )
    }
}