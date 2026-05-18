use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

const FLOWCHART: &str = "flowchart TD\n  A --> B --> C\n";

fn arielc() -> Command {
    Command::cargo_bin("arielc").unwrap()
}

fn mmd_file() -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(FLOWCHART.as_bytes()).unwrap();
    f
}

#[test]
fn version() {
    arielc()
        .arg("--version")
        .assert()
        .success()
        .stdout(contains("arielc"));
}

#[test]
fn basic_render_to_file() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"), "output must contain <svg");
}

#[test]
fn theme_dark() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .args(["--theme", "dark"])
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn theme_forest() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .args(["--theme", "forest"])
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn theme_neutral() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .args(["--theme", "neutral"])
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn unknown_theme_falls_back_to_default() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .args(["--theme", "unknown"])
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn quiet_flag_suppresses_stderr() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .arg("--quiet")
        .assert()
        .success()
        .stderr("");
}

#[test]
fn stdout_output() {
    let input = mmd_file();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", "-"])
        .assert()
        .success()
        .stdout(contains("<svg"));
}

#[test]
fn stdin_input() {
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", "-"])
        .args(["-o", output.path().to_str().unwrap()])
        .write_stdin(FLOWCHART)
        .assert()
        .success();
    let svg = std::fs::read_to_string(output.path()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn background_color_flag_accepted() {
    let input = mmd_file();
    let output = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    arielc()
        .args(["-i", input.path().to_str().unwrap()])
        .args(["-o", output.path().to_str().unwrap()])
        .args(["--background-color", "transparent"])
        .assert()
        .success();
}

#[test]
fn missing_input_exits_nonzero() {
    arielc()
        .args(["-i", "nonexistent.mmd"])
        .args(["-o", "-"])
        .assert()
        .failure();
}

#[test]
fn all_diagram_types_render() {
    let diagrams = [
        "pie title Pets\n  \"Dogs\" : 386\n  \"Cats\" : 85",
        "sequenceDiagram\n  Alice->>Bob: Hello",
        "gantt\n  dateFormat YYYY-MM-DD\n  section A\n  Task: 2024-01-01, 7d",
    ];
    for source in &diagrams {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(source.as_bytes()).unwrap();
        arielc()
            .args(["-i", f.path().to_str().unwrap()])
            .args(["-o", "-"])
            .assert()
            .success()
            .stdout(contains("<svg"));
    }
}
