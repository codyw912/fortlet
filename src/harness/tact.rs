use crate::harness::Harness;
use crate::project::Project;

pub static TACT: Tact = Tact;

pub struct Tact;

impl Harness for Tact {
    fn name(&self) -> &'static str {
        "tact"
    }

    fn version(&self) -> &'static str {
        "0.3.7"
    }

    fn environment(&self, project: &Project) -> Vec<(String, String)> {
        vec![
            ("TACT_HOME".into(), "/home/agent/.tact".into()),
            (
                "TACT_AUTH_FILE".into(),
                "/home/agent/.codex/auth.json".into(),
            ),
            ("TACT_AUTH".into(), "chatgpt".into()),
            ("TACT_WORKSPACE".into(), project.root.display().to_string()),
        ]
    }
}
