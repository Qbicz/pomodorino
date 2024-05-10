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
mod db;

use command::Command;
use db::{DatabaseBuilder, Db};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::{env, io};

fn main() {
    info!("Pomodorino start");
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();

    // db init
    let mut builder: DatabaseBuilder = DatabaseBuilder::new(); // TODO: confirm if this works with Tokio, otherwise move to Lazy static, and mutate it with unsafe
    let db = Db::new(&mut builder, String::from("db_pomodorino")).unwrap();

    match Command::new(&args) {
        Ok(Command::Help) => {
            display_help();
        }
        Ok(Command::Add(task_name)) => {
            db.add(task_name).unwrap();
            let tasks = db.read_all().unwrap();

            info!("db tasks: {:?}", tasks);
        }
        Ok(Command::Start) => {
            let tasks = db.read_in_state(String::from("todo")).unwrap();
            for (i, task) in tasks.iter().enumerate() {
                println!("{i}: {}", task.name)
            }
            info!("Choose a task to start: ");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_input_size) => match input.trim_end().parse::<usize>() {
                    Ok(task_num) => {
                        let task_to_start = tasks.get(task_num);
                        println!("Task {task_num} was chosen: {:?}", task_to_start.unwrap());
                    }
                    Err(e) => error!("Error parsing input {input} to number: {e}"),
                },
                Err(error) => println!("error: {error}"),
            }
        }
        Ok(command) => {
            info!(
                "Command not implemented yet: {:?}, args: {:?}",
                command, args
            );
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
