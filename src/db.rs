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
    pub fn new(
        builder: &'a mut DatabaseBuilder,
        db_path: Option<String>,
    ) -> Result<Self, db_type::Error> {
        // Initialize the model
        builder.define::<Task>()?;
        let db;

        // Create a database
        if let Some(db_path) = db_path {
            db = builder.create(db_path)?;
        } else {
            db = builder.create_in_memory()?;
        }

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
        // Iterate over the items
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

    // List suppressed as the function is used in tests.
    #[allow(dead_code)]
    // Function to clean up all information from the database.
    pub fn clear(&self) -> Result<(), db_type::Error> {
        let rw = self.db.rw_transaction().unwrap();
        for entry in rw.scan().primary()?.all() {
            let entry: Task = entry;
            info!("clear: Removing {:?}", entry);
            rw.remove(entry)?;
        }
        rw.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    // Only 1 test uses on-disk db. There's no good setup/teardown in Rust testing so each test has it's own db
    const TEST_DB_PATH: &str = "test_db_pomodorino";

    #[test]
    fn test_on_disk_db() {
        let mut builder: DatabaseBuilder = DatabaseBuilder::new(); // TODO: confirm if this works with Tokio, otherwise move to Lazy static, and mutate it with unsafe
        let db = Db::new(&mut builder, Some(String::from(TEST_DB_PATH))).unwrap();
        let task_name = String::from("Test Task on disk");
        let test_name = task_name.clone();
        db.add(task_name).unwrap();
        let tasks = db.read_all().unwrap();

        assert_eq!(test_name, tasks.get(0).unwrap().name);
        fs::remove_file(TEST_DB_PATH).unwrap();
    }

    const TEST_DB_PATH_IN_MEM: Option<String> = None; // use in memory database to allow running tests concurrently

    #[test]
    fn test_add_single() {
        let mut builder: DatabaseBuilder = DatabaseBuilder::new(); // TODO: confirm if this works with Tokio, otherwise move to Lazy static, and mutate it with unsafe
        let db = Db::new(&mut builder, TEST_DB_PATH_IN_MEM).unwrap();
        let task_name = String::from("Test Task");
        let test_name = task_name.clone();
        db.add(task_name).unwrap();
        let tasks = db.read_all().unwrap();

        assert_eq!(test_name, tasks.get(0).unwrap().name);
        let _ = db.clear();
    }

    #[test]
    fn test_add_empty() {
        let mut builder: DatabaseBuilder = DatabaseBuilder::new(); // TODO: confirm if this works with Tokio, otherwise move to Lazy static, and mutate it with unsafe
        let db = Db::new(&mut builder, TEST_DB_PATH_IN_MEM).unwrap();
        let task_name = String::from("");
        let test_name = task_name.clone();
        db.add(task_name).unwrap();
        let tasks = db.read_all().unwrap();

        assert_eq!(test_name, tasks.get(0).unwrap().name);
        let _ = db.clear();
    }

    #[test]
    fn test_clear() {}
}
