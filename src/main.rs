use std::path::Path;

use clap::{App, Arg, SubCommand};
mod actions;
mod git;
mod npm;

fn cli_cmd() -> App<'static, 'static> {
    App::new("Github CLI actions extension")
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
}

fn main() {
    let cmd = cli_cmd();
    let matches = cmd.get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let action_name = matches.value_of("NAME").unwrap();
        let action_type = matches.value_of("KIND").unwrap();
        let action_path = if git::in_repository() {
            git::repository_root().join(".github").join("actions")
        } else {
            Path::new(".").to_path_buf()
        };

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
