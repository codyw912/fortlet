mod codex;
mod tact;

use anyhow::{bail, Result};

use crate::project::Project;

pub trait Harness: Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn executable(&self) -> &'static str;
    fn provision_script(&self) -> String;
    fn environment(&self, project: &Project) -> Vec<(String, String)>;

    fn launch_arguments(&self, requested: &[String]) -> Vec<String> {
        requested.to_vec()
    }
}

pub fn find(name: &str) -> Result<&'static dyn Harness> {
    match name {
        "codex" => Ok(&codex::CODEX),
        "tact" => Ok(&tact::TACT),
        _ => bail!("unsupported harness {name:?}; expected codex or tact"),
    }
}

pub fn names() -> impl Iterator<Item = &'static str> {
    [codex::CODEX.name(), tact::TACT.name()].into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_disables_apps_before_preserving_requested_arguments() {
        let requested = vec![
            "exec".to_owned(),
            "--enable".to_owned(),
            "apps".to_owned(),
            "say hello".to_owned(),
        ];

        assert_eq!(
            find("codex").unwrap().launch_arguments(&requested),
            ["--disable", "apps"]
                .into_iter()
                .map(str::to_owned)
                .chain(requested)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn tact_preserves_requested_arguments() {
        let requested = vec!["--version".to_owned(), "literal value".to_owned()];

        assert_eq!(
            find("tact").unwrap().launch_arguments(&requested),
            requested
        );
    }
}
