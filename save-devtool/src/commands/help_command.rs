//! Represents the help command.
//! 
//! Gives an example of all supported commadns and helps the user understand the devtool.

use serde::{Serialize, Deserialize};
use crate::arguments::base_argument::Argument;
use crate::commands::command_functions::CommandFunctions;
use crate::logger::{ConsoleLogger, LoggerFunctions};
use crate::commands::supported_commands::SupportedCommands;
use term;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HelpCommand {
    pub name: &'static str,
    pub command: &'static str,
    pub description: &'static str,
    pub arguments:  Vec<Argument>,
}

impl CommandFunctions for HelpCommand {
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
        self.log_help_documentation(logger);
    }
}

impl HelpCommand {
    pub fn new() -> Self {
        HelpCommand {
            name: "Help",
            command: "help",
            description: "Gives an overview of all commands supported by the editor",
            arguments: Vec::new()
        }
    }

    pub fn execute_help_command(&self, args: Vec<&str>, supported: Vec<SupportedCommands>, logger: &mut ConsoleLogger) {
        logger.log_message(
            "All supported commands:", 
            vec![term::Attr::Bold]
        );
        logger.log_break();
        
        for command in supported {
            command.execute_help_documentation(logger);
        }
    }
}
