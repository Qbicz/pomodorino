use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    ArgError,
    NotSupported,
}

impl std::error::Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::ArgError => write!(f, "Arguments error"),
            CommandError::NotSupported => write!(f, "Command not supported"),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Add(String),
    Remove,
    Start,
    Stop,
    Manage,
}

impl Command {
    pub fn new(args: &[String]) -> Result<Self, CommandError> {
        let command_type = args[1].clone();

        if args.len() < 2 {
            // commands need type
            return Err(CommandError::ArgError);
        }

        match command_type.as_str() {
            "add" => {
                // add command also needs payload
                if args.len() < 3 {
                    return Err(CommandError::ArgError);
                }
                Ok(Command::Add(args[2].clone()))
            }
            "rm" => Ok(Command::Remove),
            "start" => Ok(Command::Start),
            "stop" => Ok(Command::Stop),
            "manage" => Ok(Command::Manage),
            _ => Err(CommandError::NotSupported),
        }
    }
}
