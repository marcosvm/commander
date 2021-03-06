mod command_line;

use crate::command_line::CommandLine;
use chrono::Utc;
use log::info;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "commander", about = "Running many command lines")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(short, default_value = "yaml")]
    input_type: String,
}

fn main() -> Result<(), command_line::Error> {
    let opt = Opt::from_args();
    env_logger::init();

    let filename = match opt.file.as_path().to_str() {
        Some(filename) => filename,
        None => {
            eprintln!("can't get a filename");
            process::exit(1)
        }
    };

    let commands = match opt.input_type.as_ref() {
        "yaml" => CommandLine::from_yaml(filename)?,
        "json" => CommandLine::from_json(filename)?,
        _ => {
            eprintln!("unsupported file type");
            process::exit(1)
        }
    };

    for command_line in commands.iter() {
        let mut command = Command::from(command_line);
        info!("running {:?}", command);
        let start_time = Utc::now();
        let mut status = "SUCCESS";
        let output = command.output();
        match output {
            Ok(output) => {
                println!(
                    "{}",
                    std::str::from_utf8(&output.stdout).unwrap_or_default()
                );
                eprintln!(
                    "{}",
                    std::str::from_utf8(&output.stderr).unwrap_or_default()
                );
            }
            Err(e) => {
                eprintln!("command failed: {}", e);
                status = "FAIL"
            }
        }

        let elapsed_time = Utc::now().timestamp_millis() - start_time.timestamp_millis();
        info!(
            "finished='{:?}',time elapsed={}ms,status={}",
            command, elapsed_time, status
        );
    }

    Ok(())
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
