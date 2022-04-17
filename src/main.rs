use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Github CLI actions extension")
        .version("1.0.0")
        .author("BrokenSwing")
        .about("An extension for Github CLI to help with GHA management and creation.")
        .subcommand(SubCommand::with_name("new"));
}
