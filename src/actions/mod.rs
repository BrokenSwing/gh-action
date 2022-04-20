use dialoguer::{theme::ColorfulTheme, Confirm};
use serde::Serialize;
use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

mod composite;
mod runs;

pub use composite::CompositeAction;
pub use runs::{ActionRun, ActionStep, ShellKind};

#[derive(Debug, Serialize)]
pub struct GithubAction {
    pub name: String,
    pub description: String,
    pub runs: runs::ActionRun,
}

pub fn create_action_locally(action: &GithubAction, action_name: &str) -> std::io::Result<()> {
    let serialized_action = serde_yaml::to_string(&action).unwrap();

    let actions_dir_path = Path::new(".github").join("actions").join(action_name);

    if actions_dir_path.exists() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("An action with the given name already exists. Override it ?")
            .interact()?
        {
            remove_dir_all(&actions_dir_path)?;
        } else {
            println!("Canceled.");
            return Ok(());
        }
    }

    create_dir_all(&actions_dir_path)?;

    let action_file_path = actions_dir_path.join("action.yml");

    let mut file = File::create(&action_file_path)?;
    file.write_all(serialized_action.as_bytes())?;
    println!("Action created sucessfully!");
    Ok(())
}
