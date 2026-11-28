//! The audit log (rust/audit.jsonl): `enable` and `disable` append one line each, the file is
//! only ever appended to, `log` reads it (also on a repository with none) and `map` shows each
//! port's last switch. `gate`'s entries are checked where a real port is gated, in
//! `tests/fuzz.rs`.

mod common;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rust_anything::walk::Canonical;
use rust_anything::{audit, bridge, port_map};
use serde_json::Value;

const CLI: &str = env!("CARGO_BIN_EXE_rust-anything");

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            copy_tree(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).unwrap();
        }
    }
}

fn temp(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("rust-anything-audit-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

/// The `mini` fixture in a temporary repository, with `util.ts` bridged into `rust/util-rs`.
fn bridged(name: &str) -> (PathBuf, PathBuf) {
    let repo = temp(name);
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini"),
        &repo,
    );
    let crate_dir = repo.join("rust/util-rs");
    bridge::bridge(&bridge::Options {
        root: &repo,
        unit: "packages/a/src/util.ts",
        kind: None,
        out: Some(crate_dir.clone()),
        name: None,
        force: false,
        keep: Vec::new(),
        lang: None,
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })
    .unwrap();
    (repo, crate_dir)
}

fn cli(args: &[&str], path: &Path) -> String {
    let out = Command::new(CLI).args(args).arg(path).output().unwrap();
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn lines(repo: &Path) -> Vec<Value> {
    std::fs::read_to_string(repo.join(audit::FILE))
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// A commit in `repo` when git is installed, so the entries can carry it.
fn commit(repo: &Path) -> Option<String> {
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(repo)
            .args([
                "-c",
                "user.name=someone",
                "-c",
                "user.email=someone@example.com",
            ])
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
    };
    git(&["init", "-q"])?;
    git(&["add", "-A"])?;
    git(&["commit", "-q", "-m", "start"])?;
    let head = git(&["rev-parse", "HEAD"])?;
    Some(String::from_utf8(head.stdout).unwrap().trim().to_string())
}

#[test]
fn enable_and_disable_append_one_entry_each_and_never_rewrite_the_log() {
    let (repo, crate_dir) = bridged("switch");
    let head = commit(&repo);
    assert!(!repo.join(audit::FILE).exists());

    cli(&["enable", "--force", "--json"], &crate_dir);
    let first = std::fs::read(repo.join(audit::FILE)).unwrap();
    let entries = lines(&repo);
    assert_eq!(entries.len(), 1);
    let on = &entries[0];
    assert_eq!(on["command"], "enable");
    assert_eq!(on["change"], "on");
    assert_eq!(on["forced"], true, "{on}");
    assert_eq!(on["unit"], "packages/a/src/util.ts");
    assert_eq!(on["crate"], "util-rs");
    assert_eq!(on["crate_dir"], "rust/util-rs");
    assert_eq!(
        on["functions"],
        serde_json::json!(["rangeOf", "words", "count"])
    );
    assert_eq!(on["files"], serde_json::json!(["packages/a/src/util.ts"]));
    assert_eq!(on["version"], env!("CARGO_PKG_VERSION"));
    let time = on["time"].as_str().unwrap();
    assert!(
        regex::Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$")
            .unwrap()
            .is_match(time),
        "{time}"
    );
    if let Some(head) = &head {
        assert_eq!(on["commit"], head.as_str());
    }

    cli(&["disable", "--json"], &crate_dir);
    cli(&["disable"], &crate_dir);
    cli(&["enable", "--force", "--only", "count"], &crate_dir);

    let text = std::fs::read(repo.join(audit::FILE)).unwrap();
    assert!(
        text.starts_with(&first),
        "earlier lines are kept as written"
    );
    let entries = lines(&repo);
    assert_eq!(entries.len(), 4);
    assert_eq!(entries[1]["command"], "disable");
    assert_eq!(entries[1]["change"], "off");
    assert!(entries[1].get("forced").is_none(), "{}", entries[1]);
    assert_eq!(
        entries[1]["files"],
        serde_json::json!(["packages/a/src/util.ts"])
    );
    assert!(
        entries[2].get("files").is_none(),
        "a disable with nothing on changes no file: {}",
        entries[2]
    );
    assert_eq!(entries[3]["selection"], "only");
    assert_eq!(entries[3]["forced"], true);
    assert_eq!(entries[3]["functions"], serde_json::json!(["count"]));

    let raw = String::from_utf8(text).unwrap();
    assert!(!raw.contains("someone"), "no user name or email: {raw}");
    assert!(
        !raw.contains(repo.to_str().unwrap()),
        "no absolute path: {raw}"
    );
    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn log_reads_an_empty_repository_without_writing_anything() {
    let repo = temp("empty");
    let json: Value = serde_json::from_str(&cli(&["log", "--json"], &repo)).unwrap();
    assert_eq!(json["entries"], serde_json::json!([]));
    assert_eq!(json["unreadable"], 0);
    let text = cli(&["log"], &repo);
    assert!(text.contains("No changes logged yet"), "{text}");
    assert!(!repo.join("rust").exists(), "reading writes nothing");

    let missing = repo.join("missing");
    let out = Command::new(CLI).arg("log").arg(&missing).output().unwrap();
    assert!(!out.status.success());
    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn log_shows_forced_switches_and_map_shows_the_last_switch_per_port() {
    let (repo, crate_dir) = bridged("read");
    let map = port_map::map(&repo, &[]).unwrap();
    assert!(map.ports[0].last_change.is_none());
    assert!(port_map::markdown(&map).contains("| Last change |"));

    cli(&["enable", "--force"], &crate_dir);
    let text = cli(&["log"], &repo);
    assert!(text.contains("FORCED"), "{text}");
    assert!(text.contains("rust/util-rs"), "{text}");
    assert!(text.contains("rangeOf, words, count"), "{text}");
    assert_eq!(
        text,
        cli(&["log"], &crate_dir),
        "a crate folder reads its repository's log"
    );

    let map = port_map::map(&repo, &[]).unwrap();
    let last = map.ports[0].last_change.as_ref().unwrap();
    assert_eq!(last.change, "on");
    assert!(last.forced);
    assert!(port_map::markdown(&map).contains("FORCED"));

    cli(&["disable"], &crate_dir);
    let map = port_map::map(&repo, &[]).unwrap();
    assert_eq!(map.ports[0].last_change.as_ref().unwrap().change, "off");

    // A torn last line (a crash mid-write, a bad hand edit) is skipped and counted, and the next
    // entry still starts on a line of its own.
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(repo.join(audit::FILE))
        .unwrap();
    file.write_all(b"{\"time\": \"2026-").unwrap();
    drop(file);
    cli(&["enable", "--force"], &crate_dir);
    let json: Value = serde_json::from_str(&cli(&["log", "--json"], &repo)).unwrap();
    assert_eq!(json["entries"].as_array().unwrap().len(), 3, "{json}");
    assert_eq!(json["unreadable"], 1);
    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn the_log_tool_reads_the_same_entries() {
    let (repo, crate_dir) = bridged("mcp");
    cli(&["enable", "--force"], &crate_dir);
    let mut child = Command::new(CLI)
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let call = serde_json::json!({
        "jsonrpc": "2.0", "id": 2, "method": "tools/call",
        "params": { "name": "log", "arguments": { "repo": repo } }
    });
    let mut stdin = child.stdin.take().unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"2025-06-18","capabilities":{{}},"clientInfo":{{"name":"test","version":"0"}}}}}}"#
    )
    .unwrap();
    writeln!(stdin, "{call}").unwrap();
    drop(stdin);
    let out = child.wait_with_output().unwrap();
    let text = String::from_utf8(out.stdout).unwrap();
    let reply = text
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|m| m["id"] == 2)
        .expect("tools/call answered");
    let body = &reply["result"]["structuredContent"];
    assert_eq!(body["entries"][0]["forced"], true, "{reply}");
    assert_eq!(body["entries"][0]["crate"], "util-rs");
    std::fs::remove_dir_all(&repo).unwrap();
}
