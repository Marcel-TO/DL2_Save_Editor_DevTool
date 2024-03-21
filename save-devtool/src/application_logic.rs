//! Represents the main logic of the devtool.
//! 
//! The user inputs are getting compared with all supported commands.
//! After a match, the command will get executed, otherwise a help message will be displayed.
//! For help press `help`
//! To quit the application, just press `quit`.

use crate::logger::{ConsoleLogger, LoggerFunctions};
use crate::commands::*;

use self::command_functions::CommandFunctions;

#[allow(dead_code)]
/// Represents the enum of all supported commands.
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

/// Represents the main function of the application logic.
/// 
/// It waits for user input and executes a matching command.
pub fn main() {
    // Initializes the logger.
    let logger: ConsoleLogger = ConsoleLogger::new();

    // Logs the title page for the startup.
    logger.log_title_page();
    logger.log_break();

    loop {
        // Awaits user input.
        let user_input: String = logger.get_user_input();
        // Splits the user input into arguments.
        let args: Vec<&str> = user_input.split(' ').collect();
    
        // Gets first argument to match with supported commands.
        if let Some(command) = args.first() {
            // Indicates whether the user entered a supported command.
            let mut is_supported: bool = false;

            // Iterates through supported commands
            for supported_command in SupportedCommands::values() {
                // Checks if the first argument matches and executes.
                if *command == supported_command.command_name() {
                    supported_command.execute_command(args[1..].to_vec());
                    is_supported = true;
                    break;
                } 
            }
    
            // If user entered not a supported command.
            if !is_supported {
                // Checks if the user wants to quit.
                if *command == "quit" {
                    break;
                }
                
                // Prints the help message.
                let help_command = help_command::HelpCommand::new();
                let help_message = format!("There is no command with the following name. Please enter {} for help or press exit to exit.", help_command.command);
                logger.log_message(help_message.as_str(), Vec::new());
            }
        }
    }    
}