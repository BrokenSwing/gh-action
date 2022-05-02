use std::{path::Path, process::Command};

use clap::{App, Arg, SubCommand};
mod actions;

fn main() {
    let matches = App::new("Github CLI actions extension")
        .version("1.0.0")
        .author("BrokenSwing")
        .about("An extension for Github CLI to help with GHA management and creation.")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new action.")
                .arg(
                    Arg::with_name("KIND")
                        .possible_values(&["composite", "javascript", "docker"])
                        .help("The kind of action to to create")
                        .required(true),
                )
                .arg(
                    Arg::with_name("NAME")
                        .help("The name of the action")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let action_name = matches.value_of("NAME").unwrap();
        let action_type = matches.value_of("KIND").unwrap();
        let action_path = match Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .status()
        {
            Ok(_) => {
                let output = Command::new("git")
                    .args(["rev-parse", "--show-toplevel"])
                    .output()
                    .expect("Unable to identify root directory of the git directory.");
                Path::new(&String::from_utf8(output.stdout).unwrap().trim())
                    .join(".github")
                    .join("actions")
            }
            Err(_) => Path::new(".").to_path_buf(),
        };
        println!("{:?}", &action_path);

        let result = match action_type {
            "composite" => actions::create_composite_action(action_name, &action_path),
            "docker" => actions::create_docker_action(action_name, &action_path),
            "javascript" => actions::create_javascript_action(action_name, &action_path),
            _ => panic!("Unsupported action type"),
        };
        if let Err(err) = result {
            eprintln!("Unable to create the action. {}", err);
        }
    }
}
