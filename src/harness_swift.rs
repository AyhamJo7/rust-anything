//! The Swift side of record, build and diff. A generated `main.swift` driver is compiled by
//! `swiftc` with the unit's target sources to record, and with the generated wrapper over the
//! Rust library to diff. Swift stops the whole program on a trap (an overflow, `!` on nil), so
//! the driver runs in a loop: a case that stops it is kept as a trap, and the driver starts
//! again on the next case.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value, json};

use crate::bridge::Manifest;
use crate::harness::{BenchRow, CASES, DiffOutcome};
use crate::ts::discover::FileKind;

const WORK_DIR: &str = ".rust-anything";

/// The `error_type` of a case that stopped the program.
pub const TRAP: &str = "trap";

fn swiftc() -> String {
    std::env::var("SWIFTC").unwrap_or_else(|_| "swiftc".into())
}

fn run(cmd: &mut Command, what: &str) -> Result<()> {
    let _scratch = crate::harness::scratch_cwd(cmd)?;
    let output = cmd
        .output()
        .map_err(|e| crate::process::spawn_error(cmd, what, e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let tail: Vec<&str> = stderr
            .lines()
            .filter(|l| !l.trim().is_empty())
            .rev()
            .take(30)
            .collect();
        bail!(
            "{what} failed ({}):\n{}",
            output.status,
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        );
    }
    Ok(())
}

/// Whether a Swift file declares the program's entry point, which the driver's own top-level
/// code would clash with.
fn is_entry(path: &Path) -> bool {
    path.file_name().is_some_and(|n| n == "main.swift")
        || std::fs::read_to_string(path)
            .is_ok_and(|t| t.lines().any(|l| l.trim_start().starts_with("@main")))
}

/// The folder of the target a file belongs to: `Sources/<Target>` in a SwiftPM package, else
/// the file's own folder.
fn target_dir(file: &str) -> String {
    let parts: Vec<&str> = file.split('/').collect();
    if let Some(i) = parts.iter().position(|p| *p == "Sources")
        && i + 2 < parts.len()
    {
        return parts[..i + 2].join("/");
    }
    file.rsplit_once('/')
        .map_or(String::new(), |(d, _)| d.to_string())
}

/// The non-test, non-UI sources of the targets holding the unit's exports, without entry points.
fn sources(m: &Manifest) -> Result<Vec<PathBuf>> {
    let ws = crate::lang::swift::discover(&m.root)?;
    let mut targets: Vec<String> = m
        .exports
        .iter()
        .map(|f| target_dir(&f.file))
        .chain(m.js_only.iter().map(|j| target_dir(&j.file)))
        .collect();
    targets.sort();
    targets.dedup();
    let mut files: Vec<PathBuf> = ws
        .files
        .iter()
        .filter(|f| f.kind == FileKind::Source)
        .filter(|f| {
            targets.iter().any(|t| {
                if t.is_empty() {
                    !f.path.contains('/')
                } else {
                    f.path.starts_with(&format!("{t}/"))
                }
            })
        })
        .map(|f| m.root.join(&f.path))
        .filter(|p| !is_entry(p))
        .collect();
    files.sort();
    Ok(files)
}

fn swift_dir(crate_dir: &Path) -> PathBuf {
    crate_dir.join("swift")
}

pub fn built_file(crate_dir: &Path, m: &Manifest) -> Option<PathBuf> {
    let lib = m.lib_name();
    [
        format!("lib{lib}.dylib"),
        format!("lib{lib}.so"),
        format!("{lib}.dll"),
    ]
    .into_iter()
    .map(|n| swift_dir(crate_dir).join(n))
    .find(|p| p.exists())
}

pub fn build(crate_dir: &Path, m: &Manifest) -> Result<PathBuf> {
    let lib = crate::harness_c::build(crate_dir, m)?;
    let name = lib.file_name().context("library has no file name")?;
    std::fs::create_dir_all(swift_dir(crate_dir))?;
    let target = swift_dir(crate_dir).join(name);
    std::fs::copy(&lib, &target)?;
    Ok(target)
}

/// Compiles the driver, with the originals or with the wrapper over the Rust library.
fn prepare(crate_dir: &Path, m: &Manifest, rust: bool) -> Result<PathBuf> {
    let dir = crate_dir
        .join(WORK_DIR)
        .join(if rust { "swift-rust" } else { "swift-original" });
    std::fs::create_dir_all(&dir)?;
    let main = dir.join("main.swift");
    std::fs::write(&main, crate::codegen_swift::driver(m, rust))?;
    let exe = dir.join(if cfg!(windows) {
        "driver.exe"
    } else {
        "driver"
    });
    let mut cmd = Command::new(swiftc());
    cmd.args(["-O", "-module-name", crate::codegen_swift::DRIVER_MODULE]);
    if rust {
        let wrapper = crate_dir
            .join(crate::codegen_swift::wrapper_path(m))
            .canonicalize()
            .context("the Swift wrapper is missing (run `rust-anything refresh`)")?;
        let swift = swift_dir(crate_dir).canonicalize()?;
        cmd.arg(wrapper)
            .arg("-I")
            .arg(&swift)
            .arg("-L")
            .arg(&swift)
            .args(["-Xlinker", "-rpath", "-Xlinker"])
            .arg(&swift);
    } else {
        let files = sources(m)?;
        if files.is_empty() {
            bail!("no Swift sources to compile for `{}`", m.unit);
        }
        cmd.args(files);
    }
    cmd.arg(&main).arg("-o").arg(&exe);
    run(
        &mut cmd,
        if rust {
            "swiftc (driver with the Rust wrapper)"
        } else {
            "swiftc (driver with the original)"
        },
    )?;
    Ok(exe)
}

/// What the driver made of one case.
enum Ran {
    /// The result fields: `ok`, or `error` and `error_type`.
    Fields(Map<String, Value>),
    /// The arguments do not convert to the parameter types.
    Skipped,
    /// The case stopped the program.
    Trap(String),
    TimedOut,
}

fn trap_message(status: ExitStatus, stderr: &str) -> String {
    if let Some(line) = stderr.lines().rev().find(|l| l.contains("Fatal error")) {
        let at = line.find("Fatal error").unwrap_or(0);
        return line[at..].trim().to_string();
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt as _;
        if let Some(signal) = status.signal() {
            return format!("stopped by signal {signal}");
        }
    }
    format!("stopped ({status})")
}

/// Runs the driver over `cases`, again from the next case each time one stops it or runs past
/// the time limit.
fn run_cases(
    exe: &Path,
    cases: &[Value],
    work: &Path,
    what: &str,
    sandbox: Option<&Path>,
) -> Result<Vec<Ran>> {
    std::fs::create_dir_all(work)?;
    let timeout = crate::harness::case_timeout();
    let inputs = work.join("swift-inputs.json");
    let part = work.join("swift-part.jsonl");
    let progress = work.join("swift-progress");
    let stderr_path = work.join("swift-stderr");
    let mut results: Vec<Option<Ran>> = (0..cases.len()).map(|_| None).collect();
    let mut start = 0;
    while start < cases.len() {
        std::fs::write(&inputs, serde_json::to_string(&cases[start..])?)?;
        std::fs::write(&progress, "")?;
        let _ = std::fs::remove_file(&part);
        let mut cmd = Command::new(exe);
        cmd.arg(&inputs)
            .arg(&part)
            .env("RUST_ANYTHING_PROGRESS", &progress);
        let scratch = crate::harness::scratch_cwd(&mut cmd)?;
        if let Some(read_only) = sandbox {
            cmd = crate::process::Sandbox {
                read_only,
                scratch: scratch.as_ref().map_or(work, |s| s.path()),
                writable: &[work],
                loopback: false,
            }
            .wrap(&cmd, true)?;
        }
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(std::fs::File::create(&stderr_path)?);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt as _;
            cmd.process_group(0);
        }
        let mut child = cmd
            .spawn()
            .map_err(|e| crate::process::spawn_error(&cmd, what, e))?;
        let started = || {
            std::fs::read_to_string(&progress).ok().and_then(|p| {
                p.lines()
                    .last()
                    .and_then(|l| l.trim().parse::<usize>().ok())
            })
        };
        let mut current: Option<(usize, Instant)> = None;
        let status = loop {
            if let Some(status) = child.try_wait()? {
                break Some(status);
            }
            match (started(), current) {
                (Some(i), Some((j, _))) if i == j => {}
                (Some(i), _) => current = Some((i, Instant::now())),
                (None, _) => {}
            }
            if let Some((_, since)) = current
                && since.elapsed() > timeout
            {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        };
        let written = std::fs::read_to_string(&part).unwrap_or_default();
        for line in written.lines() {
            let Ok(Value::Object(mut fields)) = crate::harness::parse_deep(line) else {
                continue;
            };
            let Some(i) = fields.shift_remove("i").and_then(|i| i.as_u64()) else {
                continue;
            };
            let Some(slot) = usize::try_from(i)
                .ok()
                .and_then(|i| results.get_mut(start + i))
            else {
                continue;
            };
            *slot = Some(if fields.contains_key("skip") {
                Ran::Skipped
            } else {
                Ran::Fields(fields)
            });
        }
        let at = match status {
            Some(_) => started().map(|i| start + i),
            None => current.map(|(i, _)| start + i),
        };
        match (status, at) {
            (Some(status), _) if status.success() => break,
            (Some(status), Some(at)) if results[at].is_none() => {
                let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
                results[at] = Some(Ran::Trap(trap_message(status, &stderr)));
                start = at + 1;
            }
            (Some(status), _) => {
                let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
                let tail: Vec<&str> = stderr.lines().rev().take(25).collect();
                bail!(
                    "{what} failed ({status}):\n{}",
                    tail.into_iter().rev().collect::<Vec<_>>().join("\n")
                );
            }
            (None, Some(at)) => {
                results[at] = Some(Ran::TimedOut);
                start = at + 1;
            }
            (None, None) => unreachable!("the watchdog only stops a driver inside a case"),
        }
    }
    Ok(results
        .into_iter()
        .map(|r| r.unwrap_or(Ran::Skipped))
        .collect())
}

fn trap_fields(message: &str) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("error".into(), Value::from(message));
    fields.insert("error_type".into(), Value::from(TRAP));
    fields
}

pub fn record(
    crate_dir: &Path,
    m: &Manifest,
    cases: &[Value],
    sandbox: Option<&Path>,
) -> Result<PathBuf> {
    let exe = prepare(crate_dir, m, false)?;
    let work = crate_dir.join(WORK_DIR);
    let ran = run_cases(&exe, cases, &work, "Swift driver (record)", sandbox)?;
    let mut lines = String::new();
    let mut timed_out = Vec::new();
    let mut recorded = std::collections::HashSet::new();
    for (case, ran) in cases.iter().zip(ran) {
        let fields = match ran {
            Ran::Fields(fields) => fields,
            Ran::Trap(message) => trap_fields(&message),
            Ran::Skipped => continue,
            Ran::TimedOut => {
                timed_out.push(case.clone());
                continue;
            }
        };
        let mut line = Map::new();
        line.insert("fn".into(), case["fn"].clone());
        line.insert(
            "args".into(),
            case.get("args").cloned().unwrap_or_else(|| json!([])),
        );
        line.extend(fields);
        if let Some(name) = case["fn"].as_str() {
            recorded.insert(name.to_string());
        }
        lines.push_str(&serde_json::to_string(&Value::Object(line))?);
        lines.push('\n');
    }
    std::fs::write(
        work.join(crate::harness::TIMED_OUT),
        serde_json::to_string(&timed_out)?,
    )?;
    let mut asked: Vec<&str> = cases.iter().filter_map(|c| c["fn"].as_str()).collect();
    asked.sort_unstable();
    asked.dedup();
    if let Some(name) = asked.into_iter().find(|name| !recorded.contains(*name)) {
        bail!(
            "no case of {name} was recorded: none of its inputs converts to the parameter types \
             (pass cases that fit with `--inputs`); kept the previous {CASES}"
        );
    }
    let out = crate_dir.join(CASES);
    std::fs::write(&out, lines).with_context(|| format!("writing {}", out.display()))?;
    Ok(out)
}

/// The fields of a recorded case that `diff` compares.
fn expected(case: &Value) -> Map<String, Value> {
    ["ok", "error", "error_type"]
        .iter()
        .filter_map(|k| case.get(*k).map(|v| ((*k).to_string(), v.clone())))
        .collect()
}

fn same(expected: &Map<String, Value>, actual: &Map<String, Value>) -> bool {
    match expected.get("error_type").and_then(Value::as_str) {
        Some(TRAP) => actual.get("error_type").and_then(Value::as_str) == Some(TRAP),
        Some(_) => {
            actual.get("error_type") == expected.get("error_type")
                && actual.get("error") == expected.get("error")
        }
        None => !actual.contains_key("error") && actual.get("ok") == expected.get("ok"),
    }
}

pub fn diff(crate_dir: &Path, m: &Manifest) -> Result<DiffOutcome> {
    let path = crate_dir.join(CASES);
    if !path.exists() {
        bail!(
            "no {CASES} in {} (run `rust-anything record` first)",
            crate_dir.display()
        );
    }
    if built_file(crate_dir, m).is_none() {
        bail!(
            "no built library in {} (run `rust-anything build {}` first)",
            swift_dir(crate_dir).display(),
            crate_dir.display()
        );
    }
    let cases: Vec<Value> = std::fs::read_to_string(&path)?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(crate::harness::parse_deep)
        .collect::<serde_json::Result<_>>()
        .with_context(|| format!("reading {}", path.display()))?;
    let exe = prepare(crate_dir, m, true)?;
    let ran = run_cases(
        &exe,
        &cases,
        &crate_dir.join(WORK_DIR).join("swift-diff"),
        "Swift driver (diff)",
        None,
    )?;
    let mut outcome = DiffOutcome {
        total: cases.len(),
        ..Default::default()
    };
    for (case, ran) in cases.iter().zip(ran) {
        let actual = match ran {
            Ran::Fields(fields) => fields,
            Ran::Trap(message) => trap_fields(&message),
            Ran::Skipped => trap_fields("the recorded arguments do not convert any more"),
            Ran::TimedOut => {
                let mut fields = Map::new();
                fields.insert("error".into(), Value::from("ran past the time limit"));
                fields
            }
        };
        let expected = expected(case);
        if same(&expected, &actual) {
            outcome.passed += 1;
        } else {
            outcome.failed += 1;
            outcome.mismatches.push(json!({
                "fn": case["fn"],
                "args": case["args"],
                "expected": expected,
                "actual": actual,
            }));
        }
    }
    Ok(outcome)
}

pub fn bench(_crate_dir: &Path, _m: &Manifest, _min_ms: u64) -> Result<Vec<BenchRow>> {
    bail!(
        "`bench` does not time Swift units yet; `diff` checks them. Time the wrapper in your own \
         Swift benchmark (each call crosses as JSON)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_target_is_its_sources_folder() {
        assert_eq!(target_dir("Sources/Geo/Shapes/Area.swift"), "Sources/Geo");
        assert_eq!(target_dir("pkg/Sources/Geo/Area.swift"), "pkg/Sources/Geo");
        assert_eq!(target_dir("lib/Text.swift"), "lib");
        assert_eq!(target_dir("Text.swift"), "");
    }

    #[test]
    fn traps_match_traps_and_errors_match_type_and_message() {
        let fields = |v: Value| v.as_object().unwrap().clone();
        let trap = fields(json!({"error": "stopped by signal 5", "error_type": "trap"}));
        let panic =
            fields(json!({"error": "Fatal error: Rust panic: overflow", "error_type": "trap"}));
        assert!(same(&trap, &panic));
        let thrown = fields(json!({"error": "empty", "error_type": "ParseError"}));
        assert!(same(&thrown, &thrown.clone()));
        let other = fields(json!({"error": "tooLong", "error_type": "ParseError"}));
        assert!(!same(&thrown, &other));
        assert!(!same(&thrown, &trap));
        let ok = fields(json!({"ok": 3}));
        assert!(same(&ok, &fields(json!({"ok": 3}))));
        assert!(!same(&ok, &fields(json!({"ok": 4}))));
        assert!(!same(&ok, &trap));
    }
}
