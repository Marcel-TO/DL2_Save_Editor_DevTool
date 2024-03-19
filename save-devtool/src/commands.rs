use serde::{Serialize, Deserialize};
use crate::arguments::Argument;

pub trait CommandFunctions {
    fn log_help_documentation(&self);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyseSaveCommand {
    pub name: String,
    pub command: String,
    pub description: String,
    pub arguments:  Vec<Argument>,
}

impl CommandFunctions for AnalyseSaveCommand {
    fn log_help_documentation(&self) {
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("Name: {:?} - {:?}", self.name, self.command);
        terminal.reset().unwrap();
        println!("Description: {}", self.description);
        println!("");

        println!("Arguments: ");
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "Long Arg", "Short Args", "Type", "Description"
        );
        for arg in &self.arguments {
            if arg.is_required {
                terminal.fg(term::color::BRIGHT_CYAN).unwrap();
            } else {
                terminal.reset().unwrap();
            }
            println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", arg.long_arg, arg.short_arg, arg.type_of, arg.description);
        }
    }
}

impl AnalyseSaveCommand {
    pub fn new() -> Self {
        AnalyseSaveCommand {
            name: "Analyse Save".to_string(),
            command: "analyse-save".to_string(),
            description: "Scrapes the .sav and validates the data.".to_string(),
            arguments: vec![
                Argument::new(
                "--path",
                "-p",
                "The path to the .sav file.",
                true,
                "String",
                ),
                Argument::new(
                    "--debug",
                    "-d",
                    "Indicates whether each step will be manually continued by pressing enter or not.",
                    false,
                    "Boolean",
                    ),
            ],
        }
    }
}
