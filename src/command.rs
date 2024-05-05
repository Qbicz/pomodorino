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
    Start,
    Stop,
    Manage,
    Help, // handled by top level main
}

impl Command {
    pub fn new(args: &[String]) -> Result<Self, CommandError> {
        if args.len() < 2 {
            // commands need type
            return Err(CommandError::ArgError);
        }

        let command_type = args[1].clone();
        match command_type.as_str() {
            "help" => Ok(Command::Help),
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let args = [
            String::from("target/debug/pomodorino"),
            String::from("add"),
            String::from("Task 1"),
        ];
        assert_eq!(Command::new(&args), Ok(Command::Add("Task 1".to_string())));
    }

    #[test]
    fn test_add_no_arg() {
        let args = [String::from("target/debug/pomodorino"), String::from("add")];
        assert_eq!(Command::new(&args), Err(CommandError::ArgError));
    }

    #[test]
    fn test_too_little_arg() {
        let args = [String::from("target/debug/pomodorino")];
        assert_eq!(Command::new(&args), Err(CommandError::ArgError));
    }

    #[test]
    fn test_rm() {
        let args = [String::from("pomodorino"), String::from("rm")];
        assert_eq!(Command::new(&args), Ok(Command::Remove));
    }

    #[test]
    fn test_not_supported() {
        let args = [String::from("pomodorino"), String::from("flytothemoon")];
        assert_eq!(Command::new(&args), Err(CommandError::NotSupported));
    }
}
