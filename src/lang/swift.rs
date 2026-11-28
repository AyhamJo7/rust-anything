//! Swift: SwiftPM packages, imports, risks, the package-to-crate table, and the top-level
//! functions a Swift file declares.
//!
//! Units are top-level functions (not methods, not functions in an `extension`). Each export
//! keeps its declaration as written in `prototype` (`func add(_ a: Int, to b: Int) throws -> Int`),
//! so the generated wrapper can redeclare it with the same argument labels and defaults;
//! [`signature`] reads it back.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use tree_sitter::Node;

use super::syntax::{self, named_children, text};
use crate::risks::Risk;
use crate::ts::deps::CrateMatch;
use crate::ts::discover::{FileKind, Package, SourceFile, Workspace};
use crate::ts::exports::{ExportedFn, Param};
use crate::ts::scan::{FileFacts, Import};

fn language() -> tree_sitter::Language {
    tree_sitter_swift::LANGUAGE.into()
}

const SKIP_DIRS: &[&str] = &[
    ".build",
    ".swiftpm",
    "DerivedData",
    "Pods",
    "Carthage",
    "node_modules",
    "target",
    "build",
];

/// Modules that come with the toolchain or the OS, not from a package.
const SYSTEM_MODULES: &[&str] = &[
    "Swift",
    "Foundation",
    "FoundationEssentials",
    "FoundationNetworking",
    "Dispatch",
    "Darwin",
    "Glibc",
    "Musl",
    "WinSDK",
    "CoreFoundation",
    "os",
    "Combine",
    "Observation",
    "RegexBuilder",
    "Synchronization",
    "XCTest",
    "Testing",
    "SwiftUI",
    "UIKit",
    "AppKit",
    "CoreGraphics",
    "CoreData",
    "CryptoKit",
    "_Concurrency",
];

/// UI frameworks: a file that imports one is UI, flagged and not ported.
const UI_MODULES: &[&str] = &["SwiftUI", "UIKit", "AppKit", "WatchKit"];

// ---- discovery -------------------------------------------------------------------------------

/// Package names from `.package(url: ...)` and `.package(name: ...)` in a `Package.swift`.
pub fn package_dependencies(manifest: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, _) in manifest.match_indices(".package(") {
        let rest = &manifest[i + ".package(".len()..];
        let call = &rest[..rest.find(')').unwrap_or(rest.len())];
        let quoted = |key: &str| -> Option<String> {
            let at = call.find(key)? + key.len();
            let after = call[at..].trim_start().strip_prefix(':')?.trim_start();
            let after = after.strip_prefix('"')?;
            Some(after[..after.find('"')?].to_string())
        };
        let name = quoted("name").or_else(|| {
            let url = quoted("url")?;
            let last = url.trim_end_matches('/').rsplit('/').next()?.to_string();
            Some(last.trim_end_matches(".git").to_string())
        });
        if let Some(name) = name
            && !name.is_empty()
            && !out.contains(&name)
        {
            out.push(name);
        }
    }
    out
}

fn is_test(rel: &str, name: &str) -> bool {
    rel.split('/').any(|c| c == "Tests")
        || name.ends_with("Tests.swift")
        || name.ends_with("Test.swift")
}

fn imports_ui(path: &Path) -> bool {
    std::fs::read_to_string(path).is_ok_and(|text| {
        text.lines().any(|l| {
            let l = l.trim();
            l.strip_prefix("import ")
                .or_else(|| l.strip_prefix("@testable import "))
                .is_some_and(|m| UI_MODULES.contains(&m.trim()))
        })
    })
}

pub fn discover(root: &Path) -> Result<Workspace> {
    let mut packages: Vec<Package> = Vec::new();
    let mut files = Vec::new();
    let walker = crate::walk::filtered(root, |e| {
        !(e.file_type().is_some_and(|t| t.is_dir())
            && SKIP_DIRS.contains(&e.file_name().to_string_lossy().as_ref()))
    })
    .hidden(true)
    .require_git(false)
    .build();
    for entry in walker.flatten() {
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(root) else {
            continue;
        };
        let rel = rel.to_string_lossy().replace('\\', "/");
        let name = rel.rsplit('/').next().unwrap_or(&rel);
        let dir = rel
            .rsplit_once('/')
            .map_or(String::new(), |(d, _)| d.to_string());
        if name == "Package.swift" {
            let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
            packages.push(Package {
                name: if dir.is_empty() {
                    root.file_name()
                        .map_or_else(|| "root".into(), |n| n.to_string_lossy().into_owned())
                } else {
                    dir.clone()
                },
                src_root: if root.join(&dir).join("Sources").is_dir() {
                    join(&dir, "Sources")
                } else {
                    dir.clone()
                },
                dir,
                deps: package_dependencies(&text),
                dev_deps: Vec::new(),
                namespaces: Vec::new(),
            });
        } else if name.ends_with(".swift") {
            let kind = if is_test(&rel, name) {
                FileKind::Test
            } else if imports_ui(entry.path()) {
                FileKind::Ui
            } else {
                FileKind::Source
            };
            files.push((rel, kind));
        }
    }
    if packages.is_empty() {
        packages.push(Package {
            name: root
                .file_name()
                .map_or_else(|| "root".into(), |n| n.to_string_lossy().into_owned()),
            dir: String::new(),
            src_root: if root.join("Sources").is_dir() {
                "Sources".into()
            } else {
                String::new()
            },
            deps: Vec::new(),
            dev_deps: Vec::new(),
            namespaces: Vec::new(),
        });
    }
    packages.sort_by(|a, b| a.dir.cmp(&b.dir));
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let files = files
        .into_iter()
        .map(|(path, kind)| {
            let package = packages
                .iter()
                .enumerate()
                .filter(|(_, p)| p.dir.is_empty() || path.starts_with(&format!("{}/", p.dir)))
                .max_by_key(|(_, p)| p.dir.len())
                .map_or(0, |(i, _)| i);
            SourceFile {
                path,
                package,
                kind,
            }
        })
        .collect();
    Ok(Workspace { packages, files })
}

fn join(dir: &str, rest: &str) -> String {
    if dir.is_empty() {
        rest.to_string()
    } else {
        format!("{dir}/{rest}")
    }
}

// ---- risks and imports -----------------------------------------------------------------------

/// String members that count, index or slice by grapheme cluster.
const GRAPHEME_MEMBERS: &[&str] = &[
    "count",
    "index",
    "startIndex",
    "endIndex",
    "prefix",
    "suffix",
    "dropFirst",
    "dropLast",
    "reversed",
    "first",
    "last",
    "unicodeScalars",
    "utf16",
];

pub fn scan(source: &str) -> FileFacts {
    let mut facts = FileFacts {
        loc: syntax::loc(source, crate::lang::Lang::Swift),
        ..FileFacts::default()
    };
    let Some(tree) = syntax::parse(&language(), source) else {
        return facts;
    };
    let root = tree.root_node();
    facts.parse_errors = usize::from(root.has_error());
    let uses_floats = source.contains("Double") || source.contains("Float");
    let add = |facts: &mut FileFacts, risk: Risk, node: Node| {
        facts
            .risks
            .entry(risk)
            .or_default()
            .push(syntax::line(node))
    };
    syntax::walk(root, |node| match node.kind() {
        "import_declaration" => {
            let Some(module) = named_children(node)
                .into_iter()
                .find(|n| n.kind() == "identifier")
            else {
                return;
            };
            let spec = text(module, source).to_string();
            facts.imports.push(Import {
                specifier: spec,
                type_only: false,
                dynamic: false,
            });
        }
        "throw_keyword" | "catch_block" | "try_operator" => add(&mut facts, Risk::Exceptions, node),
        "await_expression" => add(&mut facts, Risk::Concurrency, node),
        "lambda_literal" => add(&mut facts, Risk::Callbacks, node),
        "for_statement" | "while_statement" | "repeat_while_statement" => facts.loops += 1,
        "class_declaration" => {
            let kind = node
                .child_by_field_name("declaration_kind")
                .map_or("", |k| text(k, source));
            if kind == "actor" {
                add(&mut facts, Risk::Concurrency, node);
            }
            if kind == "class"
                && named_children(node)
                    .iter()
                    .any(|n| n.kind() == "inheritance_specifier")
            {
                add(&mut facts, Risk::ClassInheritance, node);
            }
        }
        "multiplicative_expression" | "additive_expression" => {
            let op = node
                .child_by_field_name("op")
                .map_or("", |o| text(o, source));
            if matches!(op, "*" | "/" | "%") {
                add(&mut facts, Risk::SwiftTraps, node);
            }
        }
        "bitwise_operation" => {
            let op = node
                .child_by_field_name("op")
                .map_or("", |o| text(o, source));
            if matches!(op, "<<" | ">>") {
                add(&mut facts, Risk::IntegerSemantics, node);
            }
        }
        "custom_operator" => {
            if matches!(text(node, source), "&+" | "&-" | "&*" | "&<<" | "&>>") {
                add(&mut facts, Risk::IntegerSemantics, node);
            }
        }
        "bang"
            if node
                .parent()
                .is_some_and(|p| p.kind() == "postfix_expression") =>
        {
            add(&mut facts, Risk::SwiftTraps, node)
        }
        "navigation_suffix" => {
            let member = node
                .child_by_field_name("suffix")
                .map_or("", |s| text(s, source));
            if GRAPHEME_MEMBERS.contains(&member) {
                add(&mut facts, Risk::GraphemeStrings, node);
            }
            if member == "random" {
                add(&mut facts, Risk::ClockAndRandom, node);
            }
        }
        "user_type" => match text(node, source) {
            "Character" | "String.Index" => add(&mut facts, Risk::GraphemeStrings, node),
            "Set" | "Dictionary" => add(&mut facts, Risk::UnorderedIteration, node),
            "Mirror" => add(&mut facts, Risk::Reflection, node),
            "NSRegularExpression" | "Regex" => add(&mut facts, Risk::RegexSyntax, node),
            _ => {}
        },
        "dictionary_type" => add(&mut facts, Risk::UnorderedIteration, node),
        "interpolated_expression" if uses_floats => add(&mut facts, Risk::DoubleFormatting, node),
        "call_expression" => {
            let callee = named_children(node)
                .into_iter()
                .next()
                .map_or("", |c| text(c, source));
            match callee {
                "Int" | "Int32" | "Int64" | "UInt" | "UInt8" | "fatalError" | "precondition" => {
                    add(&mut facts, Risk::SwiftTraps, node)
                }
                "Date" | "UUID" => add(&mut facts, Risk::ClockAndRandom, node),
                "Mirror" => add(&mut facts, Risk::Reflection, node),
                "NSRegularExpression" | "Regex" => add(&mut facts, Risk::RegexSyntax, node),
                "String" if uses_floats => add(&mut facts, Risk::DoubleFormatting, node),
                "DispatchQueue.main.async" | "DispatchQueue.global" | "Task" => {
                    add(&mut facts, Risk::Concurrency, node)
                }
                _ => {}
            }
        }
        _ => {}
    });
    let async_fn = |n: Node| {
        let mut cursor = n.walk();
        n.children(&mut cursor).any(|c| c.kind() == "async")
    };
    for decl in named_children(root) {
        if decl.kind() == "function_declaration" && async_fn(decl) {
            add(&mut facts, Risk::Async, decl);
        }
    }
    facts
}

pub fn resolve(spec: &str, index: &HashMap<&str, usize>, ws: &Workspace) -> super::Resolution {
    let module = spec.split('.').next().unwrap_or(spec).trim();
    if SYSTEM_MODULES.contains(&module) {
        return super::Resolution::Stdlib;
    }
    for pkg in &ws.packages {
        let dir = join(&pkg.src_root, module);
        if let Some((_, i)) = index
            .iter()
            .filter(|(f, _)| f.starts_with(&format!("{dir}/")))
            .min_by_key(|(f, _)| **f)
        {
            return super::Resolution::File(*i);
        }
    }
    super::Resolution::External(module.to_string())
}

pub fn lookup(name: &str) -> Option<CrateMatch> {
    crate::crates::find("swift", name)
}

// ---- types -----------------------------------------------------------------------------------

/// Split at top-level commas (outside `<>`, `[]` and `()`).
fn split_top(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '<' | '[' | '(' => depth += 1,
            '>' | ']' | ')' => depth -= 1,
            ',' if depth == 0 => {
                out.push(current.trim().to_string());
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

/// The position of the top-level `:` of a dictionary type's inside, `K: V`.
fn top_colon(text: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in text.char_indices() {
        match c {
            '<' | '[' | '(' => depth += 1,
            '>' | ']' | ')' => depth -= 1,
            ':' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn element(inner: Result<String, String>) -> Result<String, String> {
    inner.map(|i| {
        if i.contains(' ') {
            format!("({i})[]")
        } else {
            format!("{i}[]")
        }
    })
}

fn dictionary(key: &str, value: &str, whole: &str) -> Result<String, String> {
    if swift_to_ts(key)? != "string" {
        return Err(format!("`{whole}`: JSON object keys are strings"));
    }
    Ok(format!("Record<string, {}>", swift_to_ts(value)?))
}

/// A Swift type in the TS-like dialect, or why the bridge cannot carry it.
pub fn swift_to_ts(t: &str) -> Result<String, String> {
    let t = t.trim();
    if t.contains("->") {
        return Err("takes or returns a closure".into());
    }
    if t.starts_with("inout ") {
        return Err("takes an `inout` parameter".into());
    }
    if t.starts_with("some ") || t.starts_with("any ") {
        return Err(format!("uses an existential or opaque type `{t}`"));
    }
    if let Some(inner) = t.strip_suffix('?').or_else(|| t.strip_suffix('!')) {
        return swift_to_ts(inner).map(|i| format!("{i} | undefined"));
    }
    if let Some(inside) = t.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        return match top_colon(inside) {
            Some(i) => dictionary(&inside[..i], &inside[i + 1..], t),
            None => element(swift_to_ts(inside)),
        };
    }
    if t.starts_with('(') {
        return if t == "()" {
            Ok("void".into())
        } else {
            Err("uses a tuple".into())
        };
    }
    let (head, args) = match t.find('<') {
        Some(i) if t.ends_with('>') => (&t[..i], split_top(&t[i + 1..t.len() - 1])),
        _ => (t, Vec::new()),
    };
    let head = head.trim().strip_prefix("Swift.").unwrap_or(head.trim());
    let arg = |i: usize| -> Result<String, String> {
        args.get(i)
            .map_or_else(|| Err(format!("raw `{head}`")), |a| swift_to_ts(a))
    };
    Ok(match head {
        "String" => "string".into(),
        "Int" | "Int64" | "Int32" | "Int16" | "Int8" | "UInt32" | "UInt16" | "UInt8" => {
            "int".into()
        }
        "UInt" | "UInt64" => return Err(format!("`{head}`: wider than a JSON-safe integer")),
        "Double" | "Float" | "Float64" | "Float32" => "number".into(),
        "Bool" => "boolean".into(),
        "Void" | "Never" => "void".into(),
        "Optional" => format!("{} | undefined", arg(0)?),
        "Array" | "ContiguousArray" => return element(arg(0)),
        "Dictionary" => {
            return match args.as_slice() {
                [k, v] => dictionary(k, v, t),
                _ => Err(format!("raw `{head}`")),
            };
        }
        "Set" => return Err("uses a `Set`, whose order changes from run to run".into()),
        "Character" => return Err("uses a `Character`: one grapheme cluster".into()),
        "Substring" => return Err("uses a `Substring`, which shares its string's storage".into()),
        "Any" | "AnyObject" => return Err("takes or returns `Any`: values of any type".into()),
        "Decimal" => return Err("uses `Decimal`, a decimal no JSON number keeps exactly".into()),
        "Date" | "URL" | "Data" | "UUID" => {
            return Err(format!("uses the Foundation type `{head}`"));
        }
        other => return Err(super::uses_class(other)),
    })
}

// ---- signatures ------------------------------------------------------------------------------

/// One parameter of a Swift function as declared.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwiftParam {
    /// The argument label callers write; `None` for `_`.
    pub label: Option<String>,
    /// The name inside the function.
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
    pub variadic: bool,
}

/// A Swift function declaration, read from its text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwiftSig {
    pub name: String,
    pub params: Vec<SwiftParam>,
    pub throws: bool,
    pub rethrows: bool,
    pub is_async: bool,
    pub generic: bool,
    /// The result type as written; `Void` when there is none.
    pub ret: String,
}

fn read_signature(function: Node, source: &str) -> Option<SwiftSig> {
    let name = text(function.child_by_field_name("name")?, source).to_string();
    let mut params = Vec::new();
    let mut throws = false;
    let mut rethrows = false;
    let mut is_async = false;
    let mut generic = false;
    let mut ret = None;
    let mut after_arrow = false;
    let mut cursor = function.walk();
    let children: Vec<(Option<&str>, Node)> = function
        .children(&mut cursor)
        .enumerate()
        .map(|(i, c)| (function.field_name_for_child(i as u32), c))
        .collect();
    for (field, child) in children {
        match child.kind() {
            "parameter" => {
                let mut label = None;
                let mut pname = None;
                let mut ty = None;
                let mut variadic = false;
                let mut c2 = child.walk();
                let inner: Vec<(Option<&str>, Node)> = child
                    .children(&mut c2)
                    .enumerate()
                    .map(|(i, c)| (child.field_name_for_child(i as u32), c))
                    .collect();
                let mut modifiers = String::new();
                for (f, n) in inner {
                    match (f, n.kind()) {
                        (Some("external_name"), _) => label = Some(text(n, source).to_string()),
                        (Some("name"), "simple_identifier") if pname.is_none() => {
                            pname = Some(text(n, source).to_string());
                        }
                        (_, "parameter_modifiers") => {
                            modifiers = format!("{} ", text(n, source));
                        }
                        (_, "...") => variadic = true,
                        (Some("name"), _) => ty = Some(format!("{modifiers}{}", text(n, source))),
                        _ => {}
                    }
                }
                let pname = pname?;
                let label = match label.as_deref() {
                    Some("_") => None,
                    Some(l) => Some(l.to_string()),
                    None => Some(pname.clone()),
                };
                params.push(SwiftParam {
                    label,
                    name: pname,
                    ty: ty.unwrap_or_else(|| "Any".into()),
                    default: None,
                    variadic,
                });
            }
            _ if field == Some("default_value") => {
                if let Some(last) = params.last_mut() {
                    last.default = Some(text(child, source).to_string());
                }
            }
            "type_parameters" => generic = true,
            "async" => is_async = true,
            "throws" => {
                if text(child, source).starts_with("rethrows") {
                    rethrows = true;
                } else {
                    throws = true;
                }
            }
            "->" => after_arrow = true,
            "type_constraints" => generic = true,
            _ if after_arrow && ret.is_none() && field == Some("name") => {
                ret = Some(text(child, source).to_string());
            }
            _ => {}
        }
    }
    Some(SwiftSig {
        name,
        params,
        throws,
        rethrows,
        is_async,
        generic,
        ret: ret.unwrap_or_else(|| "Void".into()),
    })
}

/// Reads a declaration back from an export's `prototype` (`func f(_ a: Int) throws -> Int`).
pub fn signature(prototype: &str) -> Option<SwiftSig> {
    let source = format!("{prototype} {{}}\n");
    let tree = syntax::parse(&language(), &source)?;
    let function = named_children(tree.root_node())
        .into_iter()
        .find(|n| n.kind() == "function_declaration")?;
    read_signature(function, &source)
}

// ---- exports ---------------------------------------------------------------------------------

fn visibility(function: Node, source: &str) -> Option<String> {
    let modifiers = named_children(function)
        .into_iter()
        .find(|n| n.kind() == "modifiers")?;
    named_children(modifiers)
        .into_iter()
        .find(|n| n.kind() == "visibility_modifier")
        .map(|n| text(n, source).to_string())
}

fn has_attribute(function: Node, source: &str, name: &str) -> bool {
    named_children(function)
        .into_iter()
        .filter(|n| n.kind() == "modifiers")
        .any(|m| text(m, source).contains(name))
}

/// The functions a declaration's body calls, by the name written at the call.
fn calls(function: Node, source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(body) = function.child_by_field_name("body") else {
        return out;
    };
    syntax::walk(body, |node| {
        if node.kind() != "call_expression" {
            return;
        }
        let Some(callee) = named_children(node).into_iter().next() else {
            return;
        };
        let call: String = text(callee, source).split_whitespace().collect();
        if !call.is_empty()
            && call
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            && !out.contains(&call)
        {
            out.push(call);
        }
    });
    out
}

fn function_export(path: &str, function: Node, source: &str) -> Option<ExportedFn> {
    let vis = visibility(function, source);
    if matches!(vis.as_deref(), Some("private" | "fileprivate")) {
        return None;
    }
    let sig = read_signature(function, source)?;
    let mut blocker: Option<String> = None;
    if sig.generic {
        blocker = Some("generic: the wrapper needs concrete types".into());
    }
    if sig.is_async {
        blocker.get_or_insert_with(|| "is async".into());
    }
    if sig.rethrows {
        blocker.get_or_insert_with(|| "rethrows: takes a throwing closure".into());
    }
    if has_attribute(function, source, "@objc") {
        blocker.get_or_insert_with(|| "is `@objc`: Objective-C callers bind it by selector".into());
    }
    let mut params = Vec::new();
    for p in &sig.params {
        if p.variadic {
            blocker.get_or_insert_with(|| format!("takes a variadic parameter `{}`", p.name));
        }
        let ts_type = swift_to_ts(&p.ty).unwrap_or_else(|reason| {
            blocker.get_or_insert(reason);
            "unknown".into()
        });
        params.push(Param {
            name: p.name.clone(),
            ts_type,
            native_type: Some(p.ty.clone()),
            ..Default::default()
        });
    }
    let returns = swift_to_ts(&sig.ret).unwrap_or_else(|reason| {
        blocker.get_or_insert(reason);
        "unknown".into()
    });
    let mut cursor = function.walk();
    let start = function
        .children(&mut cursor)
        .find(|n| n.kind() == "func")
        .map_or(function.start_byte(), |n| n.start_byte());
    let end = function
        .child_by_field_name("body")
        .map_or(function.end_byte(), |b| b.start_byte());
    let declared: String = source
        .get(start..end)
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let prototype = if vis.as_deref() == Some("public") {
        format!("public {declared}")
    } else {
        declared
    };
    Some(ExportedFn {
        name: sig.name.clone(),
        file: path.to_string(),
        line: syntax::line(function),
        params,
        returns,
        is_async: sig.is_async,
        native_return: Some(sig.ret.clone()),
        prototype: Some(prototype),
        blocker,
        calls: calls(function, source),
        ..Default::default()
    })
}

/// The top-level functions of a Swift file that the rest of its module can call (not `private`
/// or `fileprivate`).
pub fn exports(path: &str, source: &str) -> Vec<ExportedFn> {
    if !path.ends_with(".swift") || path.ends_with("Package.swift") {
        return Vec::new();
    }
    let Some(tree) = syntax::parse(&language(), source) else {
        return Vec::new();
    };
    let mut out: Vec<ExportedFn> = named_children(tree.root_node())
        .into_iter()
        .filter(|n| n.kind() == "function_declaration")
        .filter_map(|n| function_export(path, n, source))
        .collect();
    let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for f in &out {
        *counts.entry(f.name.clone()).or_default() += 1;
    }
    for f in &mut out {
        if counts.get(&f.name).copied().unwrap_or(0) > 1 {
            f.blocker = Some(super::OVERLOADED.into());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_map_to_the_dialect_or_a_reason() {
        for (swift, ts) in [
            ("String", "string"),
            ("Int", "int"),
            ("Swift.Int32", "int"),
            ("Double", "number"),
            ("Bool", "boolean"),
            ("Void", "void"),
            ("()", "void"),
            ("Int?", "int | undefined"),
            ("Optional<String>", "string | undefined"),
            ("[String]", "string[]"),
            ("Array<Int>", "int[]"),
            ("[Int?]", "(int | undefined)[]"),
            ("[String: Int]", "Record<string, int>"),
            ("Dictionary<String, [Double]>", "Record<string, number[]>"),
            (
                "[String: [String: Bool]]",
                "Record<string, Record<string, boolean>>",
            ),
        ] {
            assert_eq!(swift_to_ts(swift), Ok(ts.to_string()), "{swift}");
        }
        for (swift, reason) in [
            ("(Int) -> Int", "closure"),
            ("inout Int", "inout"),
            ("[Int: String]", "keys are strings"),
            ("Set<Int>", "order changes"),
            ("(Int, String)", "tuple"),
            ("UInt64", "JSON-safe"),
            ("Point", "uses the class `Point`"),
            ("Character", "grapheme"),
        ] {
            let got = swift_to_ts(swift).unwrap_err();
            assert!(got.contains(reason), "{swift}: {got}");
        }
    }

    #[test]
    fn a_prototype_reads_back_with_labels_and_defaults() {
        let sig =
            signature("public func add(_ a: Int, to b: Int = 2, c: [String: Int]) throws -> Int")
                .unwrap();
        assert_eq!(sig.name, "add");
        assert!(sig.throws && !sig.rethrows && !sig.is_async && !sig.generic);
        assert_eq!(sig.ret, "Int");
        let read: Vec<(Option<&str>, &str, &str, Option<&str>)> = sig
            .params
            .iter()
            .map(|p| {
                (
                    p.label.as_deref(),
                    p.name.as_str(),
                    p.ty.as_str(),
                    p.default.as_deref(),
                )
            })
            .collect();
        assert_eq!(
            read,
            [
                (None, "a", "Int", None),
                (Some("to"), "b", "Int", Some("2")),
                (Some("c"), "c", "[String: Int]", None),
            ]
        );
        assert_eq!(signature("func f()").unwrap().ret, "Void");
    }

    #[test]
    fn exports_are_top_level_and_visible() {
        let src = r#"import Foundation

enum ParseError: Error { case empty }

/// Doc.
public func parse(_ text: String) throws -> Int { 0 }
func shout(word w: String) -> String { w }
private func hidden() {}
fileprivate func alsoHidden() {}
func generic<T>(_ x: T) -> T { x }
func later() async -> Int { 1 }
func apply(_ f: (Int) -> Int) -> Int { f(1) }
func sum(_ xs: Int...) -> Int { 0 }
func twice(_ x: Int) -> Int { x }
func twice(_ x: Double) -> Double { x }
@objc public func legacy() -> Int { 0 }
struct S { func method() {} }
extension S { func more() {} }
"#;
        let out = exports("Sources/Lib/Text.swift", src);
        let got: Vec<(&str, Option<&str>)> = out
            .iter()
            .map(|f| (f.name.as_str(), f.blocker.as_deref()))
            .collect();
        assert_eq!(
            got,
            [
                ("parse", None),
                ("shout", None),
                ("generic", Some("generic: the wrapper needs concrete types")),
                ("later", Some("is async")),
                ("apply", Some("takes or returns a closure")),
                ("sum", Some("takes a variadic parameter `xs`")),
                ("twice", Some(crate::lang::OVERLOADED)),
                ("twice", Some(crate::lang::OVERLOADED)),
                (
                    "legacy",
                    Some("is `@objc`: Objective-C callers bind it by selector")
                ),
            ]
        );
        assert_eq!(
            out[0].prototype.as_deref(),
            Some("public func parse(_ text: String) throws -> Int")
        );
        assert_eq!(out[0].params[0].ts_type, "string");
        assert_eq!(out[0].returns, "int");
        assert_eq!(
            out[1].prototype.as_deref(),
            Some("func shout(word w: String) -> String")
        );
        assert_eq!(out[1].params[0].name, "w");
    }

    #[test]
    fn scans_imports_and_swift_risks() {
        let src = "import Foundation\nimport Collections\n\nfunc f(s: String, d: [String: Int], r: Double) throws -> Int {\n    let n = s.count * 2\n    let x = d[\"a\"]!\n    if n < 0 { throw E.bad }\n    return \"\\(r)\".count + x\n}\n";
        let facts = scan(src);
        let specs: Vec<&str> = facts.imports.iter().map(|i| i.specifier.as_str()).collect();
        assert_eq!(specs, ["Foundation", "Collections"]);
        for risk in [
            Risk::SwiftTraps,
            Risk::GraphemeStrings,
            Risk::Exceptions,
            Risk::UnorderedIteration,
            Risk::DoubleFormatting,
        ] {
            assert!(
                facts.risks.contains_key(&risk),
                "{risk:?}: {:?}",
                facts.risks
            );
        }
        assert_eq!(facts.parse_errors, 0);
        let not = scan("func g(_ ok: Bool) -> Bool {\n    return !ok\n}\n");
        assert!(
            !not.risks.contains_key(&Risk::SwiftTraps),
            "{:?}",
            not.risks
        );
    }

    #[test]
    fn package_dependencies_are_named_from_the_manifest() {
        let manifest = r#"let package = Package(
    name: "Geo",
    dependencies: [
        .package(url: "https://github.com/apple/swift-argument-parser", from: "1.2.0"),
        .package(url: "https://github.com/apple/swift-collections.git", from: "1.0.0"),
        .package(name: "Local", path: "../Local"),
    ]
)"#;
        assert_eq!(
            package_dependencies(manifest),
            ["swift-argument-parser", "swift-collections", "Local"]
        );
    }
}
