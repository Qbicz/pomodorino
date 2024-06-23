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

use clap::{arg, command};
use command::Command;
use db::{DatabaseBuilder, Db};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::{env, io};

// timers.rs
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let matches =
        command!() // requires `cargo` feature
            .arg(arg!([command] "Command").value_parser(["add", "rm", "start", "stop"]))
            .arg(
                arg!(
                    -n --name <TASK_NAME> "Task name, needed for `add` command"
                )
                .required(false),
            )
            .arg(
                arg!(
                    -d --database <FILE> "Sets a custom database file (useful for testing)"
                )
                .default_value("db_pomodorino")
                // Clap doesn't have syntax yet for optional options, so manually calling `required`
                .required(false),
            )
            .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(command_arg) = matches.get_one::<String>("command") {
        println!("Value for command: {command_arg}");
    }

    if let Some(task_name) = matches.get_one::<String>("name") {
        println!("Value for task name: {}", task_name);
    }

    if let Some(database) = matches.get_one::<String>("database") {
        println!("Value for database path: {}", database);
    }

    // db init - injecting db path is possible from CLI for integration testing
    let mut builder: DatabaseBuilder = DatabaseBuilder::new(); // TODO: confirm if this works with Tokio, otherwise move to Lazy static, and mutate it with unsafe
    let db = Db::new(&mut builder, matches.get_one::<String>("database")).unwrap();

    match Command::new(
        matches.get_one::<String>("command"),
        matches.get_one::<String>("name"),
    ) {
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
            println!("\nChoose a task to start 🍅:");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_input_size) => match input.trim_end().parse::<usize>() {
                    Ok(task_num) => {
                        let task_to_start = tasks.get(task_num);

                        match task_to_start {
                            Some(task) => {
                                info!("Task {task_num} was chosen: {:?}", task);

                                // TODO: move to a timer.rs module
                                // Start timer, track and display time left every second

                                // TODO: move to config module
                                let mut seconds_left = 25 * 60;

                                while seconds_left > 0 {
                                    sleep(Duration::from_secs(1)).await;
                                    seconds_left -= 1;
                                    // TODO: update timer view
                                    println!("{}m{} left", seconds_left / 60, seconds_left % 60);
                                }

                                // When timer finishes, mark as done. TODO: pass as function to ticking_timer()
                                if let Err(e) = db.set_done(&task.name) {
                                    error!("Failed to set task to done: {e}");
                                }

                                // TODO: play sound

                                println!("Start a break!");

                                let mut break_seconds_left = 5 * 60;

                                while break_seconds_left > 0 {
                                    sleep(Duration::from_secs(1)).await;
                                    break_seconds_left -= 1;
                                    // TODO: update timer view
                                    println!(
                                        "break: {}m{} left",
                                        break_seconds_left / 60,
                                        break_seconds_left % 60
                                    );
                                }

                                println!("Finished break. Pick a new task.");
                                // TODO: play sound
                            }
                            None => error!("No task in database with index {task_num}"),
                        }
                    }
                    Err(e) => error!("Error parsing input {input} to number: {e}"),
                },
                Err(error) => error!("Failed to read input: {error}"),
            }
        }
        Ok(command) => {
            info!(
                "Command not implemented yet: {:?}, arg: {:?}",
                command,
                matches.get_one::<String>("command")
            );
        }
        Err(e) => {
            error!(
                "error when creating a Command: {:?}. See `pomodorino --help`",
                e
            );
        }
    }
}
