use std::process::Command;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_run() {
    let output = Command::new("/bin/bash")
        .args(["fixtures/run.sh", "1", "dos", "three"])
        .output()
        .expect("failed miserably");
    assert_eq!(b"1 dos three\n", output.stdout.as_slice());
}

#[test]
fn test_fail() {
    let output = Command::new("/bin/bash")
        .args(["fixtures/error.sh", "1", "dos", "three"])
        .output()
        .expect("failed miserably");
    assert_eq!(
        "1 dos three\n",
        std::str::from_utf8(&output.stderr).unwrap()
    );
}
