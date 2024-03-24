use super::base_argument::Argument;

pub struct DebugArgument;

impl DebugArgument {
    pub fn new() -> Argument {
        Argument::new(
            "--debug",
            "-d",
            "Indicates whether each step will be manually continued by pressing enter or not.",
            false,
            "Boolean",
        )
    }
}