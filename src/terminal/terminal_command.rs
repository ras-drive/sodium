use std::{fmt::Display, process::Child};

///
pub struct TerminalCommand {
    /// Name of the program
    pub name: String,
    /// String supplied to constructor
    pub command_string: String,
    /// Args the command is to be run with
    pub args: String,
    /// A child process that is populated when the task is spawned
    child: Option<Box<Child>>,
}

impl TerminalCommand {
    /// creates a command struct
    pub fn new<T: Display>(command_string: T) -> Result<Self, Box<dyn std::error::Error>> {
        let binding = command_string.to_string();
        let command = binding.as_str();
        if command_string.to_string().trim().contains(' ') {
            let (name, arg_str) = command.split_once(' ').unwrap();

            Ok(TerminalCommand {
                name: name.to_string(),
                command_string: command.to_string(),
                args: arg_str.to_string(),
                child: None,
                // child: Box::new(Command::new(name))
            })
        } else {
            Ok(TerminalCommand {
                name: command_string.to_string().trim().to_string(),
                command_string: command_string.to_string(),
                args: String::new(),
                child: None,
                // child: Box::new(Command::new(command_string.to_string().trim())),
            })
        }
    }
    /// command to set the structs child process after it has been spawned
    #[allow(dead_code)]
    fn set_child(&mut self, child: Child) -> Result<(), Box<dyn std::error::Error>> {
        if self.child.is_none() {
            self.child = Some(Box::new(child));
            Ok(())
        } else {
            Err(format!(
                "error: command {} already has a child process set",
                self.command_string
            )
            .into())
        }
    }
}
