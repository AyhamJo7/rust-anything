<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="El cangrejo de rust-anything">
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
Reescribe tu aplicación en Rust y demuestra que sigue comportándose igual.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Documentación:</b> <a href="https://rustanything.ai/es">https://rustanything.ai/es</a></p>

<p align="center"><a href="translations.md">🌐 Otros idiomas</a></p>

<p align="center">¿Migrando una base de código a Rust? Ofrezco consultoría para migraciones a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Esta traducción la mantiene la comunidad y puede quedarse atrás respecto al <a href="../README.md">README en inglés</a>.</sub></p>

## Patrocinadores

<p align="center">Hecho en Türkiye 🇹🇷 por <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything es gratuito y de código abierto. Si te ayuda en tu trabajo, tu patrocinio lo mantiene en marcha.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Patrocinador de la empresa: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Patrocinadores: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Become a sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">¿Patrocinio como empresa, o cualquier otra cosa? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">¿Migrando una base de código a Rust? Ofrezco consultoría para migraciones a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Inicio rápido

Pega esto en tu agente de código:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

O instálalo tú mismo:

```sh
cargo install rust-anything   # o: npm install -g rust-anything
rust-anything setup           # lo añade a Claude Code, Codex, Cursor, Gemini CLI y OpenCode
rust-anything doctor          # comprueba los runtimes que necesita tu original, con la solución para cada uno
```

> [!WARNING]
> Grabar ejecuta el código real del original. Trabaja sobre una copia y apunta un servidor a una base de datos de pruebas.

> [!TIP]
> **Agentes:** lee [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) para los pasos de instalación, todos los comandos y el flujo de migración.

## Cómo funciona

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/es-dark.svg">
    <img src="assets/how-it-works/es-light.svg" width="880" alt="analyze planifica qué portar; rust-anything go crea el puente, graba, deja que tu agente porte y lo pasa por gate, repitiendo hasta que todos los casos grabados coinciden; luego enable cambia las llamadas a Rust">
  </picture>
</p>

**Qué cuenta como terminado.** El comportamiento, no tus tests. `record` ejecuta el original con entradas generadas y dadas, y guarda lo que cada llamada devuelve, lanza e imprime. Un porte está terminado cuando `gate` pasa: todos los casos grabados coinciden, y fmt, clippy, `cargo test`, la comprobación de `unsafe`, la de stubs y la de respuestas grabadas pegadas en el porte pasan todas. Tus propios tests también se ejecutan.

**Tus tests se convierten en casos.** `record --from-tests -- npx jest` (o `pytest`) ejecuta la propia suite del original y graba cada llamada que hace a la unidad. `--pin-clock` fija el reloj y los números aleatorios en ambos lados, para que el código que depende de la fecha pueda coincidir.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Portar una aplicación entera: analyze la planifica, tú tomas las decisiones, app record captura su comportamiento real, la puerta de entrada ejecuta Rust delante del original, las unidades pasan una a una hasta que nada recurre al original, y lift deja una aplicación en Rust puro">
  </picture>
</p>

**Aplicaciones enteras: CLIs y servidores.** `app record` graba una CLI (código de salida, stdout, stderr, archivos escritos) o, con `--http`, las respuestas de un servidor a cada petición. `app front-door` genera `rust/app`, un binario de Rust que se queda con el punto de entrada y pasa al original todo lo que aún no está portado. `app wire` traslada las unidades de una en una, `app replay` debe mantenerse al 100 % mientras la tasa de fallback baja a cero, y `app lift` deja una aplicación en Rust puro.

**Los efectos secundarios cuentan.** Un porte que olvida enviar el webhook o el correo debe fallar. `app record --effects http,smtp` (o `--effect-env WEBHOOK_URL=http-url`) apunta la aplicación a sustitutos locales que graban cada llamada y cada correo salientes, y `--db-dump "sqlite3 app.db .dump"` graba la base de datos después de cada petición; el replay comprueba que el porte hace lo mismo.

**¿El original sigue cambiando?** `app drift` reproduce el original contra su grabación y lista lo que ahora responde distinto, y la [GitHub Action](install.md#in-ci) ejecuta `gate` en cada push.

**Tú tomas las decisiones.** Cuando corriges el comportamiento del porte, `rules --add "<rule>"` lo conserva, y cada prompt posterior al agente lo incluye. Todo lo que escribe rust-anything vive en `<repo>/rust/`.

**Informe o migración, tú eliges.** `analyze` y `profile` solo informan: un plan ordenado de qué portar y de qué está caliente bajo una carga real. Son comandos normales, así que el informe no cuesta tokens. `go` migra una unidad: crea el puente para la función más caliente, la graba, entrega el porte a tu agente, luego lo compila, lo compara y lo pasa por `gate`, y solo lo activa cuando todo coincide (TS/JS, Python y PowerShell; `disable` lo vuelve a desactivar).

**Cuánto dura una ejecución.** Una ejecución de `go` porta una unidad, con como máximo `--rounds` turnos del agente (3 por defecto): uno para el porte y luego uno por cada diff o gate que falle. Una aplicación entera es una ejecución por unidad, así que puedes parar después de cualquiera.

**Modelo barato para el plan, modelo potente para el porte.** El plan no necesita modelo. Para el porte, pasa cualquier comando de agente: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agentes.** `rust-anything setup` lo añade a Claude Code, Codex, Cursor, Gemini CLI y OpenCode. Cualquier otro cliente MCP puede ejecutar `rust-anything mcp`, y la skill también se instala por sí sola con `npx skills add erkamyaman/rust-anything` (ver [instalación](install.md)).

## Lenguajes

| Lenguaje | Estado | Comprobado con |
| --- | --- | --- |
| TypeScript/JavaScript | Verificado | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Verificado | `colorsys` de CPython (2,400/2,400 casos) |
| Go | Verificado | `path` de Go (2,642/2,642) |
| Ruby | Verificado | `shellwords` de Ruby 3.3 (1,617/1,617) |
| Java | Verificado | JDK 21 detrás de una fachada (4,492/4,492) |
| C | Verificado | libavutil de ffmpeg, glibc |
| C# | Verificado | Google.Protobuf (2,941/2,941) |
| PHP | Verificado | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Probado | los fixtures de este repositorio, incluido Android |
| PowerShell | Probado | los fixtures de este repositorio |

**Verificado** significa portado de principio a fin sobre código real escrito por otra persona; **Probado** significa cubierto por los tests de este repositorio sobre fixtures. Cada ejecución, con fechas y cómo se comprobó: [STATUS.md](STATUS.md).

## Comandos

| Comando | Qué hace |
| --- | --- |
| `rust-anything go -- <workload>` | Perfila, crea el puente para la función más caliente, la graba, entrega el porte a tu agente, lo pasa por gate y lo activa |
| `rust-anything analyze <repo>` | Plan de migración ordenado: unidades, preparación, riesgos, dependencias |
| `rust-anything profile <repo> -- <workload>` | Qué está caliente bajo una carga real, y qué portar o agrupar |
| `rust-anything bridge <repo> <unit>` | Crate de Rust con bindings para una unidad, stubs en `src/port.rs` |
| `rust-anything record <crate>` | Graba lo que el original devuelve, lanza e imprime (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Reproduce cada caso grabado contra el módulo de Rust |
| `rust-anything bench <crate>` | Mide el original y Rust en los casos grabados, puente incluido |
| `rust-anything fuzz <crate>` | Entradas nuevas por ambos lados: lo que la grabación no cubrió |
| `rust-anything gate <crate>` | La definición de terminado |
| `rust-anything enable` / `disable <crate>` | Cambia los llamadores a Rust, o de vuelta |
| `rust-anything report <crate>` | `REPORT.md` y una tarjeta para un pull request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Porta una CLI o un servidor entero |
| `rust-anything rules <repo> --add "<rule>"` | Comportamiento que corregiste, conservado en cada porte posterior |
| `rust-anything map <repo>` | Cada unidad, su crate, sus casos y su último diff |
| `rust-anything setup` / `doctor` / `mcp` | Instalación para agentes, comprobación de runtimes, servidor MCP |

## Licencia

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Los portes y el código generado en tu repositorio son tuyos.
