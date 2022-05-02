use std::{path::PathBuf, process::Command};

#[cfg(windows)]
const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
const NPM: &'static str = "npm";

pub struct Npm {
    cwd: PathBuf,
}

impl Npm {
    pub fn new(cwd: PathBuf) -> Npm {
        Npm { cwd }
    }
    pub fn init(&self) {
        Command::new(NPM)
            .current_dir(&self.cwd)
            .args(["init", "-y"])
            .output()
            .expect("Unable to initialize NPM repository. Is `npm` installed ?");
    }
    pub fn install(&self, module_name: &str) {
        Command::new(NPM)
            .current_dir(&self.cwd)
            .args(["install", "@actions/core"])
            .output()
            .expect(
                format!(
                    "Unable to install NPM package: {}. Is NPM installed ?",
                    module_name
                )
                .as_str(),
            );
    }
}
