use std::{error::Error, process::Command};
use std::*;

use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use tempdir::TempDir;

static mut Test_dir: Option<TempDir> = None;

#[test]
fn generate_test_data() -> Result<(), Box<dyn Error>> {

    unsafe {
        Test_dir = Some(TempDir::new("test_dir")?);
    }

    Ok(())
}

#[test]
fn create_new_repository() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("new").arg("repository");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("create new repository in"));

    Ok(())
}

#[test]
fn create_new_repository_with_path() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("new").arg("repository").arg("/test_dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("create new repository in /test_dir"));

    Ok(())
}

#[test]
fn check_repository() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("check").arg("repository");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_version() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_version_with_id() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("version").arg("-id id");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_file() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("file").arg("-p /path/to/file");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn add_directory() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("add").arg("directory").arg("/path/to/dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn update_version() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("update").arg("version").arg("new_version_id");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn report() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("report");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn remove_version() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("remove").arg("version").arg("version_id");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn apply() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("apply");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn list_versions() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("list").arg("versions");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn list_files() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("list").arg("files");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn duplicate() -> Result<(), Box<dyn Error>> {

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("dublicate").arg("version").arg("version_id").arg("new_version_id");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
#[should_panic]
fn panic_test(){
    //let mut cmd = Command::cargo_bin("grrs");

    //cmd.arg("foobar").arg("test/file/doesnt/exist");
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("No such file or directory"));

    panic!

    ()
}