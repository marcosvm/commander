use serde::Deserialize;

/// Commandline represents one executable command and its arguments
#[derive(Debug, PartialEq, Deserialize)]
pub struct CommandLine {
    executable: String,
    arguments: Option<Vec<String>>,
}

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

    let input = std::fs::read_to_string("./fixtures/input.json").unwrap();
    assert_eq!(
        commands,
        serde_json::from_str::<Vec<CommandLine>>(&input).unwrap()
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

    let input = std::fs::read_to_string("./fixtures/input.yaml").unwrap();
    assert_eq!(
        commands,
        serde_yaml::from_str::<Vec<CommandLine>>(&input).unwrap()
    );
}
