use std::time::Duration;

use anyhow::{bail, Result};

use crate::project::Project;

mod codex;
mod tact;

pub trait Harness: Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn environment(&self, project: &Project) -> Vec<(String, String)>;

    fn launch_arguments(&self, requested: &[String]) -> Vec<String> {
        requested.to_vec()
    }

    fn non_interactive_idle_timeout(&self) -> Option<Duration> {
        None
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

    #[test]
    fn only_codex_has_a_non_interactive_inactivity_ceiling() {
        assert_eq!(
            find("codex").unwrap().non_interactive_idle_timeout(),
            Some(std::time::Duration::from_secs(10 * 60))
        );
        assert_eq!(find("tact").unwrap().non_interactive_idle_timeout(), None);
    }
}
