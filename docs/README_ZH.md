<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="rust-anything 螃蟹">
  </a>
</p>

<h1 align="center">Rust Anything</h1>

<p align="center">
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io" alt="crates.io version"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm" alt="npm version"></a>
<a href="https://github.com/erkamyaman/rust-anything/stargazers"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=stars" alt="GitHub stars"></a>
<a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/discord-join-111111?style=flat&colorA=111111&colorB=b34a1a" alt="Discord"></a>
<a href="https://github.com/erkamyaman/rust-anything#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-111111?style=flat&colorA=111111&colorB=b34a1a" alt="MIT or Apache-2.0 license"></a><br>
<a href="https://rustanything.ai/"><img src="https://img.shields.io/endpoint?url=https%3A%2F%2Frustanything.ai%2Fbadge-downloads.json&style=flat" alt="Total downloads, crates.io and npm"></a>
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/d/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io%20downloads" alt="crates.io downloads"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/d18m/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm%20downloads" alt="npm downloads"></a>
</p>

<p align="center">
用 Rust 重写你的应用，并证明它的行为依然完全一致。
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>文档:</b> <a href="https://rustanything.ai/zh">https://rustanything.ai/zh</a></p>

<p align="center"><a href="translations.md">🌐 其他语言</a></p>

<p align="center">要把代码库迁移到 Rust？我提供 Rust 迁移咨询： <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>本译文由社区维护，可能落后于<a href="../README.md">英文 README</a>。</sub></p>

## 赞助商

<p align="center">由 <a href="https://github.com/erkamyaman">erKam Yaman</a> 在 Türkiye 🇹🇷 制作<br>
Rust Anything 免费且开源。如果它对你的工作有帮助，你的赞助能让它持续发展。</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企业赞助商：CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://www.atakdomain.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/domain-dark.svg">
      <img src="assets/sponsors/domain-light.svg" width="800" alt="Domain sponsor: Atak Domain">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="赞助商：Santosh Yadav、Hakan Soytürk、Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş、bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E6%88%90%E4%B8%BA%E8%B5%9E%E5%8A%A9%E5%95%86-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="成为赞助商"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">以公司身份赞助，或有其他事宜？ <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">要把代码库迁移到 Rust？我提供 Rust 迁移咨询： <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## 快速开始

把这段粘贴到你的编程代理中：

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

或者自己安装：

```sh
cargo install rust-anything   # 或： npm install -g rust-anything
rust-anything setup           # 把它添加到 Claude Code、Codex、Cursor、Gemini CLI 和 OpenCode
rust-anything doctor          # 检查原始代码需要的运行时，并给出每一项的修复方法
```

> [!WARNING]
> 录制会真正运行原始代码。请在副本上操作，并让服务器连接测试数据库。
> `record --sandbox` 和 `app record --sandbox` 会在无网络、仓库只读、限制 CPU 和内存的环境中运行它（Linux 上用 bubblewrap，macOS 上用 sandbox-exec；`doctor` 会检查它是否可用）。

> [!TIP]
> **代理：**阅读 [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt)，了解安装步骤、全部命令和移植流程。

## 工作原理

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/zh-dark.svg">
    <img src="assets/how-it-works/zh-light.svg" width="880" alt="analyze 规划要移植什么；rust-anything go 建立桥接、录制、让你的代理移植并用 gate 检查，循环直到每个录制的用例都匹配；然后 enable 把调用切换到 Rust">
  </picture>
</p>

**怎样才算完成。** 看行为，而不是你的测试。`record` 用生成的和给定的输入运行原始代码，保存每次调用的返回值、抛出的异常和打印的输出。`gate` 通过时移植才算完成：每个录制的用例都匹配，并且 fmt、clippy、`cargo test`、`unsafe` 检查、桩代码检查，以及检查录制的答案是否被直接粘贴进移植代码，全部通过。你自己的测试也会运行。

**你的测试变成用例。** `record --from-tests -- npx jest`（或 `pytest`）运行原始代码自带的测试套件，并录制它对该单元的每一次调用。`--pin-clock` 在两边固定时钟和随机数，让依赖日期的代码也能匹配。

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="移植整个应用：analyze 做计划，由你做决定，app record 录制它的真实行为，前门在原始代码前面运行 Rust，单元逐个迁移，直到不再回退到原始代码，最后 lift 留下一个纯 Rust 应用">
  </picture>
</p>

**整个应用：CLI 和服务器。** `app record` 录制一个 CLI（退出码、stdout、stderr、写入的文件），或者加上 `--http` 录制服务器对每个请求的响应。`app front-door` 生成 `rust/app`，这是一个接管入口点的 Rust 二进制程序，会把所有尚未移植的部分交给原始代码。`app wire` 一次迁移一个单元，在回退率降到零的过程中 `app replay` 必须保持 100%，最后 `app lift` 留下一个纯 Rust 应用。

**副作用也算数。** 忘了发送 webhook 或邮件的移植必须失败。`app record --effects http,smtp`（或 `--effect-env WEBHOOK_URL=http-url`）让应用连接本地替身，录制每一个外发调用和邮件，`--db-dump "sqlite3 app.db .dump"` 则在每个请求之后录制数据库；replay 会检查移植是否做了同样的事。

**原始代码还在变？** `app drift` 用录制结果重放原始代码，列出现在响应不同的地方，[GitHub Action](install.md#in-ci) 会在每次 push 时运行 `gate`。

**由你做决定。** 当你纠正移植的行为时，`rules --add "<rule>"` 会保存它，之后每个代理提示都会包含它。rust-anything 写入的所有内容都在 `<repo>/rust/` 里。

**只看报告还是直接迁移，由你选择。** `analyze` 和 `profile` 只出报告：一份按优先级排序的移植计划，以及真实负载下哪些代码最热。它们是普通命令，所以报告不消耗 token。`go` 迁移一个单元：为最热的函数建立桥接，录制它，把移植交给你的代理，然后构建、比对并运行 `gate`，只有全部匹配时才启用（TS/JS、Python 和 PowerShell；`disable` 可以再关掉）。

**一次运行要多久。** 一次 `go` 运行移植一个单元，最多 `--rounds` 轮代理交互（默认 3）：一轮用于移植，之后每次 diff 或 gate 失败各一轮。整个应用就是每个单元运行一次，所以你可以在任何一次之后停下。

**计划用便宜的模型，移植用强的模型。** 计划不需要模型。移植时，传入任意代理命令：`rust-anything go --agent "claude --model <model>" -- <workload>`。

**代理。** `rust-anything setup` 会把它加到 Claude Code、Codex、Cursor、Gemini CLI 和 OpenCode 中。其他 MCP 客户端可以运行 `rust-anything mcp`，这个 skill 也可以用 `npx skills add erkamyaman/rust-anything` 单独安装（见[安装](install.md)）。

## 语言

| 语言 | 状态 | 检查对象 |
| --- | --- | --- |
| TypeScript/JavaScript | 已验证 | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | 已验证 | CPython 的 `colorsys`（2,400/2,400 个用例） |
| Go | 已验证 | Go 的 `path`（2,642/2,642） |
| Ruby | 已验证 | Ruby 3.3 的 `shellwords`（1,617/1,617） |
| Java | 已验证 | 门面之后的 JDK 21（4,492/4,492） |
| C | 已验证 | ffmpeg 的 libavutil、glibc |
| C# | 已验证 | Google.Protobuf (2,941/2,941) |
| PHP | 已验证 | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | 已测试 | 本仓库的测试夹具，包括 Android |
| PowerShell | 已测试 | 本仓库的测试夹具 |
| Swift | 已测试 | 本仓库的测试夹具 |

**已验证** 指在别人写的真实代码上完成了端到端移植；**已测试** 指由本仓库针对测试夹具的测试覆盖。每次运行的日期和检查方式：[STATUS.md](STATUS.md)。

## 命令

| 命令 | 作用 |
| --- | --- |
| `rust-anything go -- <workload>` | 分析性能，为最热的函数建立桥接，录制它，把移植交给你的代理，运行 gate，然后启用 |
| `rust-anything analyze <repo>` | 按优先级排序的迁移计划：单元、就绪程度、风险、依赖 |
| `rust-anything profile <repo> -- <workload>` | 真实负载下哪些代码最热，以及哪些该移植或批量调用 |
| `rust-anything bridge <repo> <unit>` | 带有单个单元绑定的 Rust crate，桩代码在 `src/port.rs` |
| `rust-anything record <crate>` | 录制原始代码的返回值、抛出的异常和打印的输出（`--inputs`、`--from-tests`、`--pin-clock`） |
| `rust-anything diff <crate>` | 用 Rust 模块重放每个录制的用例 |
| `rust-anything bench <crate>` | 在录制的用例上为原始代码和 Rust 计时，包括桥接开销 |
| `rust-anything fuzz <crate>` | 给两边喂新的输入：找出录制没有覆盖的部分 |
| `rust-anything gate <crate>` | 完成的定义 |
| `rust-anything enable` / `disable <crate>` | 把调用方切换到 Rust，或切回去 |
| `rust-anything report <crate>` | `REPORT.md` 和一张用于 pull request 的卡片 |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | 移植整个 CLI 或服务器 |
| `rust-anything rules <repo> --add "<rule>"` | 你纠正过的行为，之后每次移植都会保留 |
| `rust-anything map <repo>` | 每个单元、它的 crate、用例和最近一次 diff |
| `rust-anything log <repo>` | 每一次启用或关闭，以及每个 wire、lift 和 gate 的结果，来自 `rust/audit.jsonl` |
| `rust-anything setup` / `doctor` / `mcp` | 为代理安装、检查运行时、MCP 服务器 |

## 许可证

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman)。你在自己仓库中生成的移植代码与产物均归你所有。
