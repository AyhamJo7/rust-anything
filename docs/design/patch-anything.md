# Design: proving a security fix (draft)

Status: proposal, nothing built. Scope: fixes to the user's **own code**; dependency CVEs come later.

## Idea

rust-anything proves a rewrite: record what the original does, let an agent change it, accept the
change only when every recorded case still matches. A security fix has the same shape with one
extra requirement:

> The patched code must behave exactly as before on every legitimate input, and must now handle
> the known-bad input safely.

The recording supplies the first half. A pinned exploit case supplies the second. The agent writes
the patch; the tool decides whether it is accepted. This is defensive: it takes a vulnerability you
already know about (an advisory, a bug report, a failing test) and checks the fix, it does not
search for or build attacks.

## Loop

```
record ──> pin ──> patch ──> diff ──> gate ──> report
legit      the bad   agent     legit      both      patch + proof
behavior   input     writes    cases      checks    for the PR
                     it        unchanged  pass
```

| Step | What it does |
| --- | --- |
| record | Reuse `record`: run the original on generated and real inputs (including `--from-tests`), save outputs, errors and effects (files, stdout, exit code). |
| pin | A *finding case*: an input the maintainer supplies from the advisory or bug report, plus the **safe outcome** it must have after the fix (rejected with an error, output escaped, no file written outside a root, no child process started). Stored beside `cases.jsonl` as `findings.jsonl`. The case must reproduce the problem on the unpatched code, or the tool refuses to continue (a fix for something that does not fail proves nothing). |
| patch | The agent edits the original source, with the smallest change that makes the finding case safe. Same prompt-on-stdin contract as `go --agent`; without `--agent` it writes `TASK.md` and stops. |
| diff | Replay every recorded case against the patched code. Any difference on a legitimate case is a regression, listed with input, before and after. Intended differences must be named explicitly in `allowed-changes.toml` with a reason. |
| gate | Passes only when: all legitimate cases match (or are allowed), every finding case now has its safe outcome, the patch touches only the files the unit owns, and no new dependency or `unsafe` appeared. |
| report | `PATCH_REPORT.md`: diff, cases matched, finding cases flipped, allowed changes with reasons. Written for a reviewer. |

## Safe-outcome checks (first version)

Declarative, checked from recorded effects, so no new sandbox is needed:

- `error`: the call rejects with an error (optionally of a given kind).
- `output_not_contains`: the output no longer holds a given string (escaping).
- `no_write_outside: <root>`: filesystem effects stay under a root (path traversal).
- `no_spawn`: no child process started (command injection).
- `bounded: {time, memory}`: the call finishes within a limit (ReDoS, unbounded allocation).

## Why a recording beats "tests pass"

Existing test suites rarely cover the behaviour a patch accidentally changes (encodings, edge
formats, error text). `record` already generates those cases for the port loop; the same cases
catch an over-eager fix that "solves" an injection by rejecting valid input.

## Limits

- It proves the fix against the cases it has. A bad input that is not pinned is not covered;
  `fuzz` with a pinned safe-outcome predicate narrows that, it does not close it.
- Only effects the harness observes can be checked.
- It does not find vulnerabilities and will not write exploits. Finding cases come from the
  maintainer's own report.

## Open questions

1. Name and home: new repo, or a `patch` subcommand next to `go`.
2. Does `allowed-changes.toml` need per-field matching, or is whole-case allow enough for v1?
3. Which language plugin first: TS/JS and Python already have effect recording.
4. Dependency-CVE mode: bump the dependency, replay the recording, report the delta.
