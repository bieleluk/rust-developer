use std::process::Command;

#[test]
fn test_wrong_number_of_arguments() {
    let output = Command::new("cargo")
        .arg("run")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Failed to execute cargo run command");

    // Assert that the command failed (non-zero exit code)
    assert!(!output.status.success());
}

#[test]
fn test_wrong_transformation() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("random")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Failed to execute cargo run command");

    // Assert that the command failed (non-zero exit code)
    assert!(!output.status.success());
}
