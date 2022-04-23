use super::{ActionRun, GithubAction};

pub struct JavascriptAction;

impl JavascriptAction {
    pub fn default(action_name: &str) -> GithubAction {
        GithubAction {
            name: action_name.to_string(),
            description: "A JavaScript action generated w/ gh-action".to_string(),
            runs: ActionRun::Javascript {
                main: "index.js".to_string(),
            },
        }
    }
}
