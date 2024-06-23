use std::fmt;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String),
    Remove,
    Show,
    Start,
    Stop,
    Manage,
    Help, // handled by top level main
}

impl Command {
    pub fn new(command: Option<&String>, payload: Option<&String>) -> Result<Self, CommandError> {
        let command = command.ok_or(CommandError::ArgError)?;

        let command_type = command.clone();
        match command_type.as_str() {
            "help" => Ok(Command::Help),
            "add" => {
                // add command also needs payload
                if let Some(payload) = payload {
                    Ok(Command::Add(payload.clone()))
                } else {
                    Err(CommandError::ArgError)
                }
            }
            "rm" => Ok(Command::Remove),
            "show" => Ok(Command::Show),
            "start" => Ok(Command::Start),
            "stop" => Ok(Command::Stop),
            "manage" => Ok(Command::Manage),
            _ => Err(CommandError::NotSupported),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Command::new(Some(&String::from("add")), Some(&String::from("Task 1"))),
            Ok(Command::Add("Task 1".to_string()))
        );
    }

    #[test]
    fn test_add_no_arg() {
        assert_eq!(
            Command::new(Some(&String::from("add")), None),
            Err(CommandError::ArgError)
        );
    }

    #[test]
    fn test_too_little_arg() {
        assert_eq!(Command::new(None, None), Err(CommandError::ArgError));
    }

    #[test]
    fn test_rm() {
        assert_eq!(
            Command::new(Some(&String::from("rm")), None),
            Ok(Command::Remove)
        );
    }

    #[test]
    fn test_not_supported() {
        assert_eq!(
            Command::new(Some(&String::from("flytothemoon")), None),
            Err(CommandError::NotSupported)
        );
    }
}
