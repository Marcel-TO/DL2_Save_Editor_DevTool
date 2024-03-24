use super::base_argument::Argument;

pub struct HelpArgument;

impl HelpArgument {
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