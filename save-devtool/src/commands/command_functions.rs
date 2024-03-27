use crate::logger::ConsoleLogger;

/// Represents the functions that all supported commands need to implement.
pub trait CommandFunctions {
    fn log_help_documentation(&self, logger: &mut ConsoleLogger);
    fn execute_command(&mut self, args: Vec<&str>, logger: &mut ConsoleLogger);
    fn validate_arguments(&mut self, args: Vec<&str>, logger: &mut ConsoleLogger);
}