//! Represents the analyse command.
//! 
//! It uses the crate `file_analyser` to analyse a save path.

use std::error::Error;
use std::path::Path;
use term;
use serde::{Serialize, Deserialize};

use crate::arguments::base_argument::Argument;
use crate::arguments::*;
use crate::commands::command_functions::CommandFunctions;
use crate::commands::*;
use crate::logger::{ConsoleLogger, LoggerFunctions};
use crate::save_logic::file_analyser::{get_contents_from_file, load_save_file, load_save_file_pc};
use crate::save_logic::struct_data::SaveFile;

// Define global result definition for easier readability.
type Result<T> = std::result::Result<T,Box<dyn Error>>;

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
    pub arguments: Vec<base_argument::Argument>,
    selected_path: String,
    is_debugging: bool,
    is_decompressing: bool,
}

impl CommandFunctions for AnalyseSaveCommand {
    fn log_help_documentation(&self, logger: &mut ConsoleLogger) {
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

    fn execute_command(&self, args: Vec<&str>, logger: &mut ConsoleLogger) {
        let mut cmd = self.clone();

        match args.as_slice() {
            [a, b, rest @ ..] => {
                match (*a, *b) {
                    (arg, value) => {
                        if arg == path_argument::PathArgument::new().long_arg || arg == path_argument::PathArgument::new().short_arg {
                            if Path::new(&value.replace('"', "")).exists() {
                                cmd.selected_path = value.replace('"', "");
                                logger.log_message(&format!("Path is set to {}.", cmd.selected_path), Vec::new());
                            } else {
                                logger.log_error(
                                    format!(
                                        "Invalid value for [{} | {}] argument. Please provide a valid path.",
                                        path_argument::PathArgument::new().long_arg,
                                        path_argument::PathArgument::new().short_arg
                                    ).as_str()
                                );
                            }
                        } else if arg == debug_argument::DebugArgument::new().long_arg || arg == debug_argument::DebugArgument::new().short_arg{
                            match value.parse::<bool>() {
                                Ok(value) => {
                                    cmd.is_debugging = value;
                                    if value {
                                        logger.log_message("Debugging is enabled.", Vec::new());
                                    } else {
                                        logger.log_message("Debugging is disabled.", Vec::new());
                                    }
                                },
                                Err(_) => {
                                    logger.log_error(
                                        format!(
                                            "Invalid value for [{} | {}] argument. Please provide a boolean value.",
                                            debug_argument::DebugArgument::new().long_arg,
                                            debug_argument::DebugArgument::new().short_arg
                                        ).as_str()
                                    );
                                }
                            }
                        } else if arg == decompress_argument::DecompressArgument::new().long_arg || arg == decompress_argument::DecompressArgument::new().short_arg {
                            match value.parse::<bool>() {
                                Ok(value) => {
                                    cmd.is_decompressing = value;
                                    if value {
                                        logger.log_message("Decompression is enabled.", Vec::new());
                                    } else {
                                        logger.log_message("Decompression is disabled.", Vec::new());
                                    }
                                },
                                Err(_) => {
                                    logger.log_error(
                                        format!(
                                            "Invalid value for [{} | {}] argument. Please provide a boolean value.",
                                            decompress_argument::DecompressArgument::new().long_arg,
                                            decompress_argument::DecompressArgument::new().short_arg
                                        ).as_str()
                                    );
                                }
                            }
                        }
                    },
                    _ => {
                        let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                        logger.log_error(help_message.as_str());
                    }
                }

                // Redo the match with the rest
                if rest.len() > 0 {
                    cmd.execute_command(rest.to_vec(), logger);
                }
            },
            [a] => {
                let help = help_argument::HelpArgument::new();
                match *a {
                    ref arg if arg == &help.long_arg => logger.log_message("Help is on the way.", Vec::new()),
                    _ => {
                        let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                        logger.log_error(help_message.as_str());
                    }
                }
            },
            _ => {
                let help_message = format!("There is no argument with the following name. Please enter {} for help or press exit to exit.", help_command::HelpCommand::new().command);
                logger.log_error(help_message.as_str());
            }
        }

        analyse_save(cmd, logger)
    }
}

impl AnalyseSaveCommand {
    pub fn new() -> Self {
        AnalyseSaveCommand {
            name: "Analyse Save",
            command: "analyse-save",
            description: "Scrapes the .sav and validates the data.",
            arguments: vec![
                path_argument::PathArgument::new(),
                debug_argument::DebugArgument::new(),
                decompress_argument::DecompressArgument::new()
            ],
            selected_path: String::new(),
            is_debugging: false,
            is_decompressing: false,
        }
    }
}

fn analyse_save(command: AnalyseSaveCommand, logger: &mut ConsoleLogger) {
    let file_content: Vec<u8> = get_contents_from_file(&command.selected_path).unwrap();
    let save_file: Result<SaveFile>;
    
    if command.is_decompressing {
        save_file = load_save_file_pc(&command.selected_path, file_content, logger, command.is_debugging);
    } else {
        save_file = load_save_file(&command.selected_path, file_content, logger, command.is_debugging);
    }
    
    logger.log_break();

    match save_file {
        Ok(save) => {
            logger.log_message("The save was loaded successfully!", vec![term::Attr::ForegroundColor(term::color::GREEN)]);
            logger.log_message(format!("Base Skills: {}", save.skills.base_skills.len()).as_str(), Vec::new());
            logger.log_message(format!("Legend Skills: {}", save.skills.legend_skills.len()).as_str(), Vec::new());
            logger.log_message(format!("Unlockables: {}", save.unlockable_items.len()).as_str(), Vec::new());
            for tab in save.items {
                logger.log_message(format!("{}: {}", tab.name, tab.inventory_items.len()).as_str(), Vec::new());
            }

        },
        Err(err) => {
            logger.log_error(&err.to_string());
            logger.log_error("Please use the debug function for more detailed steps.");
        }
    }
}