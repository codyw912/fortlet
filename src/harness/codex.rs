use crate::harness::Harness;
use crate::project::Project;

pub static CODEX: Codex = Codex;

pub struct Codex;

impl Harness for Codex {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn version(&self) -> &'static str {
        "0.147.0"
    }

    fn executable(&self) -> &'static str {
        "codex"
    }

    fn provision_script(&self) -> String {
        format!(
            "set -eu\nnpm install --global --prefix /out '@openai/codex@{}'\n/out/bin/codex --version\n",
            self.version()
        )
    }

    fn environment(&self, _project: &Project) -> Vec<(String, String)> {
        vec![("CODEX_HOME".into(), "/home/agent/.codex".into())]
    }
}
