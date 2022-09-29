use std::{collections::VecDeque, fmt::Display, process::Command};

use super::terminal_command::TerminalCommand;

/// A terminal object that is spawned on prompt call
#[derive(Default)]
pub struct Terminal {
    /// Contains a session output history
    pub output: VecDeque<String>,
}

impl Terminal {
    /// Constructs a new terminal
    pub fn new() -> Self {
        Terminal {
            output: VecDeque::new(),
        }
    }
    /// Terminal function to add output to session history
    #[allow(dead_code)]
    fn add_output<T: Display>(&mut self, data: T) {
        for i in data.to_string().lines() {
            self.output.push_back(i.to_string());
        }
    }
    /// code to spawn a new command in the terminal
    pub fn spawn_command(
        &mut self,
        command: &mut TerminalCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let child = Command::new(&command.name).arg(&command.args).spawn(); // .wait_with_output().unwrap();// .spawn().expect("error while spawning child!");
        let child = match child {
            Ok(ch) => ch,
            Err(e) => return Err(format!("{}", e).into()),
        };
        let output = child.wait_with_output().unwrap();
        for i in String::from_utf8_lossy(&output.stdout).into_owned().lines() {
            self.add_output(i);
        }
        Ok(())
    }
}
