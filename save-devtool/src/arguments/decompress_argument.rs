use super::base_argument::Argument;

pub struct DecompressArgument;

impl DecompressArgument {
    pub fn new() -> Argument {
        Argument::new(
            "--decompress",
            "-dc",
            "Indicates whether the save file is compressed for PC or not.",
            false,
            "Boolean",
        )
    }
}