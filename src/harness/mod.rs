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
