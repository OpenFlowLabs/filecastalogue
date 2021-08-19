use std::{error::Error, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use tempdir::TempDir;

static mut test_dir: Option<TempDir> = None;

#[test]
fn generate_test_data() -> Result<(), Box<dyn Error>> {

    unsafe {
        test_dir = Some(TempDir::new("test_dir")?);
    }

    Ok(())
}



#[test]
fn init_repository() -> Result<(), Box<dyn Error>> {
    //filecastaloguer init /path

    let mut cmd = Command::cargo_bin("filecastaloguer")?;

    cmd.arg("init");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));

    Ok(())
}