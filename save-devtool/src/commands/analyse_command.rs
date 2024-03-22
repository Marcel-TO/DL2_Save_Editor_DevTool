//! Represents the analyse command.
//! 
//! It uses the crate `file_analyser` to analyse a save path.

use std::path::Path;
use term;
use serde::{Serialize, Deserialize};

use crate::commands::arguments::Argument;
use crate::commands::command_functions::CommandFunctions;
use crate::commands::help_command;
use crate::logger::{ConsoleLogger, LoggerFunctions};
use crate::save_logic::file_analyser::{get_contents_from_file, load_save_file, load_save_file_pc};

/// Defines the analyse command struct.
/// 
/// ### Parameters
/// - `name`: The name of the command.
/// - `command`: The command initializer.
/// - `description`: The description of the command.
/// - `arguments`: The list of all the arguments for the command.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyseSaveCommand {
    pub name: &'static str,
    pub command: &'static str,
    pub description: &'static str,
    pub arguments: Vec<Argument>,
    selected_path: String,
    debug: bool,
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

    fn execute_command(&self, args: Vec<&str>) {
        // Initializes the logger.
        let logger: ConsoleLogger = ConsoleLogger::new();

        match args.as_slice() {
            [a, b, rest @ ..] => {
                match (*a, *b) {
                    ("--path", path) => {
                        if Path::new(&path.replace('"', "")).exists() {
                            logger.log_message("Path is set.", Vec::new());
                        } else {
                            logger.log_message("Invalid value for --path argument. Please provide a valid path.", vec![term::Attr::ForegroundColor(term::color::RED)]);
                        }
                    },
                    ("--debug", debug) => {
                        match debug.parse::<bool>() {
                            Ok(value) => {
                                if value {
                                    logger.log_message("Debugging is enabled.", Vec::new());
                                } else {
                                    logger.log_message("Debugging is disabled.", Vec::new());
                                }
                            },
                            Err(_) => {
                                logger.log_message("Invalid value for --debug argument. Please provide a boolean value.", vec![term::Attr::ForegroundColor(term::color::RED)]);
                            }
                        }
                    },
                    _ => {
                        let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                        logger.log_message(help_message.as_str(), vec![term::Attr::ForegroundColor(term::color::RED)]);
                    }
                }

                // Redo the match with the rest
                if rest.len() > 0 {
                    self.execute_command(rest.to_vec());
                }
            },
            [a] => {
                match *a {
                    "--help" => logger.log_message("Help is on the way.", Vec::new()),
                    _ => {
                        let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                        logger.log_message(help_message.as_str(), vec![term::Attr::ForegroundColor(term::color::RED)]);
                    }
                }
            },
            _ => {
                let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                logger.log_message(help_message.as_str(), vec![term::Attr::ForegroundColor(term::color::RED)]);
            }
        }

        analyse_save(self.clone());
    }
}

impl AnalyseSaveCommand {
    pub fn new() -> Self {
        AnalyseSaveCommand {
            name: "Analyse Save",
            command: "analyse-save",
            description: "Scrapes the .sav and validates the data.",
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
            selected_path: String::new(),
            debug: false,
        }
    }
}

fn analyse_save(mut command: AnalyseSaveCommand) {
    command.selected_path = "HEllo".to_string();
}