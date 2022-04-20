use super::{ActionRun, ActionStep, GithubAction, ShellKind};
pub struct CompositeAction;

impl CompositeAction {
    pub fn default(action_name: &str) -> GithubAction {
        GithubAction {
            name: String::from(action_name),
            description: "A composite action saying hello to the world".to_string(),
            runs: ActionRun::Composite {
                steps: vec![ActionStep {
                    run: "echo Hello, world!".to_string(),
                    shell: Some(ShellKind::BASH),
                }],
            },
        }
    }
}
