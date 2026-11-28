<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="Die rust-anything-Krabbe">
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
Schreibe deine App in Rust neu und beweise, dass sie sich weiterhin genauso verhält.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Dokumentation:</b> <a href="https://rustanything.ai/de">https://rustanything.ai/de</a></p>

<p align="center"><a href="translations.md">🌐 Andere Sprachen</a></p>

<p align="center">Du überträgst eine Codebasis nach Rust? Ich biete Beratung für Rust-Migrationen an: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Diese Übersetzung wird von der Community gepflegt und kann hinter dem <a href="../README.md">englischen README</a> zurückliegen.</sub></p>

## Sponsoren

<p align="center">Made in Türkiye 🇹🇷 von <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything ist kostenlos und Open Source. Wenn es dir bei der Arbeit hilft, hält dein Sponsoring es am Laufen.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Unternehmenssponsor: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsoren: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Sponsor_werden-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Sponsor werden"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Sponsoring als Unternehmen oder etwas anderes? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Du überträgst eine Codebasis nach Rust? Ich biete Beratung für Rust-Migrationen an: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Schnellstart

Füge das in deinen Coding-Agenten ein:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Oder installiere es selbst:

```sh
cargo install rust-anything   # oder: npm install -g rust-anything
rust-anything setup           # fügt es zu Claude Code, Codex, Cursor, Gemini CLI und OpenCode hinzu
rust-anything doctor          # prüft die Laufzeitumgebungen, die dein Original braucht, mit der Lösung für jede
```

> [!WARNING]
> Die Aufzeichnung führt den echten Code des Originals aus. Arbeite an einer Kopie und richte einen Server auf eine Testdatenbank.
> `record --sandbox` und `app record --sandbox` führen ihn ohne Netzwerk, mit schreibgeschütztem Repository und mit CPU- und Speicherlimits aus (bubblewrap unter Linux, sandbox-exec unter macOS; `doctor` prüft, ob es vorhanden ist).

> [!TIP]
> **Agenten:** Lies [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) für die Installationsschritte, alle Befehle und den Portierungsablauf.

## So funktioniert es

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/de-dark.svg">
    <img src="assets/how-it-works/de-light.svg" width="880" alt="analyze plant, was portiert wird; rust-anything go baut die Brücke, zeichnet auf, lässt deinen Agenten portieren und prüft mit gate, in Runden, bis jeder aufgezeichnete Fall übereinstimmt; enable schaltet die Aufrufer dann auf Rust um">
  </picture>
</p>

**Was als fertig gilt.** Das Verhalten, nicht deine Tests. `record` führt das Original mit generierten und vorgegebenen Eingaben aus und speichert, was jeder Aufruf zurückgibt, wirft und ausgibt. Eine Portierung ist fertig, wenn `gate` besteht: Jeder aufgezeichnete Fall stimmt überein, und fmt, clippy, `cargo test`, die `unsafe`-Prüfung, die Stub-Prüfung und die Prüfung auf aufgezeichnete Antworten, die in die Portierung kopiert wurden, bestehen alle. Deine eigenen Tests laufen ebenfalls.

**Deine Tests werden zu Fällen.** `record --from-tests -- npx jest` (oder `pytest`) führt die eigene Testsuite des Originals aus und zeichnet jeden Aufruf auf, den sie an die Einheit macht. `--pin-clock` fixiert Uhr und Zufallszahlen auf beiden Seiten, damit datumsabhängiger Code übereinstimmen kann.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Portierung einer ganzen App: analyze plant sie, du triffst die Entscheidungen, app record erfasst ihr echtes Verhalten, die Front Door führt Rust vor dem Original aus, Einheiten wandern einzeln herüber, bis nichts mehr zurückfällt, dann hinterlässt lift eine reine Rust-App">
  </picture>
</p>

**Ganze Apps: CLIs und Server.** `app record` zeichnet eine CLI auf (Exit-Code, stdout, stderr, geschriebene Dateien) oder, mit `--http`, die Antworten eines Servers auf jede Anfrage. `app front-door` legt `rust/app` an, ein Rust-Binary, das den Einstiegspunkt übernimmt und alles noch nicht Portierte an das Original weiterreicht. `app wire` holt Einheiten einzeln herüber, `app replay` muss bei 100 % bleiben, während die Fallback-Rate auf null sinkt, und `app lift` hinterlässt eine reine Rust-App.

**Nebenwirkungen zählen.** Eine Portierung, die vergisst, den Webhook oder die E-Mail zu senden, muss scheitern. `app record --effects http,smtp` (oder `--effect-env WEBHOOK_URL=http-url`) richtet die App auf lokale Stellvertreter, die jeden ausgehenden Aufruf und jede Mail aufzeichnen, und `--db-dump "sqlite3 app.db .dump"` zeichnet die Datenbank nach jeder Anfrage auf; das Replay prüft, dass die Portierung dasselbe tut.

**Das Original ändert sich weiter?** `app drift` spielt das Original gegen seine Aufzeichnung ab und listet auf, was jetzt anders antwortet, und die [GitHub Action](install.md#in-ci) führt `gate` bei jedem Push aus.

**Du triffst die Entscheidungen.** Wenn du das Verhalten der Portierung korrigierst, behält `rules --add "<rule>"` es, und jeder spätere Agenten-Prompt enthält es. Alles, was rust-anything schreibt, liegt in `<repo>/rust/`.

**Bericht oder Migration, du entscheidest.** `analyze` und `profile` berichten nur: ein priorisierter Plan, was portiert werden sollte, und was unter echter Last heiß ist. Es sind normale Befehle, der Bericht kostet also keine Tokens. `go` migriert eine Einheit: Es baut eine Brücke zur heißesten Funktion, zeichnet sie auf, übergibt die Portierung an deinen Agenten, kompiliert und vergleicht sie dann, lässt `gate` laufen und schaltet sie erst ein, wenn alles übereinstimmt (TS/JS, Python und PowerShell; `disable` schaltet sie wieder ab).

**Wie lange ein Lauf dauert.** Ein `go`-Lauf portiert eine Einheit, mit höchstens `--rounds` Agentenrunden (standardmäßig 3): eine für die Portierung, dann eine pro fehlgeschlagenem Diff oder Gate. Eine ganze App ist ein Lauf pro Einheit, du kannst also nach jedem aufhören.

**Günstiges Modell für den Plan, starkes Modell für die Portierung.** Der Plan braucht kein Modell. Für die Portierung übergib einen beliebigen Agentenbefehl: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agenten.** `rust-anything setup` fügt es zu Claude Code, Codex, Cursor, Gemini CLI und OpenCode hinzu. Jeder andere MCP-Client kann `rust-anything mcp` ausführen, und der Skill lässt sich auch eigenständig mit `npx skills add erkamyaman/rust-anything` installieren (siehe [Installation](install.md)).

## Sprachen

| Sprache | Status | Geprüft an |
| --- | --- | --- |
| TypeScript/JavaScript | Verifiziert | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Verifiziert | `colorsys` aus CPython (2,400/2,400 Fälle) |
| Go | Verifiziert | `path` aus Go (2,642/2,642) |
| Ruby | Verifiziert | `shellwords` aus Ruby 3.3 (1,617/1,617) |
| Java | Verifiziert | JDK 21 hinter einer Fassade (4,492/4,492) |
| C | Verifiziert | libavutil aus ffmpeg, glibc |
| C# | Verifiziert | Google.Protobuf (2,941/2,941) |
| PHP | Verifiziert | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Getestet | Fixtures dieses Repositorys, einschließlich Android |
| PowerShell | Getestet | Fixtures dieses Repositorys |
| Swift | Getestet | Fixtures dieses Repositorys |

**Verifiziert** heißt: durchgehend portiert, an echtem Code, den jemand anderes geschrieben hat; **Getestet** heißt: durch die Tests dieses Repositorys an Fixtures abgedeckt. Jeder Lauf, mit Datum und Prüfweise: [STATUS.md](STATUS.md).

## Befehle

| Befehl | Was er tut |
| --- | --- |
| `rust-anything go -- <workload>` | Profilieren, Brücke zur heißesten Funktion bauen, aufzeichnen, Portierung an deinen Agenten übergeben, mit gate prüfen, einschalten |
| `rust-anything analyze <repo>` | Priorisierter Migrationsplan: Einheiten, Bereitschaft, Risiken, Abhängigkeiten |
| `rust-anything profile <repo> -- <workload>` | Was unter echter Last heiß ist, und was portiert oder gebündelt werden sollte |
| `rust-anything bridge <repo> <unit>` | Rust-Crate mit Bindings für eine Einheit, Stubs in `src/port.rs` |
| `rust-anything record <crate>` | Aufzeichnen, was das Original zurückgibt, wirft und ausgibt (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Jeden aufgezeichneten Fall gegen das Rust-Modul abspielen |
| `rust-anything bench <crate>` | Original und Rust auf den aufgezeichneten Fällen messen, Brücke inklusive |
| `rust-anything fuzz <crate>` | Frische Eingaben durch beide Seiten: was die Aufzeichnung nicht abgedeckt hat |
| `rust-anything gate <crate>` | Die Definition von fertig |
| `rust-anything enable` / `disable <crate>` | Aufrufer auf Rust umschalten, oder zurück |
| `rust-anything report <crate>` | `REPORT.md` und eine Karte für einen Pull Request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Eine ganze CLI oder einen ganzen Server portieren |
| `rust-anything rules <repo> --add "<rule>"` | Verhalten, das du korrigiert hast, von jeder späteren Portierung beibehalten |
| `rust-anything map <repo>` | Jede Einheit, ihr Crate, ihre Fälle und der letzte Diff |
| `rust-anything log <repo>` | Jedes Ein- oder Ausschalten, jedes wire-, lift- und gate-Ergebnis, aus `rust/audit.jsonl` |
| `rust-anything setup` / `doctor` / `mcp` | Installation für Agenten, Laufzeiten prüfen, MCP-Server |

## Lizenz

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Portierungen und generierter Code in
deinem Repository gehören dir.
