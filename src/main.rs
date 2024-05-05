// Plan: implement basic function as quickly as possible, then refactor, move to separate files.
// 1. CLI accepts commands: DONE
// - add
// - rm
// - start
// - stop
// - manage -> list -> done/undo
// 2. Have tests for the command creation DONE
// 3, Adding them to db, iterate with tests for adding to db
// 4. Timer - iterate with tests for timers and moving to done
// 5. Improve displaying

mod command;

use command::Command;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::env;

fn main() {
    info!("Pomodorino start");
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    match Command::new(&args) {
        Ok(Command::Help) => {
            display_help();
        }
        Ok(command) => {
            info!("Command: {:?}, args: {:?}", command, args);
            // Add to DB
        }
        Err(e) => {
            error!("error when creating a Command: {:?}", e);
        }
    }
}

fn display_help() {
    println!("Usage: pomodorino <command> [task string]");
    println!("add command requires payload, others don't");
}
