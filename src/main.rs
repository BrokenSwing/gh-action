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
        let action = match action_type {
            "composite" => actions::CompositeAction::default(action_name),
            _ => panic!("Unsupported action kind"),
        };
        if let Err(err) = actions::create_action_locally(&action, action_name) {
            eprintln!("Unable to create the action. {}", err);
        }
    }
}
