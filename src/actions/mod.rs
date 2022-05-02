use dialoguer::{theme::ColorfulTheme, Confirm};
use serde::Serialize;
use std::fs::OpenOptions;
use std::process::Command;
use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::PathBuf,
};

mod composite;
mod docker;
mod javascript;
mod runs;

pub use composite::CompositeAction;
pub use docker::DockerAction;
pub use runs::{ActionRun, ActionStep, ShellKind};

use self::javascript::JavascriptAction;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

#[derive(Debug, Serialize)]
pub struct GithubAction {
    pub name: String,
    pub description: String,
    pub runs: runs::ActionRun,
}

fn create_action_yaml(
    action_name: &str,
    action: &GithubAction,
    action_path: &PathBuf
) -> std::io::Result<Option<PathBuf>> {
    let actions_dir_path = action_path.join(action_name);

    if actions_dir_path.exists() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("An action with the given name already exists. Override it ?")
            .interact()?
        {
            remove_dir_all(&actions_dir_path)?;
        } else {
            println!("Canceled.");
            return Ok(None);
        }
    }

    create_dir_all(&actions_dir_path)?;

    let action_file_path = actions_dir_path.join("action.yml");
    let serialized_action = serde_yaml::to_string(&action).unwrap();

    let mut file = File::create(&action_file_path)?;
    file.write_all(serialized_action.as_bytes())?;
    Ok(Some(actions_dir_path))
}

pub fn create_composite_action(action_name: &str, action_path: &PathBuf) -> std::io::Result<()> {
    let action = CompositeAction::default(action_name);
    let created = create_action_yaml(action_name, &action, action_path)?;
    if let Some(_) = created {
        println!("Action created sucessfully!");
    };
    Ok(())
}

pub fn create_docker_action(action_name: &str, action_path: &PathBuf) -> std::io::Result<()> {
    let action = DockerAction::default(action_name);
    let created = create_action_yaml(action_name, &action, action_path)?;
    if let Some(action_dir) = created {
        let dockerfile_path = action_dir.join("Dockerfile");
        let mut dockerfile = File::create(&dockerfile_path)?;
        dockerfile.write_all("FROM alpine:3.10\n".as_bytes())?;
        dockerfile.write_all("COPY entrypoint.sh /entrypoint.sh\n".as_bytes())?;
        dockerfile.write_all(r#"ENTRYPOINT ["/entrypoint.sh"]"#.as_bytes())?;

        let entrypoint_file_path = action_dir.join("entrypoint.sh");
        let mut entrypoint = File::create(&entrypoint_file_path)?;
        entrypoint.write_all("#!/bin/sh -l\n".as_bytes())?;
        entrypoint.write_all(r#"echo "Hello, world!""#.as_bytes())?;
        println!("Action created sucessfully!");
    }
    Ok(())
}

fn npm(action_dir: &PathBuf) -> Command {
    let mut npm = Command::new(NPM);
    npm.current_dir(action_dir);
    npm
}

fn git() -> Command {
    Command::new("git")
}

pub fn create_javascript_action(action_name: &str, action_path: &PathBuf) -> std::io::Result<()> {
    let action = JavascriptAction::default(action_name);
    let created = create_action_yaml(action_name, &action, action_path)?;
    if let Some(action_dir) = created {
        let js_file_path = action_dir.join("index.js");
        let mut js_file = File::create(&js_file_path)?;
        js_file.write_all("const core = require('@actions/core');\n".as_bytes())?;
        js_file.write_all("const github = require('@actions/github');\n".as_bytes())?;
        js_file.write_all("\nconsole.log('Hello, world!');".as_bytes())?;

        npm(&action_dir)
            .args(["init", "-y"])
            .output()
            .expect("Is `npm` installed ?");
        println!("NPM project initialized");

        npm(&action_dir)
            .args(["install", "@actions/core"])
            .output()?;
        println!("Package @actions/core installed");

        npm(&action_dir)
            .args(["install", "@actions/github"])
            .output()?;
        println!("Package @actions/github installed");

        let node_modules_path = action_dir.join("node_modules");
        let node_modules_path = node_modules_path.to_str().unwrap();
        let node_modules_path = node_modules_path.replace("\\", "/");

        let node_modules_ignored = git()
            .args(["check-ignore", "-q", node_modules_path.as_str()])
            .status();

        match node_modules_ignored {
            Ok(status) if status.success() => {
                if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Action's `node_modules` directory is git-ignored, it shouldn't. Update your .gitignore ?",
                )
                .interact()?
                {
                    let mut gitignore = OpenOptions::new().append(true).open(".gitignore")?;
                    gitignore.write_all(format!("\n!{}", node_modules_path).as_bytes())?;
                } else {
                    println!(".gitignore not updated. You should take a look at: https://docs.github.com/en/actions/creating-actions/creating-a-javascript-action#commit-tag-and-push-your-action-to-github");
                }
            },
            _ => {}
        }
    }
    Ok(())
}
