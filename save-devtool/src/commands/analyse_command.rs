use serde::{Serialize, Deserialize};
use crate::commands::arguments::Argument;
use crate::commands::command_functions::CommandFunctions;
use crate::logger::{ConsoleLogger, LoggerFunctions};
use term;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyseSaveCommand {
    pub name: String,
    pub command: String,
    pub description: String,
    pub arguments:  Vec<Argument>,
}

impl CommandFunctions for AnalyseSaveCommand {
    fn log_help_documentation(&self) {
        let logger = ConsoleLogger::new();
        logger.log_message(&format!("Name: {:?} - {:?}", self.name, self.command), 
            vec![
                term::Attr::ForegroundColor(term::color::GREEN), 
                term::Attr::Bold
            ]);
            logger.log_message(&format!("Description: {}", self.description), Vec::new());
            
        logger.log_break();
        logger.log_message("Arguments:", vec![term::Attr::Bold]);
        logger.log_message(&format!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "Long Arg", "Short Args", "Type", "Description"
        ), Vec::new());
        for arg in &self.arguments {
            if arg.is_required {
                logger.log_message(
                    &format!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", arg.long_arg, arg.short_arg, arg.type_of, arg.description), 
                    vec![term::Attr::ForegroundColor(term::color::YELLOW)]
                );
            } else {
                logger.log_message(
                    &format!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", arg.long_arg, arg.short_arg, arg.type_of, arg.description), 
                    Vec::new()
                );
            }

        }

        logger.log_break();
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
