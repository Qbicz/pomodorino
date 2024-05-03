// Plan: implement basic function as quickly as possible, then refactor, move to separate files.
// 1. CLI accepts commands:
// - add
// - rm
// - start
// - stop
// - manage -> list -> done/undo
// 2. Have tests for the command creation
// 3, Adding them to db, iterate with tests for adding to db
// 4. Timer - iterate with tests for timers and moving to done
// 5. Improve displaying

use log::{error, info};
use simple_logger::SimpleLogger;
use std::env;
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
enum Command {
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

fn main() {
    info!("Pomodorino start");
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    match Command::new(&args) {
        Ok(command) => {
            info!("Command: {:?}, args: {:?}", command, args);
            // Add to DB
        }
        Err(e) => {
            error!("error when creating a Command: {:?}", e);
        }
    }
}
