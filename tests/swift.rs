//! Swift units: bridge, record, build, diff and gate on a SwiftPM target's top-level functions,
//! through `swiftc` (skipped without it).

mod common;

use std::path::{Path, PathBuf};

use rust_anything::bridge::{self, Kind, Manifest};
use rust_anything::harness::{self, DiffOptions, RecordOptions};

const SIGNATURES: [&str; 5] = [
    "pub fn add_counts(a: i64, b: i64) -> Result<i64, String>",
    "pub fn visible_length(text: String) -> Result<i64, String>",
    "pub fn parse_score(text: String, limit: i64) -> Result<i64, String>",
    "pub fn average(values: Vec<f64>) -> Result<Option<f64>, String>",
    "pub fn word_counts(text: String, stop: Vec<String>) -> Result<serde_json::Value, String>",
];

const PORT: &str = r#"//! The Rust port of `TextKit.swift`.

#![forbid(unsafe_code)]

use serde_json::{Map, Value};
use unicode_segmentation::UnicodeSegmentation;

pub fn add_counts(a: i64, b: i64) -> Result<i64, String> {
    Ok(a + b)
}

pub fn visible_length(text: String) -> Result<i64, String> {
    Ok(text.graphemes(true).count() as i64)
}

pub fn parse_score(text: String, limit: i64) -> Result<i64, String> {
    if text.is_empty() {
        return Err("ScoreError: empty".into());
    }
    let Ok(n) = text.parse::<i64>() else {
        return Err("ScoreError: notANumber".into());
    };
    if n < 0 || n > limit {
        return Err(format!("ScoreError: outOfRange({n})"));
    }
    Ok(n)
}

pub fn average(values: Vec<f64>) -> Result<Option<f64>, String> {
    if values.is_empty() {
        return Ok(None);
    }
    let mut sum = 0.0;
    for v in &values {
        sum += v;
    }
    Ok(Some(sum / values.len() as f64))
}

pub fn word_counts(text: String, stop: Vec<String>) -> Result<Value, String> {
    let mut counts = Map::new();
    for word in text.split(' ').filter(|w| !w.is_empty()) {
        // Swift lower-cases each character on its own: no final sigma.
        let key: String = word.chars().flat_map(char::to_lowercase).collect();
        if !stop.contains(&key) {
            let n = counts.get(&key).and_then(Value::as_i64).unwrap_or(0);
            counts.insert(key, Value::from(n + 1));
        }
    }
    Ok(Value::Object(counts))
}
"#;

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/swift-package")
}

fn options<'a>(root: &'a Path, out: &Path, batch: Vec<String>) -> bridge::Options<'a> {
    bridge::Options {
        root,
        unit: "Sources/TextKit/TextKit.swift",
        kind: None,
        out: Some(out.to_path_buf()),
        name: Some("textkit-rs".into()),
        force: false,
        keep: Vec::new(),
        lang: None,
        batch,
        as_json: Vec::new(),
        allocator: Default::default(),
    }
}

fn diff(dir: &Path) -> harness::DiffOutcome {
    harness::diff(&DiffOptions {
        crate_dir: dir,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap()
}

fn with_segmentation(dir: &Path) {
    let path = dir.join("Cargo.toml");
    let toml = std::fs::read_to_string(&path).unwrap();
    if !toml.contains("unicode-segmentation") {
        std::fs::write(
            &path,
            toml.replace(
                "[dependencies]\n",
                "[dependencies]\nunicode-segmentation = \"1\"\n",
            ),
        )
        .unwrap();
    }
}

fn port(dir: &Path, text: &str) {
    std::fs::write(dir.join("src/port.rs"), text).unwrap();
    harness::build(dir).unwrap();
}

#[test]
fn a_target_is_bridged_with_its_labels_and_throws() {
    let dir =
        std::env::temp_dir().join(format!("rust-anything-swift-bridge-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let batch = bridge::bridge(&options(&fixture(), &dir, vec!["average:values".into()]))
        .unwrap_err()
        .to_string();
    assert!(batch.contains("not available for Swift"), "{batch}");
    let outcome = bridge::bridge(&options(&fixture(), &dir, Vec::new())).unwrap();
    assert_eq!(outcome.kind, Kind::Ffi);
    let manifest = Manifest::load(&dir).unwrap();
    assert_eq!(manifest.language, rust_anything::lang::Lang::Swift);
    let kept: Vec<(&str, &str)> = manifest
        .js_only
        .iter()
        .map(|j| (j.name.as_str(), j.reason.as_str()))
        .collect();
    assert_eq!(kept, [("applyTwice", "takes or returns a closure")]);
    let port = std::fs::read_to_string(dir.join("src/port.rs")).unwrap();
    for signature in SIGNATURES {
        assert!(port.contains(signature), "{signature}\n{port}");
    }
    let cargo = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(cargo.contains("overflow-checks = true"), "{cargo}");
    let wrapper = std::fs::read_to_string(dir.join("swift/TextkitRs.swift")).unwrap();
    assert!(
        wrapper.contains(
            "public func parseScore(_ text: String, max limit: Int = 100) throws -> Int {"
        ),
        "{wrapper}"
    );
    assert!(!wrapper.contains("func applyTwice"), "{wrapper}");
    assert!(
        dir.join("swift/TextkitRsNative/module.modulemap").is_file(),
        "the wrapper's C module"
    );
    let refused = rust_anything::shadow::shadow(&dir, None)
        .unwrap_err()
        .to_string();
    assert!(refused.contains("no shadow module for Swift"), "{refused}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_target_records_diffs_and_gates() {
    if !common::have("swiftc") {
        return;
    }
    let dir = std::env::temp_dir().join(format!("rust-anything-swift-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    bridge::bridge(&options(&fixture(), &dir, Vec::new())).unwrap();

    let inputs = dir.join("inputs.jsonl");
    std::fs::write(
        &inputs,
        "{\"fn\": \"addCounts\", \"args\": [9223372036854775807, 1]}\n\
         {\"fn\": \"addCounts\", \"args\": [2, 3]}\n\
         {\"fn\": \"visibleLength\", \"args\": [\"👍🏽\"]}\n\
         {\"fn\": \"visibleLength\", \"args\": [\"e\\u0301\"]}\n\
         {\"fn\": \"visibleLength\", \"args\": [\"🇹🇷 ok\"]}\n\
         {\"fn\": \"parseScore\", \"args\": [\"\", 100]}\n\
         {\"fn\": \"parseScore\", \"args\": [\"x1\", 100]}\n\
         {\"fn\": \"parseScore\", \"args\": [\"150\", 100]}\n\
         {\"fn\": \"parseScore\", \"args\": [\"+42\", 100]}\n\
         {\"fn\": \"average\", \"args\": [[]]}\n\
         {\"fn\": \"average\", \"args\": [[1.5, 2.5, 4]]}\n\
         {\"fn\": \"wordCounts\", \"args\": [\"To be or not to be\", [\"or\"]]}\n",
    )
    .unwrap();
    let recorded = harness::record(&RecordOptions {
        crate_dir: &dir,
        count: 20,
        inputs: Some(&inputs),
        module: None,
        resolve_from: None,
        node_args: &[],
        sandbox: false,
    })
    .unwrap();
    assert!(recorded.cases > 50, "{recorded:?}");
    let cases = std::fs::read_to_string(dir.join("cases.jsonl")).unwrap();
    for line in [
        r#"{"fn":"addCounts","args":[2,3],"ok":5}"#,
        r#"{"fn":"visibleLength","args":["👍🏽"],"ok":1}"#,
        "{\"fn\":\"visibleLength\",\"args\":[\"e\u{301}\"],\"ok\":1}",
        r#"{"fn":"visibleLength","args":["🇹🇷 ok"],"ok":4}"#,
        r#"{"fn":"parseScore","args":["",100],"error":"empty","error_type":"ScoreError"}"#,
        r#"{"fn":"parseScore","args":["x1",100],"error":"notANumber","error_type":"ScoreError"}"#,
        r#"{"fn":"parseScore","args":["150",100],"error":"outOfRange(150)","error_type":"ScoreError"}"#,
        r#"{"fn":"parseScore","args":["+42",100],"ok":42}"#,
        r#"{"fn":"average","args":[[]],"ok":null}"#,
        r#"{"fn":"average","args":[[1.5,2.5,4]],"ok":2.6666666666666665}"#,
        r#"{"fn":"wordCounts","args":["To be or not to be",["or"]],"ok":{"be":2,"not":1,"to":2}}"#,
    ] {
        assert!(cases.lines().any(|l| l == line), "{line}\n{cases}");
    }
    let trap = cases
        .lines()
        .find(|l| l.starts_with(r#"{"fn":"addCounts","args":[9223372036854775807,1]"#))
        .unwrap_or_else(|| panic!("the overflow is recorded\n{cases}"));
    assert!(trap.ends_with(r#""error_type":"trap"}"#), "{trap}");

    with_segmentation(&dir);
    port(&dir, PORT);
    let outcome = diff(&dir);
    assert_eq!(outcome.total, recorded.cases);
    assert_eq!(outcome.passed, outcome.total, "{:?}", outcome.mismatches);

    let gate = rust_anything::gate::gate(&dir, &[]).unwrap();
    assert!(gate.passed, "{:?}", gate.checks);

    port(
        &dir,
        &PORT.replace("text.graphemes(true).count()", "text.chars().count()"),
    );
    let scalars = diff(&dir);
    assert!(scalars.failed > 0);
    assert!(
        scalars
            .mismatches
            .iter()
            .all(|m| m["fn"].as_str() == Some("visibleLength")),
        "{:?}",
        scalars.mismatches
    );

    port(&dir, &PORT.replace("Ok(a + b)", "Ok(a.wrapping_add(b))"));
    let wrapped = diff(&dir);
    assert!(wrapped.failed > 0);
    assert!(
        wrapped
            .mismatches
            .iter()
            .all(|m| m["fn"].as_str() == Some("addCounts")
                && m["expected"]["error_type"].as_str() == Some("trap")),
        "{:?}",
        wrapped.mismatches
    );
    let _ = std::fs::remove_dir_all(&dir);
}
