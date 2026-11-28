//! `record --sandbox` and `app record --sandbox`: the original runs with no network and a
//! read-only source tree. Each test records the same original with and without the flag; it is
//! skipped where the sandbox tool (bubblewrap on Linux, sandbox-exec on macOS) cannot run.

mod common;

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CLI: &str = env!("CARGO_BIN_EXE_rust-anything");

fn sandbox_runs() -> bool {
    match rust_anything::process::sandbox_ready() {
        Ok(_) => true,
        Err(why) => {
            eprintln!("skipped: no sandbox here ({why})");
            false
        }
    }
}

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rust-anything-sandbox-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    for (path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    root.canonicalize().unwrap()
}

fn cli(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(CLI).args(args).output().unwrap()
}

fn ok(out: Output) -> String {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{text}");
    text
}

const PROBE_PY: &str = r#"import os
import socket


def ping(port: int) -> str:
    socket.create_connection(("127.0.0.1", port), timeout=5).close()
    return "connected"


def touch(name: str) -> str:
    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), name), "w") as f:
        f.write("x")
    return "wrote"
"#;

/// What `cases.jsonl` holds for `function`: its `ok` value, or `error: ...`.
fn outcome(crate_dir: &Path, function: &str) -> String {
    let text = std::fs::read_to_string(crate_dir.join("cases.jsonl")).unwrap();
    let case: serde_json::Value = text
        .lines()
        .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap())
        .find(|c| c["fn"] == function)
        .unwrap_or_else(|| panic!("no case for {function} in {text}"));
    match case.get("error") {
        Some(e) => format!("error: {e}"),
        None => case["ok"].as_str().unwrap_or_default().to_string(),
    }
}

#[test]
fn a_sandboxed_unit_recording_has_no_network_and_cannot_write_the_source() {
    if !common::have("python3") || !sandbox_runs() {
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let root = project(
        "unit",
        &[
            (
                "pyproject.toml",
                "[project]\nname = \"probe\"\nversion = \"0.1.0\"\n",
            ),
            ("probe.py", PROBE_PY),
        ],
    );
    let inputs = root.join("inputs.jsonl");
    std::fs::write(
        &inputs,
        format!(
            "{{\"fn\": \"ping\", \"args\": [{port}]}}\n{{\"fn\": \"touch\", \"args\": [\"marker.txt\"]}}\n"
        ),
    )
    .unwrap();
    let crate_dir = root.join("port");
    ok(cli(&[
        "bridge".as_ref(),
        root.as_os_str(),
        "probe.py".as_ref(),
        "--out".as_ref(),
        crate_dir.as_os_str(),
    ]));
    let record = |sandbox: bool| {
        let mut args: Vec<&std::ffi::OsStr> = vec![
            "record".as_ref(),
            crate_dir.as_os_str(),
            "--count".as_ref(),
            "0".as_ref(),
            "--inputs".as_ref(),
            inputs.as_os_str(),
        ];
        if sandbox {
            args.push("--sandbox".as_ref());
        }
        ok(cli(&args))
    };

    record(false);
    assert_eq!(outcome(&crate_dir, "ping"), "connected");
    assert_eq!(outcome(&crate_dir, "touch"), "wrote");
    assert!(root.join("marker.txt").is_file());
    std::fs::remove_file(root.join("marker.txt")).unwrap();

    record(true);
    let ping = outcome(&crate_dir, "ping");
    let touch = outcome(&crate_dir, "touch");
    let wrote = root.join("marker.txt").exists();
    drop(listener);
    std::fs::remove_dir_all(&root).unwrap();
    assert!(ping.starts_with("error:"), "{ping}");
    assert!(touch.starts_with("error:"), "{touch}");
    assert!(!wrote, "the sandboxed original wrote into the source tree");
}

const PROBE_CLI: &str = r#"import os
import socket
import sys

try:
    if sys.argv[1] == "ping":
        socket.create_connection(("127.0.0.1", int(sys.argv[2])), timeout=5).close()
    else:
        with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "marker.txt"), "w") as f:
            f.write("x")
    print("done")
except OSError:
    print("blocked")
"#;

#[test]
fn a_sandboxed_app_recording_has_no_network_and_cannot_write_the_source() {
    if !common::have("python3") || !sandbox_runs() {
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let root = project("app", &[("probe.py", PROBE_CLI)]);
    let inputs = root.join("inputs.jsonl");
    std::fs::write(
        &inputs,
        format!(
            "{{\"name\": \"ping\", \"args\": [\"ping\", \"{port}\"]}}\n{{\"name\": \"touch\", \"args\": [\"touch\"]}}\n"
        ),
    )
    .unwrap();
    let record = |sandbox: bool| {
        let mut args: Vec<&std::ffi::OsStr> = vec![
            "app".as_ref(),
            "record".as_ref(),
            root.as_os_str(),
            "--no-traps".as_ref(),
            "--inputs".as_ref(),
            inputs.as_os_str(),
        ];
        if sandbox {
            args.push("--sandbox".as_ref());
        }
        args.extend(["--", "python3", "probe.py"].map(std::ffi::OsStr::new));
        ok(cli(&args))
    };
    let stdout = |name: &str| {
        std::fs::read_to_string(root.join(format!("rust/app-cases/{name}.stdout"))).unwrap()
    };

    record(false);
    assert_eq!(stdout("ping"), "done\n");
    assert_eq!(stdout("touch"), "done\n");
    assert!(root.join("marker.txt").is_file());
    std::fs::remove_file(root.join("marker.txt")).unwrap();

    record(true);
    let ping = stdout("ping");
    let touch = stdout("touch");
    let wrote = root.join("marker.txt").exists();
    drop(listener);
    std::fs::remove_dir_all(&root).unwrap();
    assert_eq!(ping, "blocked\n");
    assert_eq!(touch, "blocked\n");
    assert!(!wrote, "the sandboxed original wrote into the source tree");
}

#[test]
fn a_sandboxed_server_still_answers_on_loopback_or_says_why_not() {
    if !common::have("python3") || !sandbox_runs() {
        return;
    }
    let server = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/http/python/server.py"),
    )
    .unwrap();
    let readme = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/http/python/README.md"),
    )
    .unwrap();
    let root = project("http", &[("server.py", &server), ("README.md", &readme)]);
    let out = cli(&[
        "app".as_ref(),
        "record".as_ref(),
        root.as_os_str(),
        "--http".as_ref(),
        "--no-traps".as_ref(),
        "--sandbox".as_ref(),
        "--".as_ref(),
        "python3".as_ref(),
        "server.py".as_ref(),
    ]);
    let hello =
        std::fs::read_to_string(root.join("rust/app-cases/http/example-001-get-hello.body"));
    std::fs::remove_dir_all(&root).unwrap();
    if cfg!(target_os = "linux") {
        let text = String::from_utf8_lossy(&out.stderr);
        assert!(!out.status.success());
        assert!(
            text.contains("--sandbox") && text.contains("loopback"),
            "{text}"
        );
    } else {
        ok(out);
        assert_eq!(hello.unwrap(), r#"{"message":"hello"}"#);
    }
}
