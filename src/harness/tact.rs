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

    fn executable(&self) -> &'static str {
        "tact"
    }

    fn provision_script(&self) -> String {
        format!(
            r#"set -eu
case "$(uname -m)" in
  x86_64|amd64) target=x86_64-unknown-linux-gnu ;;
  aarch64|arm64) target=aarch64-unknown-linux-gnu ;;
  *) echo "unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac
version="v{}"
archive="tact-$target-$version.tar.gz"
url="https://github.com/clabby/tact/releases/download/$version"
temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT
curl --proto '=https' --tlsv1.2 -LsSf -o "$temporary/$archive" "$url/$archive"
curl --proto '=https' --tlsv1.2 -LsSf -o "$temporary/$archive.sha256" "$url/$archive.sha256"
(cd "$temporary" && sha256sum -c "$archive.sha256")
tar -xzf "$temporary/$archive" -C "$temporary" "tact-$target-$version/tact"
mkdir -p /out/bin
install -m 755 "$temporary/tact-$target-$version/tact" /out/bin/tact
/out/bin/tact --version
"#,
            self.version()
        )
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
