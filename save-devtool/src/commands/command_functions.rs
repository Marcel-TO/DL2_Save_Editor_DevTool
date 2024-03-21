/// Represents the functions that all supported commands need to implement.
pub trait CommandFunctions {
    fn log_help_documentation(&self);
    fn execute_command(&self, args: Vec<&str>);
}