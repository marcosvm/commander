use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::process::Command;

/// Commandline represents one executable command and its arguments
#[derive(Debug, PartialEq, Deserialize)]
pub struct CommandLine {
    executable: String,
    arguments: Option<Vec<String>>,
}

impl CommandLine {
    pub(crate) fn from_yaml(path: &str) -> Result<Vec<Self>> {
        let text = read_to_string(path)?;
        Ok(serde_yaml::from_str::<Vec<CommandLine>>(&text)?)
    }

    pub(crate) fn from_json(path: &str) -> Result<Vec<Self>> {
        let text = read_to_string(path)?;
        Ok(serde_json::from_str::<Vec<CommandLine>>(&text)?)
    }
}
impl From<&CommandLine> for Command {
    fn from(item: &CommandLine) -> Self {
        let mut command = Command::new(item.executable.clone());
        for args in item.arguments.as_ref().into_iter() {
            command.args(args);
        }
        command
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(err.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::new(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(err.to_string())
    }
}

#[derive(Debug)]
pub(crate) struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
type Result<T> = std::result::Result<T, self::Error>;

#[test]
fn parse_yaml_input() {
    let input = r#"
---
executable: /bin/bash
arguments:
- 1
- duo
- three
"#;
    let cmd = CommandLine {
        executable: String::from("/bin/bash"),
        arguments: Some(vec![
            "1".to_string(),
            "duo".to_string(),
            "three".to_string(),
        ]),
    };

    assert_eq!(cmd, serde_yaml::from_str(input).unwrap())
}

#[test]
fn parse_json_input() {
    let input = r#"
{
  "executable": "/bin/bash",
  "arguments": [
    "1",
    "duo",
    "three"
   ]
}
"#;
    let cmd = CommandLine {
        executable: "/bin/bash".to_string(),
        arguments: Some(vec![
            "1".to_string(),
            "duo".to_string(),
            "three".to_string(),
        ]),
    };

    assert_eq!(cmd, serde_json::from_str(input).unwrap())
}

#[test]
fn parse_json_file_into_commands() {
    let commands = vec![
        CommandLine {
            executable: "/bin/bash".to_string(),
            arguments: Some(vec![
                "1".to_string(),
                "duo".to_string(),
                "three".to_string(),
            ]),
        },
        CommandLine {
            executable: "/scripts/command".to_string(),
            arguments: Some(vec![]),
        },
        CommandLine {
            executable: "/bin/ps".to_string(),
            arguments: None,
        },
        CommandLine {
            executable: "/bin/ps".to_string(),
            arguments: Some(vec!["-ef".to_string()]),
        },
    ];

    assert_eq!(
        commands,
        CommandLine::from_json("./fixtures/input.json").unwrap()
    );
}

#[test]
fn parse_yaml_file_into_commands() {
    let commands = vec![
        CommandLine {
            executable: "/bin/bash".to_string(),
            arguments: Some(vec![
                "1".to_string(),
                "duo".to_string(),
                "three".to_string(),
            ]),
        },
        CommandLine {
            executable: "/scripts/command".to_string(),
            arguments: Some(vec![]),
        },
        CommandLine {
            executable: "/bin/ps".to_string(),
            arguments: None,
        },
        CommandLine {
            executable: "/bin/ps".to_string(),
            arguments: Some(vec!["-ef".to_string()]),
        },
    ];

    assert_eq!(
        commands,
        CommandLine::from_yaml("./fixtures/input.yaml").unwrap()
    );
}
