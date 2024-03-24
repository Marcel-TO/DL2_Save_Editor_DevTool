use self::command_functions::CommandFunctions;
use crate::commands::*;
use crate::logger::{ConsoleLogger, LoggerFunctions};

/// Represents the enum of all supported commands.
#[allow(dead_code)]
pub enum SupportedCommands {
    AnalyseSaveCommand,
    HelpCommand,
}

impl SupportedCommands {
    // Implements a method to get the command initializer for each variant.
    pub fn command_name(&self) -> &'static str {
        match self {
            SupportedCommands::HelpCommand => help_command::HelpCommand::new().command,
            SupportedCommands::AnalyseSaveCommand => analyse_command::AnalyseSaveCommand::new().command,
        }
    }

    // Implements a method to execute supported command.
    pub fn execute_command(&self, args: Vec<&str>) {
        match self {
            SupportedCommands::HelpCommand => help_command::HelpCommand::new().execute_help_command(args, SupportedCommands::values()),
            SupportedCommands::AnalyseSaveCommand => analyse_command::AnalyseSaveCommand::new().execute_command(args),
        }
    }

    // Implements a method to execute supported command.
    pub fn execute_help_documentation(&self) {
        match self {
            SupportedCommands::HelpCommand => help_command::HelpCommand::new().log_help_documentation(),
            SupportedCommands::AnalyseSaveCommand => analyse_command::AnalyseSaveCommand::new().log_help_documentation(),
        }
    }

    // Implements a method to get all supported command variants.
    pub fn values() -> Vec<SupportedCommands> {
        vec![
            SupportedCommands::AnalyseSaveCommand,
            SupportedCommands::HelpCommand
        ]
    }
}