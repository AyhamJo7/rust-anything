<p align="center">
  <a href="https://rustanything.ai/">
    <img src="docs/assets/logo.png" width="140" height="140" alt="The Rust Anything crab">
  </a>
</p>

<h1 align="center">Rust Anything</h1>

<p align="center">
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io" alt="crates.io version"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm" alt="npm version"></a>
<a href="https://github.com/erkamyaman/rust-anything/stargazers"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=stars" alt="GitHub stars"></a>
<a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/discord-join-111111?style=flat&colorA=111111&colorB=b34a1a" alt="Discord"></a>
<a href="#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-111111?style=flat&colorA=111111&colorB=b34a1a" alt="MIT or Apache-2.0 license"></a><br>
<a href="https://rustanything.ai/"><img src="https://img.shields.io/endpoint?url=https%3A%2F%2Frustanything.ai%2Fbadge-downloads.json&style=flat" alt="Total downloads, crates.io and npm"></a>
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/d/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io%20downloads" alt="crates.io downloads"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/d18m/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm%20downloads" alt="npm downloads"></a>
</p>

<p align="center">
Migrate your app to Rust, and prove it still behaves the same.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center">
<b>Documentation:</b> <a href="https://rustanything.ai/">https://rustanything.ai/</a>
</p>

<p align="center"><a href="docs/translations.md">🌐 Other languages</a></p>

<p align="center">Moving a codebase to Rust? I offer consulting for Rust migrations: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Sponsors

<p align="center">
  Made in Türkiye 🇹🇷 by <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
  Rust Anything is free and open source. If it helps your work, sponsoring keeps it going.
</p>

<p align="center"><sub>Your safety comes first: sponsors get a logo and a link here, nothing more. No paid recommendations and no setup guides for third-party AI API resellers, so nothing here steers your code, keys or data somewhere you didn't choose.</sub></p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/assets/sponsors/company-dark.svg">
      <img src="docs/assets/sponsors/company-light.svg" width="800" alt="Company sponsor: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://www.atakdomain.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/assets/sponsors/domain-dark.svg">
      <img src="docs/assets/sponsors/domain-light.svg" width="800" alt="Domain sponsor: Atak Domain">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/assets/sponsors/sponsors-dark.svg">
      <img src="docs/assets/sponsors/sponsors-light.svg" width="800" alt="Sponsors: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Become a sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Sponsoring as a company, or anything else? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Moving a codebase to Rust? I offer consulting for Rust migrations: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Quick start

Paste this into your coding agent:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Or install it yourself:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
rust-anything doctor          # checks the runtimes your original needs, with the fix for each
```

> [!WARNING]
> Recording runs the original's real code. Work on a copy, and point a server at a test database.

> [!TIP]
> **Agents:** read [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) for the install steps, every command and the porting workflow.

## How it works

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/how-it-works-dark.svg">
    <img src="docs/assets/how-it-works-light.svg" width="880" alt="analyze plans what to port; rust-anything go bridges, records, lets your agent port and gates it, looping until every recorded case matches; enable then switches callers to Rust">
  </picture>
</p>

**What counts as done.** Behavior, not your tests. `record` runs the original on generated and given inputs and saves what each call returns, throws and prints. A port is done when `gate` passes: every recorded case matches, and fmt, clippy, `cargo test`, the `unsafe` check, the stub check and the check for recorded answers pasted into the port all pass. Your own tests run too.

**Your tests become cases.** `record --from-tests -- npx jest` (or `pytest`) runs the original's own suite and records every call it makes to the unit. `--pin-clock` fixes the clock and random numbers on both sides, so date-dependent code can match.

**Whole apps: CLIs and servers.** `app record` records a CLI (exit code, stdout, stderr, files written) or, with `--http`, a server's answers to every request. `app front-door` scaffolds `rust/app`, a Rust binary that owns the entry point and hands everything not ported yet to the original. `app wire` moves units over one at a time, `app replay` must stay at 100% while the fallback rate drops to zero, and `app lift` leaves a pure Rust app.

**Side effects count.** A port that forgets to send the webhook or the email must fail. `app record --effects http,smtp` (or `--effect-env WEBHOOK_URL=http-url`) points the app at local stand-ins that record every outgoing call and mail, and `--db-dump "sqlite3 app.db .dump"` records the database after each request; replay checks the port does the same.

**The original keeps changing?** `app drift` replays the original against its recording and lists what now answers differently, and the [GitHub Action](docs/install.md#in-ci) runs `gate` on every push.

**You make the calls.** When you correct the port's behavior, `rules --add "<rule>"` keeps it, and every later agent prompt includes it. Everything rust-anything writes lives in `<repo>/rust/`.

**Report or migrate, you pick.** `analyze` and `profile` only report: a ranked plan of what to port, and what is hot under a real workload. They are plain commands, so the report costs no tokens. `go` migrates one unit: it bridges the hottest function, records it, hands the port to your agent, then builds, diffs and gates it, and switches it on only when everything matches (TS/JS, Python and PowerShell; `disable` switches it back off).

**How long a run takes.** One `go` run ports one unit, with at most `--rounds` agent turns (3 by default): one for the port, then one per failing diff or gate. A whole app is one run per unit, so you can stop after any of them.

**Cheap model for the plan, strong model for the port.** The plan needs no model. For the port, pass any agent command: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agents.** `rust-anything setup` adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode. Any other MCP client can run `rust-anything mcp`, and the skill also installs on its own with `npx skills add erkamyaman/rust-anything` (see [install](docs/install.md)).

## Commands

| Command | What it does |
| --- | --- |
| `rust-anything go -- <workload>` | Profile, bridge the hottest function, record it, hand the port to your agent, gate it, switch it on |
| `rust-anything analyze <repo>` | Ranked migration plan: units, readiness, risks, dependencies |
| `rust-anything profile <repo> -- <workload>` | What is hot under a real workload, and what to port or batch |
| `rust-anything bridge <repo> <unit>` | Rust crate with bindings for one unit, stubs in `src/port.rs` |
| `rust-anything record <crate>` | Record what the original returns, throws and prints (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Replay every recorded case against the Rust module |
| `rust-anything bench <crate>` | Time the original and Rust on the recorded cases, bridge included |
| `rust-anything fuzz <crate>` | Fresh inputs through both sides: what the recording did not cover |
| `rust-anything gate <crate>` | The definition of done |
| `rust-anything enable` / `disable <crate>` | Switch callers to Rust, or back |
| `rust-anything report <crate>` | `REPORT.md` and a card for a pull request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Port a whole CLI or server |
| `rust-anything rules <repo> --add "<rule>"` | Behavior you corrected, kept by every later port |
| `rust-anything map <repo>` | Every unit, its crate, cases and last diff |
| `rust-anything setup` / `doctor` / `mcp` | Install for agents, check runtimes, MCP server |

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE), at your option. © [Erkam Yaman](https://github.com/erkamyaman). Ports and generated code in your repository are yours.

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in rust-anything, as defined in the Apache-2.0 license, is dual licensed as above, without any additional terms or conditions.
