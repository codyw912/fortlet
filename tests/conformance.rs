use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

#[test]
fn conformance_map_is_complete_and_references_existing_evidence() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let map_path = root.join("arch/conformance.json");
    let map: Value = serde_json::from_str(
        &fs::read_to_string(&map_path)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", map_path.display())),
    )
    .unwrap_or_else(|error| panic!("invalid {}: {error}", map_path.display()));

    let entries = map["entries"]
        .as_array()
        .expect("conformance entries must be an array");
    let mut mapped = BTreeSet::new();

    for entry in entries {
        let proposal = text(entry, "proposal");
        assert!(
            proposal.len() == 4 && proposal.chars().all(|character| character.is_ascii_digit()),
            "proposal id must contain four digits: {proposal}"
        );
        assert!(
            mapped.insert(proposal.to_owned()),
            "duplicate FIP-{proposal}"
        );

        let status = text(entry, "status");
        assert!(
            ["conformant", "partial", "unimplemented", "n-a"].contains(&status),
            "FIP-{proposal} has invalid status {status}"
        );

        let tests = strings(entry, "tests");
        let gaps = strings(entry, "gaps");
        if status == "conformant" {
            assert!(!tests.is_empty(), "conformant FIP-{proposal} needs tests");
        }
        if status == "partial" {
            assert!(!gaps.is_empty(), "partial FIP-{proposal} needs gaps");
        }

        for evidence in strings(entry, "modules").into_iter().chain(tests) {
            assert!(
                root.join(evidence).exists(),
                "FIP-{proposal} references missing path {evidence}"
            );
        }
    }

    let proposals = proposal_ids(&root.join("arch/proposals"));
    assert_eq!(
        mapped, proposals,
        "every proposal file must have exactly one conformance entry"
    );
}

fn text<'a>(entry: &'a Value, key: &str) -> &'a str {
    entry[key]
        .as_str()
        .unwrap_or_else(|| panic!("conformance field {key} must be a string"))
}

fn strings<'a>(entry: &'a Value, key: &str) -> Vec<&'a str> {
    entry[key]
        .as_array()
        .unwrap_or_else(|| panic!("conformance field {key} must be an array"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("conformance field {key} must contain strings"))
        })
        .collect()
}

fn proposal_ids(directory: &Path) -> BTreeSet<String> {
    fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", directory.display()))
        .map(|entry| {
            let name = entry.expect("cannot read proposal entry").file_name();
            let name = name.to_string_lossy();
            let (id, _) = name
                .split_once('-')
                .unwrap_or_else(|| panic!("proposal filename needs NNNN-slug: {name}"));
            assert_eq!(id.len(), 4, "proposal id must contain four digits: {name}");
            id.to_owned()
        })
        .collect()
}
