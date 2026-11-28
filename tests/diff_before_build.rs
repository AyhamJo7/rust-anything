//! `diff`, `bench` and `fuzz` on an unbuilt crate stop before starting node, python or go,
//! with `no built library in <dir> (run `rust-anything build <crate>` first)`.

mod common;

use std::path::{Path, PathBuf};

use rust_anything::bridge::{self, Kind};
use rust_anything::fuzz::{self, FuzzOptions};
use rust_anything::harness::{self, BenchOptions, DiffOptions, RecordOptions};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini")
}

fn bridged_ts(name: &str, kind: Kind) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rust-anything-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    bridge::bridge(&bridge::Options {
        root: &fixture(),
        unit: "packages/a/src/util.ts",
        kind: Some(kind),
        out: Some(dir.clone()),
        name: None,
        force: false,
        keep: Vec::new(),
        lang: None,
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })
    .unwrap();
    dir
}

#[test]
fn unbuilt_ts_napi_crate_reports_run_build_first() {
    if !common::have("node") {
        return;
    }
    let dir = bridged_ts("unbuilt-napi", Kind::Napi);
    harness::record(&RecordOptions {
        crate_dir: &dir,
        count: 2,
        inputs: None,
        module: None,
        resolve_from: None,
        node_args: &[],
        sandbox: false,
    })
    .unwrap();

    let expected = format!(
        "no built library in {} (run `rust-anything build {}` first)",
        dir.display(),
        dir.display()
    );

    let diff_err = harness::diff(&DiffOptions {
        crate_dir: &dir,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(diff_err.to_string(), expected);

    let bench_err = harness::bench(&BenchOptions {
        crate_dir: &dir,
        min_ms: 1,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(bench_err.to_string(), expected);

    let fuzz_err = fuzz::fuzz(&FuzzOptions {
        crate_dir: &dir,
        cases: 2,
        seed: Some(1),
        time: None,
        keep: None,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(fuzz_err.to_string(), expected);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn unbuilt_ts_wasm_crate_reports_run_build_first() {
    let dir = bridged_ts("unbuilt-wasm", Kind::Wasm);
    std::fs::write(dir.join(harness::CASES), "{\"fn\":\"test\",\"args\":[]}\n").unwrap();

    let expected = format!(
        "no built library in {} (run `rust-anything build {}` first)",
        dir.join("pkg").display(),
        dir.display()
    );

    let diff_err = harness::diff(&DiffOptions {
        crate_dir: &dir,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(diff_err.to_string(), expected);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn unbuilt_python_crate_reports_run_build_first() {
    let dir = std::env::temp_dir().join(format!("rust-anything-unbuilt-py-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = serde_json::json!({
        "schema": "rust-anything/bridge@1",
        "root": ".",
        "unit": "test.py",
        "language": "python",
        "kind": "pyo3",
        "crate_name": "test-rs",
        "exports": []
    });
    std::fs::write(
        dir.join(rust_anything::bridge::MANIFEST),
        manifest.to_string(),
    )
    .unwrap();
    std::fs::write(dir.join(harness::CASES), "{\"fn\":\"test\",\"args\":[]}\n").unwrap();

    let expected = format!(
        "no built library in {} (run `rust-anything build {}` first)",
        dir.join("python").join("test_rs").display(),
        dir.display()
    );

    let diff_err = harness::diff(&DiffOptions {
        crate_dir: &dir,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(diff_err.to_string(), expected);

    let bench_err = harness::bench(&BenchOptions {
        crate_dir: &dir,
        min_ms: 1,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(bench_err.to_string(), expected);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn unbuilt_go_crate_reports_run_build_first() {
    let dir = std::env::temp_dir().join(format!("rust-anything-unbuilt-go-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = serde_json::json!({
        "schema": "rust-anything/bridge@1",
        "root": ".",
        "unit": "test.go",
        "language": "go",
        "kind": "c-abi",
        "crate_name": "test-rs",
        "exports": []
    });
    std::fs::write(
        dir.join(rust_anything::bridge::MANIFEST),
        manifest.to_string(),
    )
    .unwrap();
    std::fs::write(dir.join(harness::CASES), "{\"fn\":\"test\",\"args\":[]}\n").unwrap();

    let expected = format!(
        "no built library in {} (run `rust-anything build {}` first)",
        dir.join("target/release").display(),
        dir.display()
    );

    let diff_err = harness::diff(&DiffOptions {
        crate_dir: &dir,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(diff_err.to_string(), expected);

    let bench_err = harness::bench(&BenchOptions {
        crate_dir: &dir,
        min_ms: 1,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap_err();
    assert_eq!(bench_err.to_string(), expected);

    let _ = std::fs::remove_dir_all(&dir);
}
