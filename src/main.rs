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

// DB start
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

// #[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
// //#[native_db]
// enum TaskState {
//     Todo,
//     Done,
// }

// impl InnerKeyValue for TaskState {
//     fn database_inner_key_value(&self) -> db_type::DatabaseInnerKeyValue {
//         db_type::DatabaseInnerKeyValue::new(vec![*self as u8])
//     }
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Task {
    #[primary_key]
    name: String,
    #[secondary_key]
    state: String, // using String for now, switch to enum when native_db issue resolved
}

impl Task {
    pub fn new(name: String) -> Self {
        Task {
            name,
            state: String::from("todo"), // TaskState::Todo,
        }
    }
}

pub fn db_init(builder: &mut DatabaseBuilder) -> Result<Database, db_type::Error> {
    // Initialize the model
    builder.define::<Task>()?;
    let db = builder.create_in_memory()?;

    Ok(db)
}
pub fn db_add(db: &Database, name: String) -> Result<(), db_type::Error> {
    info!("db_add: {name}");

    let rw = db.rw_transaction().unwrap();
    rw.insert(Task::new(name))?;
    rw.commit()?;
    info!("transaction committed!");
    Ok(())
}
pub fn db_read_all(db: &Database) -> Result<Vec<Task>, db_type::Error> {
    // Read all tasks
    // Open a read-only transaction
    let r = db.r_transaction()?;
    // Iterate items with name starting with "red"
    let values: Vec<Task> = r.scan().primary()?.all().collect();
    Ok(values)
}
pub fn db_rm() {}
pub fn db_set_done() {}
pub fn db_set_todo() {}
// DB end

fn main() {
    info!("Pomodorino start");
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();

    // db init
    let mut builder = DatabaseBuilder::new();
    let db = db_init(&mut builder).unwrap();

    match Command::new(&args) {
        Ok(Command::Help) => {
            display_help();
        }
        Ok(Command::Add(task_name)) => {
            db_add(&db, task_name).unwrap();
            let tasks = db_read_all(&db).unwrap();

            info!("db tasks: {:?}", tasks);
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
