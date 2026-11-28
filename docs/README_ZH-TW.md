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
用 Rust 重寫你的應用程式，並證明它的行為依然完全相同。
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>文件:</b> <a href="https://rustanything.ai/zh-tw">https://rustanything.ai/zh-tw</a></p>

<p align="center"><a href="translations.md">🌐 其他語言</a></p>

<p align="center">要把程式碼庫遷移到 Rust？我提供 Rust 遷移諮詢： <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>本譯文由社群維護，可能落後於<a href="../README.md">英文 README</a>。</sub></p>

## 贊助商

<p align="center">由 <a href="https://github.com/erkamyaman">erKam Yaman</a> 在 Türkiye 🇹🇷 製作<br>
Rust Anything 免費且開源。如果它對你的工作有幫助，你的贊助能讓它持續發展。</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企業贊助商：CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="個人贊助商：Santosh Yadav、Hakan Soytürk、Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş、bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E6%88%90%E7%82%BA%E8%B4%8A%E5%8A%A9%E5%95%86-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="成為贊助商"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">以公司身分贊助，或有其他事宜？ <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">要把程式碼庫遷移到 Rust？我提供 Rust 遷移諮詢： <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## 快速開始

把這段貼到你的程式代理中：

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

或者自己安裝：

```sh
cargo install rust-anything   # 或： npm install -g rust-anything
rust-anything setup           # 把它加入 Claude Code、Codex、Cursor、Gemini CLI 和 OpenCode
rust-anything doctor          # 檢查原始程式需要的執行環境，並附上每一項的修正方法
```

> [!WARNING]
> 錄製會真正執行原始程式。請在副本上操作，並讓伺服器連到測試資料庫。

> [!TIP]
> **代理：**閱讀 [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt)，了解安裝步驟、所有指令和移植流程。

## 運作方式

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/zh-tw-dark.svg">
    <img src="assets/how-it-works/zh-tw-light.svg" width="880" alt="analyze 規劃要移植什麼；rust-anything go 建立橋接、錄製、讓你的代理移植並用 gate 檢查，循環直到每個錄製的案例都相符；接著 enable 把呼叫切換到 Rust">
  </picture>
</p>

**怎樣才算完成。** 看行為，而不是你的測試。`record` 用產生的和給定的輸入執行原始程式，儲存每次呼叫的回傳值、拋出的例外和印出的輸出。`gate` 通過時移植才算完成：每個錄製的案例都相符，而且 fmt、clippy、`cargo test`、`unsafe` 檢查、stub 檢查，以及檢查錄製的答案是否被直接貼進移植程式碼，全部通過。你自己的測試也會執行。

**你的測試變成案例。** `record --from-tests -- npx jest`（或 `pytest`）執行原始程式自帶的測試套件，並錄製它對該單元的每一次呼叫。`--pin-clock` 在兩邊固定時鐘和亂數，讓依賴日期的程式碼也能相符。

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="移植整個應用程式：analyze 做計畫，由你做決定，app record 錄製它的真實行為，前門在原始程式前面執行 Rust，單元逐個搬移，直到不再退回原始程式，最後 lift 留下一個純 Rust 應用程式">
  </picture>
</p>

**整個應用程式：CLI 和伺服器。** `app record` 錄製一個 CLI（結束代碼、stdout、stderr、寫入的檔案），或者加上 `--http` 錄製伺服器對每個請求的回應。`app front-door` 產生 `rust/app`，這是一個接管進入點的 Rust 執行檔，會把所有尚未移植的部分交給原始程式。`app wire` 一次搬移一個單元，在退回率降到零的過程中 `app replay` 必須維持 100%，最後 `app lift` 留下一個純 Rust 應用程式。

**副作用也算數。** 忘了送出 webhook 或電子郵件的移植必須失敗。`app record --effects http,smtp`（或 `--effect-env WEBHOOK_URL=http-url`）讓應用程式連到本機替身，錄製每一個對外呼叫和郵件，`--db-dump "sqlite3 app.db .dump"` 則在每個請求之後錄製資料庫；replay 會檢查移植是否做了同樣的事。

**原始程式還在變？** `app drift` 以錄製結果重播原始程式，列出現在回應不同的地方，[GitHub Action](install.md#in-ci) 會在每次 push 時執行 `gate`。

**由你做決定。** 當你修正移植的行為時，`rules --add "<rule>"` 會保存它，之後每個代理提示都會包含它。rust-anything 寫入的所有內容都在 `<repo>/rust/` 裡。

**只看報告還是直接遷移，由你決定。** `analyze` 和 `profile` 只產生報告：一份依優先順序排列的移植計畫，以及真實負載下哪些程式碼最熱。它們是一般指令，所以報告不耗費 token。`go` 遷移一個單元：為最熱的函式建立橋接，錄製它，把移植交給你的代理，接著建置、比對並執行 `gate`，只有全部相符時才啟用（TS/JS、Python 和 PowerShell；`disable` 可以再關閉）。

**一次執行要多久。** 一次 `go` 執行移植一個單元，最多 `--rounds` 輪代理互動（預設 3）：一輪用於移植，之後每次 diff 或 gate 失敗各一輪。整個應用程式就是每個單元執行一次，所以你可以在任何一次之後停下。

**計畫用便宜的模型，移植用強的模型。** 計畫不需要模型。移植時，傳入任何代理指令：`rust-anything go --agent "claude --model <model>" -- <workload>`。

**代理。** `rust-anything setup` 會把它加到 Claude Code、Codex、Cursor、Gemini CLI 和 OpenCode 中。其他 MCP 用戶端可以執行 `rust-anything mcp`，這個 skill 也可以用 `npx skills add erkamyaman/rust-anything` 單獨安裝（見[安裝](install.md)）。

## 語言

| 語言 | 狀態 | 檢查對象 |
| --- | --- | --- |
| TypeScript/JavaScript | 已驗證 | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | 已驗證 | CPython 的 `colorsys`（2,400/2,400 個案例） |
| Go | 已驗證 | Go 的 `path`（2,642/2,642） |
| Ruby | 已驗證 | Ruby 3.3 的 `shellwords`（1,617/1,617） |
| Java | 已驗證 | 置於 facade 之後的 JDK 21（4,492/4,492） |
| C | 已驗證 | ffmpeg 的 libavutil、glibc |
| C# | 已驗證 | Google.Protobuf (2,941/2,941) |
| PHP | 已驗證 | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | 已測試 | 本儲存庫的測試 fixture，包括 Android |
| PowerShell | 已測試 | 本儲存庫的測試 fixture |

**已驗證** 指在別人寫的真實程式碼上完成了端到端移植；**已測試** 指由本儲存庫針對 fixture 的測試涵蓋。每次執行的日期和檢查方式：[STATUS.md](STATUS.md)。

## 指令

| 指令 | 作用 |
| --- | --- |
| `rust-anything go -- <workload>` | 分析效能，為最熱的函式建立橋接，錄製它，把移植交給你的代理，執行 gate，然後啟用 |
| `rust-anything analyze <repo>` | 依優先順序排列的遷移計畫：單元、就緒程度、風險、相依套件 |
| `rust-anything profile <repo> -- <workload>` | 真實負載下哪些程式碼最熱，以及哪些該移植或批次呼叫 |
| `rust-anything bridge <repo> <unit>` | 含單一單元綁定的 Rust crate，stub 在 `src/port.rs` |
| `rust-anything record <crate>` | 錄製原始程式的回傳值、拋出的例外和印出的輸出（`--inputs`、`--from-tests`、`--pin-clock`） |
| `rust-anything diff <crate>` | 用 Rust 模組重播每個錄製的案例 |
| `rust-anything bench <crate>` | 在錄製的案例上為原始程式和 Rust 計時，包含橋接開銷 |
| `rust-anything fuzz <crate>` | 給兩邊餵新的輸入：找出錄製沒有涵蓋的部分 |
| `rust-anything gate <crate>` | 完成的定義 |
| `rust-anything enable` / `disable <crate>` | 把呼叫端切換到 Rust，或切回去 |
| `rust-anything report <crate>` | `REPORT.md` 和一張用於 pull request 的卡片 |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | 移植整個 CLI 或伺服器 |
| `rust-anything rules <repo> --add "<rule>"` | 你修正過的行為，之後每次移植都會保留 |
| `rust-anything map <repo>` | 每個單元、它的 crate、案例和最近一次 diff |
| `rust-anything setup` / `doctor` / `mcp` | 為代理安裝、檢查執行環境、MCP 伺服器 |

## 授權條款

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman)。你在自己儲存庫中產生的移植程式碼與產物均歸你所有。
