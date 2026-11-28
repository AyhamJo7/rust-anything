<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="Il granchio di rust-anything">
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
Riscrivi la tua applicazione in Rust e dimostra che si comporta ancora allo stesso modo.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Documentazione:</b> <a href="https://rustanything.ai/it">https://rustanything.ai/it</a></p>

<p align="center"><a href="translations.md">🌐 Altre lingue</a></p>

<p align="center">Stai portando una codebase in Rust? Offro consulenza per le migrazioni a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Questa traduzione è mantenuta dalla community e può restare indietro rispetto al <a href="../README.md">README in inglese</a>.</sub></p>

## Sponsor

<p align="center">Fatto in Türkiye 🇹🇷 da <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything è gratuito e open source. Se ti aiuta nel lavoro, la tua sponsorizzazione lo fa andare avanti.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Sponsor aziendale: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsor: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Become a sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Sponsorizzazione come azienda, o qualcos'altro? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Stai portando una codebase in Rust? Offro consulenza per le migrazioni a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Avvio rapido

Incolla questo nel tuo agente di programmazione:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Oppure installalo tu:

```sh
cargo install rust-anything   # oppure: npm install -g rust-anything
rust-anything setup           # lo aggiunge a Claude Code, Codex, Cursor, Gemini CLI e OpenCode
rust-anything doctor          # controlla i runtime di cui ha bisogno il tuo originale, con la correzione per ciascuno
```

> [!WARNING]
> La registrazione esegue il codice reale dell'originale. Lavora su una copia e punta un server a un database di test.

> [!TIP]
> **Agenti:** leggete [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) per i passi di installazione, tutti i comandi e il flusso di porting.

## Come funziona

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/it-dark.svg">
    <img src="assets/how-it-works/it-light.svg" width="880" alt="analyze pianifica cosa portare; rust-anything go crea il ponte, registra, fa portare il codice al tuo agente e lo fa passare da gate, ripetendo finché ogni caso registrato corrisponde; poi enable passa le chiamate a Rust">
  </picture>
</p>

**Cosa conta come finito.** Il comportamento, non i tuoi test. `record` esegue l'originale su input generati e forniti e salva ciò che ogni chiamata restituisce, lancia e stampa. Un port è finito quando `gate` passa: ogni caso registrato corrisponde, e fmt, clippy, `cargo test`, il controllo `unsafe`, il controllo degli stub e quello sulle risposte registrate incollate nel port passano tutti. Girano anche i tuoi test.

**I tuoi test diventano casi.** `record --from-tests -- npx jest` (o `pytest`) esegue la suite dell'originale e registra ogni chiamata che fa all'unità. `--pin-clock` fissa l'orologio e i numeri casuali su entrambi i lati, così anche il codice che dipende dalla data può corrispondere.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Portare un'intera applicazione: analyze la pianifica, tu prendi le decisioni, app record cattura il suo comportamento reale, il front door esegue Rust davanti all'originale, le unità passano una alla volta finché nulla ricade sull'originale, poi lift lascia un'applicazione in puro Rust">
  </picture>
</p>

**Applicazioni intere: CLI e server.** `app record` registra una CLI (codice di uscita, stdout, stderr, file scritti) o, con `--http`, le risposte di un server a ogni richiesta. `app front-door` crea `rust/app`, un binario Rust che prende il punto di ingresso e passa all'originale tutto ciò che non è ancora portato. `app wire` sposta le unità una alla volta, `app replay` deve restare al 100% mentre il tasso di fallback scende a zero, e `app lift` lascia un'applicazione in puro Rust.

**Gli effetti collaterali contano.** Un port che dimentica di inviare il webhook o l'email deve fallire. `app record --effects http,smtp` (o `--effect-env WEBHOOK_URL=http-url`) punta l'applicazione verso sostituti locali che registrano ogni chiamata e ogni email in uscita, e `--db-dump "sqlite3 app.db .dump"` registra il database dopo ogni richiesta; il replay controlla che il port faccia lo stesso.

**L'originale continua a cambiare?** `app drift` riesegue l'originale contro la sua registrazione ed elenca ciò che ora risponde diversamente, e la [GitHub Action](install.md#in-ci) esegue `gate` a ogni push.

**Decidi tu.** Quando correggi il comportamento del port, `rules --add "<rule>"` lo conserva, e ogni prompt successivo per l'agente lo include. Tutto ciò che rust-anything scrive sta in `<repo>/rust/`.

**Report o migrazione, scegli tu.** `analyze` e `profile` si limitano a riferire: un piano ordinato di cosa portare e di cosa è caldo sotto un carico reale. Sono comandi normali, quindi il report non costa token. `go` migra un'unità: crea il ponte verso la funzione più calda, la registra, passa il port al tuo agente, poi lo compila, lo confronta e lo fa passare da `gate`, e lo attiva solo quando tutto corrisponde (TS/JS, Python e PowerShell; `disable` lo disattiva di nuovo).

**Quanto dura un'esecuzione.** Un'esecuzione di `go` porta un'unità, con al massimo `--rounds` turni dell'agente (3 di default): uno per il port, poi uno per ogni diff o gate fallito. Un'intera applicazione è un'esecuzione per unità, quindi puoi fermarti dopo una qualsiasi.

**Modello economico per il piano, modello potente per il port.** Il piano non richiede alcun modello. Per il port, passa un qualsiasi comando di agente: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agenti.** `rust-anything setup` lo aggiunge a Claude Code, Codex, Cursor, Gemini CLI e OpenCode. Qualsiasi altro client MCP può eseguire `rust-anything mcp`, e la skill si installa anche da sola con `npx skills add erkamyaman/rust-anything` (vedi [installazione](install.md)).

## Linguaggi

| Linguaggio | Stato | Controllato su |
| --- | --- | --- |
| TypeScript/JavaScript | Verificato | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Verificato | `colorsys` di CPython (2,400/2,400 casi) |
| Go | Verificato | `path` di Go (2,642/2,642) |
| Ruby | Verificato | `shellwords` di Ruby 3.3 (1,617/1,617) |
| Java | Verificato | JDK 21 dietro una facciata (4,492/4,492) |
| C | Verificato | libavutil di ffmpeg, glibc |
| C# | Verificato | Google.Protobuf (2,941/2,941) |
| PHP | Verificato | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Testato | le fixture di questo repository, Android incluso |
| PowerShell | Testato | le fixture di questo repository |

**Verificato** significa portato da cima a fondo su codice reale scritto da altri; **Testato** significa coperto dai test di questo repository su fixture. Ogni esecuzione, con date e modalità di controllo: [STATUS.md](STATUS.md).

## Comandi

| Comando | Cosa fa |
| --- | --- |
| `rust-anything go -- <workload>` | Profila, crea il ponte verso la funzione più calda, la registra, passa il port al tuo agente, lo fa passare da gate, lo attiva |
| `rust-anything analyze <repo>` | Piano di migrazione ordinato: unità, prontezza, rischi, dipendenze |
| `rust-anything profile <repo> -- <workload>` | Cosa è caldo sotto un carico reale, e cosa portare o raggruppare |
| `rust-anything bridge <repo> <unit>` | Crate Rust con i binding per un'unità, stub in `src/port.rs` |
| `rust-anything record <crate>` | Registra ciò che l'originale restituisce, lancia e stampa (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Riesegue ogni caso registrato contro il modulo Rust |
| `rust-anything bench <crate>` | Cronometra l'originale e Rust sui casi registrati, ponte incluso |
| `rust-anything fuzz <crate>` | Input nuovi su entrambi i lati: ciò che la registrazione non ha coperto |
| `rust-anything gate <crate>` | La definizione di finito |
| `rust-anything enable` / `disable <crate>` | Passa i chiamanti a Rust, o li riporta indietro |
| `rust-anything report <crate>` | `REPORT.md` e una scheda per una pull request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Porta un'intera CLI o un intero server |
| `rust-anything rules <repo> --add "<rule>"` | Comportamento che hai corretto, mantenuto da ogni port successivo |
| `rust-anything map <repo>` | Ogni unità, il suo crate, i casi e l'ultimo diff |
| `rust-anything setup` / `doctor` / `mcp` | Installazione per gli agenti, controllo dei runtime, server MCP |

## Licenza

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). I port e il codice generato nel tuo repository sono tuoi.
