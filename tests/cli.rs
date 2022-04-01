use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[test]
fn run_with_out_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("j3di")
        .expect("binary exists")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn file_doesnt_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("j3di")?;

    cmd.arg("edit").arg("test/file/doesnt/exists");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn change_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("j3di")?;
    let path = Path::new("tests/data/test.json");
    let content = std::fs::read_to_string(&path).unwrap();

    cmd.arg("edit")
        .arg(path)
        .arg("--update")
        .arg("Hallo.das")
        .arg("--value")
        .arg("WALDO");
    cmd.assert().success();

    let content_after = std::fs::read_to_string(&path).unwrap();

    assert_ne!(content, content_after);

    let mut file = File::create(path).unwrap();

    file.write_all(content.as_bytes())
        .expect("Could not write to file");

    Ok(())
}

#[test]
fn change_value_to_null() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("j3di")?;
    let path = Path::new("tests/data/test.json");
    let content = std::fs::read_to_string(&path).unwrap();

    cmd.arg("edit")
        .arg(path)
        .arg("--update")
        .arg("Hallo.das")
        .arg("--type-of")
        .arg("null");
    cmd.assert().success();

    let content_after = std::fs::read_to_string(&path).unwrap();

    assert_ne!(content, content_after);

    let mut file = File::create(path).unwrap();

    file.write_all(content.as_bytes())
        .expect("Could not write to file");

    Ok(())
}

#[test]
fn change_value_to_object() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("j3di")?;
    let path = Path::new("tests/data/test.json");
    let content = std::fs::read_to_string(&path).unwrap();

    cmd.arg("edit")
        .arg(path)
        .arg("--update")
        .arg("Hallo.das")
        .arg("--type-of")
        .arg("object")
        .arg("--value")
        .arg("{ \"Test\": \"Test\" }");
    cmd.assert().success();

    let content_after = std::fs::read_to_string(&path).unwrap();


    assert_ne!(content, content_after);

    let mut file = File::create(path).unwrap();

    file.write_all(content.as_bytes())
        .expect("Could not write to file");

    Ok(())
}
