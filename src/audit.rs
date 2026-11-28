//! The audit log: what runs in production, and every change to it.
//!
//! One JSON line per change in `<repo>/rust/audit.jsonl`, next to the port rules and the map.
//! `enable`, `disable`, `app wire`, `app lift` and `go` (when it switches a unit on) append an
//! entry, and so does every `gate`, passed or failed. The file is only ever appended to: nothing
//! rust-anything does rewrites or trims it. An entry holds the time in UTC, the command, the unit
//! and crate, what changed, the rust-anything version and the repository's git commit; never a
//! user name, an email or an absolute path, so it can be committed with the port.

use std::io::{Read as _, Seek as _, SeekFrom, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::bridge::Manifest;
use crate::enable::Switched;
use crate::front_door::WireOutcome;
use crate::gate::GateOutcome;
use crate::lift::LiftOutcome;
use crate::walk::Canonical;

pub const FILE: &str = "rust/audit.jsonl";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    /// UTC, to the second: `2026-11-28T10:15:03Z`.
    pub time: String,
    /// The rust-anything command that made the change: `enable`, `disable`, `gate`, `app wire`,
    /// `app lift` or `go`.
    pub command: String,
    /// `on`, `off`, `gate`, `wire` or `lift`.
    pub change: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "crate", default, skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,
    /// The crate folder, relative to the repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crate_dir: Option<String>,
    /// `on`: the functions that run in Rust now. `off`: the ones back in the original.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<String>,
    /// Source files the switch edited, relative to the repository.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// Switched on with `--force`: the recorded cases were not checked first.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub forced: bool,
    /// `only` or `partial` when `enable` switched on a subset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection: Option<String>,
    /// Wired with `--skip-gate`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub skipped_gate: bool,
    /// Recorded cases matched: the last diff (`on`), gate's behavior check (`gate`), or the
    /// replay (`wire`, `lift`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cases: Option<Count>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gate: Option<GateResult>,
    /// `wire`: the commands or routes the unit answers now.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<String>,
    /// `lift`: the unit crates that became plain libraries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub units: Vec<String>,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Count {
    pub matched: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct GateResult {
    pub passed: bool,
    pub checks_passed: usize,
    pub checks: usize,
    /// The names of the checks that failed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Log {
    pub file: PathBuf,
    /// Oldest first, as appended.
    pub entries: Vec<Entry>,
    /// Lines that are not an entry (a torn write, a hand edit); skipped.
    pub unreadable: usize,
}

/// The repository's log; empty, and nothing written, when it has none. A crate folder made by
/// `bridge` reads its original's repository.
pub fn read(root: &Path) -> Result<Log> {
    if !root.exists() {
        bail!("no folder {}", root.display());
    }
    let repo = crate::rules::repo_of(root);
    let repo = repo.canonical().unwrap_or(repo);
    let file = repo.join(FILE);
    let text = match std::fs::read_to_string(&file) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e).with_context(|| format!("reading {}", file.display())),
    };
    let mut entries = Vec::new();
    let mut unreadable = 0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        match serde_json::from_str(line) {
            Ok(entry) => entries.push(entry),
            Err(_) => unreadable += 1,
        }
    }
    Ok(Log {
        file,
        entries,
        unreadable,
    })
}

/// Appends one line. A file whose last line was torn gets a newline first, so the new entry
/// stays readable; nothing already in the file changes.
pub fn append(repo: &Path, entry: &Entry) -> Result<()> {
    let path = repo.join(FILE);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("opening {}", path.display()))?;
    let mut line = serde_json::to_string(entry)?;
    line.push('\n');
    if file.metadata()?.len() > 0 {
        let mut last = [0u8];
        file.seek(SeekFrom::End(-1))?;
        file.read_exact(&mut last)?;
        if last[0] != b'\n' {
            line.insert(0, '\n');
        }
    }
    file.write_all(line.as_bytes())
        .with_context(|| format!("writing {}", path.display()))
}

/// Stamps the entry with the time, the version and the commit, and appends it. The change it
/// records has already happened, so a log that cannot be written is a warning, not an error.
fn record(repo: &Path, mut entry: Entry) {
    let repo = repo.canonical().unwrap_or_else(|_| repo.to_path_buf());
    entry.time = utc(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()));
    entry.version = env!("CARGO_PKG_VERSION").to_string();
    entry.commit = commit(&repo);
    if let Err(e) = append(&repo, &entry) {
        eprintln!("warning: the audit log was not written: {e:#}");
    }
}

fn commit(repo: &Path) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["rev-parse", "--verify", "--quiet", "HEAD"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|o| o.status.success())?;
    let hash = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (hash.len() >= 40 && hash.chars().all(|c| c.is_ascii_hexdigit())).then_some(hash)
}

/// `secs` since the Unix epoch as `YYYY-MM-DDTHH:MM:SSZ`.
fn utc(secs: u64) -> String {
    let days = i64::try_from(secs / 86_400).unwrap_or(0);
    let rem = secs % 86_400;
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3_600,
        rem % 3_600 / 60,
        rem % 60
    )
}

/// `path` relative to the repository, with `/` on every OS; `../` where it is outside.
fn relative(repo: &Path, path: &Path) -> String {
    let path = path.canonical().unwrap_or_else(|_| path.to_path_buf());
    match path.strip_prefix(repo) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => crate::shadow::relative(repo, &path),
    }
}

/// The repository, unit, crate and crate folder of a crate made by `bridge`.
fn unit_entry(crate_dir: &Path) -> Option<(PathBuf, Entry)> {
    let crate_dir = crate_dir.canonical().ok()?;
    let manifest = Manifest::load(&crate_dir).ok()?;
    let repo = manifest.root.canonical().unwrap_or(manifest.root);
    let entry = Entry {
        unit: Some(manifest.unit),
        crate_name: Some(manifest.crate_name),
        crate_dir: Some(relative(&repo, &crate_dir)),
        ..Entry::default()
    };
    Some((repo, entry))
}

/// `enable` or `disable` switched a unit. `selection` is `only` or `partial` for a subset.
pub fn switched(
    crate_dir: &Path,
    command: &str,
    switched: &Switched,
    selection: Option<&str>,
    forced: bool,
) {
    let Some((repo, base)) = unit_entry(crate_dir) else {
        return;
    };
    let entry = Entry {
        command: command.into(),
        change: if switched.on { "on" } else { "off" }.into(),
        functions: if switched.on || !switched.files.is_empty() {
            switched.functions.clone()
        } else {
            Vec::new()
        },
        files: switched.files.iter().map(|f| relative(&repo, f)).collect(),
        forced,
        selection: selection.map(Into::into),
        cases: switched
            .on
            .then(|| crate::recording::last_diff(crate_dir))
            .flatten()
            .map(|(matched, total)| Count { matched, total }),
        ..base
    };
    record(&repo, entry);
}

/// A `gate` run, wherever it ran from (`gate`, `go`, `app wire`).
pub fn gate(crate_dir: &Path, outcome: &GateOutcome) {
    let Some((repo, base)) = unit_entry(crate_dir) else {
        return;
    };
    let entry = Entry {
        command: "gate".into(),
        change: "gate".into(),
        cases: outcome
            .cases
            .map(|(matched, total)| Count { matched, total }),
        gate: Some(GateResult {
            passed: outcome.passed,
            checks_passed: outcome.checks.iter().filter(|c| c.passed).count(),
            checks: outcome.checks.len(),
            failed: outcome
                .checks
                .iter()
                .filter(|c| !c.passed)
                .map(|c| c.name.to_string())
                .collect(),
        }),
        ..base
    };
    record(&repo, entry);
}

/// `app wire` routed commands, routes or a library's API to a unit.
pub fn wired(repo: &Path, outcome: &WireOutcome, skipped_gate: bool) {
    let repo = repo.canonical().unwrap_or_else(|_| repo.to_path_buf());
    let base = unit_entry(&outcome.crate_dir).map_or_else(Entry::default, |(_, e)| e);
    let routes = if outcome.shadow.is_some() {
        vec!["the public API".to_string()]
    } else if outcome.routes.is_empty() {
        outcome.commands.clone()
    } else {
        outcome.routes.clone()
    };
    let entry = Entry {
        command: "app wire".into(),
        change: "wire".into(),
        unit: base.unit.or_else(|| Some(outcome.unit.clone())),
        crate_dir: Some(relative(&repo, &outcome.crate_dir)),
        skipped_gate,
        routes,
        cases: outcome.replay.as_ref().map(|r| Count {
            matched: r.passed,
            total: r.total,
        }),
        ..base
    };
    record(&repo, entry);
}

/// `app lift` made `rust/app` pure Rust. A refused lift changed nothing and is not logged.
pub fn lifted(repo: &Path, outcome: &LiftOutcome) {
    if !outcome.lifted {
        return;
    }
    let entry = Entry {
        command: "app lift".into(),
        change: "lift".into(),
        units: outcome.units.clone(),
        cases: outcome.replay.as_ref().map(|r| Count {
            matched: r.passed,
            total: r.total,
        }),
        ..Entry::default()
    };
    record(repo, entry);
}

fn count(c: Option<Count>) -> String {
    c.map_or_else(String::new, |c| format!("{}/{}", c.matched, c.total))
}

/// One line on what the entry changed, for `log` and the map.
pub fn summary(e: &Entry) -> String {
    let mut out = match e.change.as_str() {
        "on" => {
            let mut s = "switched on".to_string();
            if let Some(selection) = &e.selection {
                s.push_str(&format!(" (--{selection})"));
            }
            if e.forced {
                s.push_str(", FORCED");
                s.push_str(&match e.cases {
                    Some(c) => format!(", last diff {}", count(Some(c))),
                    None => ", never diffed".to_string(),
                });
            } else if e.cases.is_some() {
                s.push_str(&format!(", {} cases match", count(e.cases)));
            }
            s
        }
        "off" if e.files.is_empty() => "switched off (it was not on)".to_string(),
        "off" => "switched off".to_string(),
        "gate" => match &e.gate {
            Some(g) if g.passed => format!("gate passed, {}/{} checks", g.checks_passed, g.checks),
            Some(g) => format!(
                "gate FAILED ({}), {}/{} checks",
                g.failed.join(", "),
                g.checks_passed,
                g.checks
            ),
            None => "gate".to_string(),
        },
        "wire" => {
            let mut s = format!("wired to {}", e.routes.join(", "));
            if e.skipped_gate {
                s.push_str(", gate SKIPPED");
            }
            s
        }
        "lift" => format!("lifted to pure Rust ({} units)", e.units.len()),
        other => other.to_string(),
    };
    match e.change.as_str() {
        "gate" if e.cases.is_some() => out.push_str(&format!(", {} cases", count(e.cases))),
        "wire" | "lift" if e.cases.is_some() => {
            out.push_str(&format!(", replay {}", count(e.cases)));
        }
        _ => {}
    }
    out
}

/// `log`'s text: a line per entry, then what it switched and where it ran.
pub fn text(log: &Log) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    if log.entries.is_empty() {
        let _ = writeln!(out, "No changes logged yet in {}.", log.file.display());
    }
    for e in &log.entries {
        let _ = writeln!(
            out,
            "{}  {}  {}  {}",
            e.time,
            e.command,
            e.crate_dir.as_deref().unwrap_or("rust/app"),
            summary(e)
        );
        if !e.functions.is_empty() {
            let label = if e.change == "off" {
                "back in the original"
            } else {
                "in Rust"
            };
            let _ = writeln!(out, "    {label}: {}", e.functions.join(", "));
        }
        if !e.units.is_empty() {
            let _ = writeln!(out, "    units: {}", e.units.join(", "));
        }
        let _ = writeln!(
            out,
            "    rust-anything {}{}",
            e.version,
            e.commit.as_deref().map_or_else(String::new, |c| format!(
                ", commit {}",
                &c[..c.len().min(12)]
            ))
        );
    }
    if log.unreadable > 0 {
        let _ = writeln!(
            out,
            "{} {} in {} could not be read and {} skipped.",
            log.unreadable,
            if log.unreadable == 1 { "line" } else { "lines" },
            log.file.display(),
            if log.unreadable == 1 { "was" } else { "were" }
        );
    }
    out
}

/// The last switch of a port (`enable`, `disable`, `app wire`, `app lift`, `go`), for the map.
/// Gate results stay in the log: a failed gate does not switch anything off.
pub fn last_switch<'a>(
    entries: &'a [Entry],
    crate_dir: &str,
    crate_name: &str,
) -> Option<&'a Entry> {
    entries.iter().rev().find(|e| {
        e.change != "gate"
            && (e.crate_dir.as_deref() == Some(crate_dir)
                || e.units.iter().any(|u| u == crate_name))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_is_iso_8601_to_the_second() {
        assert_eq!(utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(utc(1_700_000_000), "2023-11-14T22:13:20Z");
        assert_eq!(utc(4_107_542_399), "2100-02-28T23:59:59Z");
    }

    #[test]
    fn a_torn_line_is_closed_before_the_next_entry_and_read_skips_it() {
        let dir = crate::app::temp_dir("audit-torn").unwrap();
        let entry = Entry {
            command: "gate".into(),
            change: "gate".into(),
            version: "1.0.0".into(),
            ..Entry::default()
        };
        append(&dir, &entry).unwrap();
        let mut text = std::fs::read_to_string(dir.join(FILE)).unwrap();
        text.push_str("{\"time\":");
        std::fs::write(dir.join(FILE), &text).unwrap();
        append(&dir, &entry).unwrap();
        let after = std::fs::read_to_string(dir.join(FILE)).unwrap();
        assert!(after.starts_with(&text), "{after}");
        let log = read(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(log.entries, [entry.clone(), entry]);
        assert_eq!(log.unreadable, 1);
    }

    #[test]
    fn gate_entries_name_the_failed_checks_and_count_cases() {
        let failed = Entry {
            change: "gate".into(),
            cases: Some(Count {
                matched: 880,
                total: 900,
            }),
            gate: Some(GateResult {
                passed: false,
                checks_passed: 12,
                checks: 14,
                failed: vec!["behavior".into(), "literals".into()],
            }),
            ..Entry::default()
        };
        assert_eq!(
            summary(&failed),
            "gate FAILED (behavior, literals), 12/14 checks, 880/900 cases"
        );
        let json = serde_json::to_value(&failed).unwrap();
        assert_eq!(
            json["gate"]["failed"],
            serde_json::json!(["behavior", "literals"])
        );
        assert_eq!(
            json["cases"],
            serde_json::json!({ "matched": 880, "total": 900 })
        );
        assert!(json.get("forced").is_none());
    }

    #[test]
    fn a_forced_switch_says_so_and_the_map_skips_gates() {
        let on = Entry {
            change: "on".into(),
            crate_dir: Some("rust/a-rs".into()),
            forced: true,
            cases: Some(Count {
                matched: 7,
                total: 10,
            }),
            ..Entry::default()
        };
        assert_eq!(summary(&on), "switched on, FORCED, last diff 7/10");
        let gate = Entry {
            change: "gate".into(),
            crate_dir: Some("rust/a-rs".into()),
            ..Entry::default()
        };
        let lift = Entry {
            change: "lift".into(),
            units: vec!["b-rs".into()],
            ..Entry::default()
        };
        let entries = [on.clone(), gate, lift.clone()];
        assert_eq!(last_switch(&entries, "rust/a-rs", "a-rs"), Some(&on));
        assert_eq!(last_switch(&entries, "rust/b-rs", "b-rs"), Some(&lift));
        assert_eq!(last_switch(&entries, "rust/c-rs", "c-rs"), None);
    }
}
