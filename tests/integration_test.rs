// Every module has unit tests.
// Integration tests below directly guard the functionality and that
// all modules interoperate together.

use std::fs;

const TEST_DB_PATH: &str = "int_test_db_pomodorino";

#[test]
fn integration_test_add_task() {
    test_bin::get_test_bin("pomodorino")
        .args(["add", "--name", "Test_task", "--database", TEST_DB_PATH])
        .output()
        .expect("Failed to start pomodorino binary");

    let output = test_bin::get_test_bin("pomodorino")
        .args(["show", "--database", TEST_DB_PATH])
        .output()
        .expect("Failed to run show command");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "db tasks: [Task { name: \"Test_task\", state: \"todo\" }]\n"
    );

    // Cleanup
    fs::remove_file(TEST_DB_PATH).unwrap();
}

#[test]
fn integration_test_run_pomodoro() {}
