use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

impl Serialize for ActionRun {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        match &self {
            ActionRun::Composite { steps } => {
                map.serialize_entry("using", "composite")?;
                map.serialize_entry("steps", steps)?;
            }
            ActionRun::Docker { image } => {
                map.serialize_entry("using", "docker")?;
                map.serialize_entry("image", image)?;
            }
            ActionRun::Javascript { main } => {
                map.serialize_entry("using", "node16")?;
                map.serialize_entry("main", main)?;
            }
        }
        map.end()
    }
}

impl Serialize for ShellKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ShellKind::BASH => serializer.serialize_str("bash"),
            ShellKind::PWSH => serializer.serialize_str("pwsh"),
            ShellKind::PYTHON => serializer.serialize_str("python"),
            ShellKind::SH => serializer.serialize_str("sh"),
            ShellKind::CMD => serializer.serialize_str("cmd"),
            ShellKind::POWERSHELL => serializer.serialize_str("powershell"),
        }
    }
}

#[derive(Debug)]
pub enum ActionRun {
    Composite { steps: Vec<ActionStep> },
    Docker { image: String },
    Javascript { main: String },
}

#[derive(Debug, Serialize)]
pub struct ActionStep {
    pub run: String,
    pub shell: Option<ShellKind>,
}

#[derive(Debug)]
pub enum ShellKind {
    BASH,
    PWSH,
    PYTHON,
    SH,
    CMD,
    POWERSHELL,
}
