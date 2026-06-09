use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_add_file_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "@prefix ex: <http://example.org/> .\nex:Item ex:name \"Test\" .")?;

    let mut cmd = Command::cargo_bin("uht-trace-blockchain")?;
    cmd.arg("add-file")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Added RDF as block. Blockchain valid? true"));

    Ok(())
}

#[test]
fn test_cli_add_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("uht-trace-blockchain")?;
    cmd.arg("add-file")
        .arg("non_existent_file.ttl")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot read RDF file:"));

    Ok(())
}

#[test]
fn test_cli_query_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "SELECT ?s ?p ?o WHERE {{ ?s ?p ?o }}")?;

    let mut cmd = Command::cargo_bin("uht-trace-blockchain")?;
    cmd.arg("query")
        .arg(file.path())
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_cli_query_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("uht-trace-blockchain")?;
    cmd.arg("query")
        .arg("non_existent_query.rq")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot read query file:"));

    Ok(())
}

#[test]
fn test_cli_demo_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("uht-trace-blockchain")?;
    cmd.arg("demo")
        .assert()
        .success()
        .stdout(predicate::str::contains("Blockchain valid? true"));

    Ok(())
}
