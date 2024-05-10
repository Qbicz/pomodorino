// DB start
use log::info;
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

// Re-export DatabaseBuilder as for now it needs to live longer than the database.
pub use native_db::DatabaseBuilder;

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
    pub name: String,
    #[secondary_key]
    pub state: String, // using String for now, switch to enum when native_db issue resolved
}

impl Task {
    pub fn new(name: String) -> Self {
        Task {
            name,
            state: String::from("todo"), // TaskState::Todo,
        }
    }
}

pub struct Db<'a> {
    db: Database<'a>,
}

impl<'a> Db<'a> {
    pub fn new(builder: &'a mut DatabaseBuilder, db_path: String) -> Result<Self, db_type::Error> {
        // Initialize the model
        builder.define::<Task>()?;

        // Create a database
        let db = builder.create(db_path)?;

        Ok(Self { db })
    }
    pub fn add(&self, name: String) -> Result<(), db_type::Error> {
        info!("db_add: {name}");

        let rw = self.db.rw_transaction().unwrap();
        rw.insert(Task::new(name))?;
        rw.commit()?;
        info!("transaction committed!");
        Ok(())
    }
    pub fn read_all(&self) -> Result<Vec<Task>, db_type::Error> {
        // Read all tasks
        // Open a read-only transaction
        let r = self.db.r_transaction()?;
        // Iterate items with name starting with "red"
        let values: Vec<Task> = r.scan().primary()?.all().collect();
        Ok(values)
    }
    pub fn read_in_state(&self, state: String) -> Result<Vec<Task>, db_type::Error> {
        let mut tasks = self.read_all()?;
        tasks.retain(|x| x.state == state);
        Ok(tasks)
    }

    // pub fn rm() {}
    // pub fn set_done() {}
    // pub fn set_todo() {}
}
