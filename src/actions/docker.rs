use super::{ActionRun, GithubAction};

pub struct DockerAction;

impl DockerAction {
    pub fn default(action_name: &str) -> GithubAction {
        GithubAction {
            name: action_name.to_string(),
            description: "A Docker action generated w/ gh-action".to_string(),
            runs: ActionRun::Docker {
                image: "Dockerfile".to_string(),
            },
        }
    }
}
