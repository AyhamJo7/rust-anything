<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="O caranguejo do rust-anything">
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
Reescreva a sua aplicação em Rust e prove que continua a comportar-se da mesma forma.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Documentação:</b> <a href="https://rustanything.ai/pt">https://rustanything.ai/pt</a></p>

<p align="center"><a href="translations.md">🌐 Outros idiomas</a></p>

<p align="center">Vai migrar uma base de código para Rust? Ofereço consultoria para migrações para Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Esta tradução é mantida pela comunidade e pode estar desatualizada em relação ao <a href="../README.md">README em inglês</a>.</sub></p>

## Patrocinadores

<p align="center">Feito na Türkiye 🇹🇷 por <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
O Rust Anything é gratuito e de código aberto. Se ele ajuda no seu trabalho, seu patrocínio o mantém em andamento.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Patrocinador empresarial: CodeRabbit">
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
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Tornar--se_patrocinador-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Tornar-se patrocinador"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Patrocínio como empresa, ou qualquer outra coisa? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Vai migrar uma base de código para Rust? Ofereço consultoria para migrações para Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Início rápido

Cole isto no seu agente de código:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Ou instale você mesmo:

```sh
cargo install rust-anything   # ou: npm install -g rust-anything
rust-anything setup           # adiciona-o ao Claude Code, Codex, Cursor, Gemini CLI e OpenCode
rust-anything doctor          # verifica os runtimes de que o seu original precisa, com a correção para cada um
```

> [!WARNING]
> A gravação executa o código real do original. Trabalhe numa cópia e aponte um servidor para uma base de dados de teste.
> `record --sandbox` e `app record --sandbox` executam-no sem rede, com o repositório só de leitura e com limites de CPU e memória (bubblewrap no Linux, sandbox-exec no macOS; `doctor` verifica se está disponível).

> [!TIP]
> **Agentes:** leiam [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) para os passos de instalação, todos os comandos e o fluxo de portabilidade.

## Como funciona

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/pt-dark.svg">
    <img src="assets/how-it-works/pt-light.svg" width="880" alt="analyze planeja o que migrar; rust-anything go cria a ponte, grava, deixa seu agente migrar e passa pelo gate, repetindo até todos os casos gravados coincidirem; depois enable passa as chamadas para Rust">
  </picture>
</p>

**O que conta como concluído.** O comportamento, não os seus testes. `record` executa o original com entradas geradas e fornecidas e guarda o que cada chamada devolve, lança e imprime. Uma migração está concluída quando `gate` passa: todos os casos gravados coincidem, e fmt, clippy, `cargo test`, a verificação de `unsafe`, a verificação de stubs e a verificação de respostas gravadas coladas na migração passam todas. Os seus próprios testes também são executados.

**Os seus testes tornam-se casos.** `record --from-tests -- npx jest` (ou `pytest`) executa a própria suíte do original e grava cada chamada que ela faz à unidade. `--pin-clock` fixa o relógio e os números aleatórios dos dois lados, para que o código que depende da data possa coincidir.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Migrar uma aplicação inteira: analyze planeia-a, você toma as decisões, app record capta o seu comportamento real, a porta de entrada executa Rust à frente do original, as unidades passam uma a uma até nada recorrer ao original, e lift deixa uma aplicação em Rust puro">
  </picture>
</p>

**Aplicações inteiras: CLIs e servidores.** `app record` grava uma CLI (código de saída, stdout, stderr, ficheiros escritos) ou, com `--http`, as respostas de um servidor a cada pedido. `app front-door` cria `rust/app`, um binário Rust que fica com o ponto de entrada e passa ao original tudo o que ainda não foi migrado. `app wire` passa as unidades uma de cada vez, `app replay` tem de se manter a 100% enquanto a taxa de fallback desce a zero, e `app lift` deixa uma aplicação em Rust puro.

**Os efeitos secundários contam.** Uma migração que se esquece de enviar o webhook ou o email tem de falhar. `app record --effects http,smtp` (ou `--effect-env WEBHOOK_URL=http-url`) aponta a aplicação para substitutos locais que gravam cada chamada e cada email de saída, e `--db-dump "sqlite3 app.db .dump"` grava a base de dados depois de cada pedido; o replay verifica que a migração faz o mesmo.

**O original continua a mudar?** `app drift` reproduz o original contra a sua gravação e lista o que agora responde de forma diferente, e a [GitHub Action](install.md#in-ci) executa `gate` em cada push.

**Quem decide é você.** Quando corrige o comportamento da migração, `rules --add "<rule>"` guarda-o, e cada prompt posterior ao agente inclui-o. Tudo o que o rust-anything escreve fica em `<repo>/rust/`.

**Relatório ou migração, você escolhe.** `analyze` e `profile` apenas relatam: um plano ordenado do que migrar e do que está quente sob uma carga real. São comandos simples, por isso o relatório não gasta tokens. `go` migra uma unidade: cria a ponte para a função mais quente, grava-a, entrega a migração ao seu agente, depois compila, compara e passa-a pelo `gate`, e só a ativa quando tudo coincide (TS/JS, Python e PowerShell; `disable` volta a desativá-la).

**Quanto tempo leva uma execução.** Uma execução de `go` migra uma unidade, com no máximo `--rounds` turnos do agente (3 por padrão): um para a migração e depois um por cada diff ou gate que falhe. Uma aplicação inteira é uma execução por unidade, por isso pode parar depois de qualquer uma.

**Modelo barato para o plano, modelo forte para a migração.** O plano não precisa de modelo. Para a migração, passe qualquer comando de agente: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agentes.** `rust-anything setup` adiciona-o ao Claude Code, Codex, Cursor, Gemini CLI e OpenCode. Qualquer outro cliente MCP pode executar `rust-anything mcp`, e a skill também se instala sozinha com `npx skills add erkamyaman/rust-anything` (veja [instalação](install.md)).

## Linguagens

| Linguagem | Estado | Validado em |
| --- | --- | --- |
| TypeScript/JavaScript | Verificado | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Verificado | `colorsys` do CPython (2,400/2,400 casos) |
| Go | Verificado | `path` do Go (2,642/2,642) |
| Ruby | Verificado | `shellwords` do Ruby 3.3 (1,617/1,617) |
| Java | Verificado | JDK 21 atrás de uma fachada (4,492/4,492) |
| C | Verificado | libavutil do ffmpeg, glibc |
| C# | Verificado | Google.Protobuf (2,941/2,941) |
| PHP | Verificado | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Testado | as fixtures deste repositório, incluindo Android |
| PowerShell | Testado | as fixtures deste repositório |
| Swift | Testado | as fixtures deste repositório |

**Verificado** significa migrado de ponta a ponta em código real escrito por outra pessoa; **Testado** significa coberto pelos testes deste repositório em fixtures. Cada execução, com datas e a forma como foi verificada: [STATUS.md](STATUS.md).

## Comandos

| Comando | O que faz |
| --- | --- |
| `rust-anything go -- <workload>` | Faz o perfil, cria a ponte para a função mais quente, grava-a, entrega a migração ao seu agente, passa-a pelo gate e ativa-a |
| `rust-anything analyze <repo>` | Plano de migração ordenado: unidades, prontidão, riscos, dependências |
| `rust-anything profile <repo> -- <workload>` | O que está quente sob uma carga real, e o que migrar ou agrupar |
| `rust-anything bridge <repo> <unit>` | Crate Rust com bindings para uma unidade, stubs em `src/port.rs` |
| `rust-anything record <crate>` | Grava o que o original devolve, lança e imprime (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Reproduz cada caso gravado contra o módulo Rust |
| `rust-anything bench <crate>` | Mede o original e o Rust nos casos gravados, ponte incluída |
| `rust-anything fuzz <crate>` | Entradas novas pelos dois lados: o que a gravação não cobriu |
| `rust-anything gate <crate>` | A definição de concluído |
| `rust-anything enable` / `disable <crate>` | Muda os chamadores para Rust, ou de volta |
| `rust-anything report <crate>` | `REPORT.md` e um cartão para um pull request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Migra uma CLI ou um servidor inteiro |
| `rust-anything rules <repo> --add "<rule>"` | Comportamento que corrigiu, mantido por cada migração posterior |
| `rust-anything map <repo>` | Cada unidade, o seu crate, casos e último diff |
| `rust-anything log <repo>` | Cada ativação ou desativação, cada resultado de wire, lift e gate, a partir de `rust/audit.jsonl` |
| `rust-anything setup` / `doctor` / `mcp` | Instalação para agentes, verificação de runtimes, servidor MCP |

## Licença

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). As migrações e o código gerado no seu repositório pertencem-lhe.
