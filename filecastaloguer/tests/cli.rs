use std::path::*;
use std::sync::Once;
use std::*;
use std::{error::Error, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use std::fs::File;
use std::io::{self, Write};
use tempdir::TempDir;

static mut TEST_DIR: Option<TempDir> = None;
static mut TEST_REPO_DIR: Option<TempDir> = None;

// static INIT: Once = Once::new();

// pub fn initialize() {
//     INIT.call_once(|| {
//         #[test]
//         fn generate_test_data() -> Result<(), Box<dyn Error>> {
//             let mut cmd = Command::cargo_bin("filecastaloguer")?;

//             unsafe {
//                 TEST_DIR = Some(TempDir::new("test_dir")?);
//                 TEST_REPO_DIR = Some(TempDir::new("test_repo_dir")?);
//             }

//             let temp_dir = TempDir::new("example")?;
//             let file_path = temp_dir.path().join("state.json");
//             //let file_path = temp_dir.path().join("my-temporary-note.txt");
//             let mut tmp_file = File::create(&file_path)?;
//             //writeln!(tmp_file, "Brian was here. Briefly.")?;

//             //let contents = fs::read_to_string("my-temporary-note.txt")?;

//             //assert_eq!(contents, "Brian was here. Briefly.");
//             //assert!(env::set_current_dir(&Path::new("/test_repo_dir")).is_ok());

//             Ok(())
//         }
//     });
// }



#[test]
fn generate_test_data() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    unsafe {
        TEST_DIR = Some(TempDir::new("test_dir")?);
        TEST_REPO_DIR = Some(TempDir::new("test_repo_dir")?);
    }

    let temp_dir = TempDir::new("example")?;
    let file_path = temp_dir.path().join("state.json");
    //let file_path = temp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = File::create(&file_path)?;
    writeln!(tmp_file, "Brian was here. Briefly.")?;

    //let contents = fs::read_to_string("state.json")?; //errpr why?

    //assert_eq!(contents, "Brian was here. Briefly.");
    //assert!(env::set_current_dir(&Path::new("/test_repo_dir")).is_ok());

    Ok(())
}

#[test]
fn create_new_repository() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    //initialize();
    //create new repository in working directory
    cmd.arg("new").arg("repository");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("create new repository in"));

    Ok(())
}

#[test]
fn create_new_repository_with_path() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("new").arg("repository").arg("/test_repo_dir");
    cmd.assert().success().stdout(predicate::str::contains(
        "create new repository in /test_repo_dir",
    ));

    Ok(())
}

#[test]
fn check_repository() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("check").arg("repository");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("version");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_version_with_id() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("version").arg("-i id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("file").arg("-p /path/to/file");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_directory() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("directory").arg("/path/to/dir");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn insert_before_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("insert").arg("version").arg("before").arg("version id").arg("version id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn insert_after_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("insert").arg("version").arg("after").arg("version id").arg("version id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn update_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("update").arg("version").arg("version_id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn report() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("report");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn remove_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("remove").arg("version").arg("version_id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn apply() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("apply");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn list_versions() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("list").arg("versions");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn list_files() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("list").arg("files");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn duplicate() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("dublicate")
        .arg("version")
        .arg("version_id")
        .arg("new_version_id");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn delete_repository() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("delete").arg("repository");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}


#[test]
fn delete_repository_with_path() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("delete").arg("repository").arg("/test_repo_dir");
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
#[should_panic]
fn panic_test() {
    //let mut cmd = Command::cargo_bin("grrs");

    //cmd.arg("foobar").arg("test/file/doesnt/exist");
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("No such file or directory"));

    panic!()
}
