<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="Le crabe de rust-anything">
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
Réécrivez votre application en Rust, et prouvez qu'elle se comporte toujours de la même façon.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Documentation:</b> <a href="https://rustanything.ai/fr">https://rustanything.ai/fr</a></p>

<p align="center"><a href="translations.md">🌐 Autres langues</a></p>

<p align="center">Vous migrez une base de code vers Rust ? Je propose du conseil pour les migrations vers Rust : <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Cette traduction est maintenue par la communauté et peut être en retard sur le <a href="../README.md">README anglais</a>.</sub></p>

## Sponsors

<p align="center">Fait en Türkiye 🇹🇷 par <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything est gratuit et open source. S'il vous aide dans votre travail, votre parrainage le fait avancer.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Sponsor entreprise : CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsors : Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Devenir_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Devenir sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Parrainage en tant qu'entreprise, ou autre chose ? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Vous migrez une base de code vers Rust ? Je propose du conseil pour les migrations vers Rust : <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Démarrage rapide

Collez ceci dans votre agent de code :

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Ou installez-le vous-même :

```sh
cargo install rust-anything   # ou : npm install -g rust-anything
rust-anything setup           # l'ajoute à Claude Code, Codex, Cursor, Gemini CLI et OpenCode
rust-anything doctor          # vérifie les runtimes dont votre original a besoin, avec le correctif pour chacun
```

> [!WARNING]
> L'enregistrement exécute le vrai code de l'original. Travaillez sur une copie, et pointez un serveur vers une base de données de test.

> [!TIP]
> **Agents :** lisez [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) pour les étapes d'installation, toutes les commandes et le flux de portage.

## Comment ça marche

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/fr-dark.svg">
    <img src="assets/how-it-works/fr-light.svg" width="880" alt="analyze planifie quoi porter ; rust-anything go crée le pont, enregistre, laisse votre agent porter et passe par gate, en boucle jusqu'à ce que chaque cas enregistré corresponde ; enable bascule ensuite les appels vers Rust">
  </picture>
</p>

**Ce qui compte comme terminé.** Le comportement, pas vos tests. `record` exécute l'original sur des entrées générées et fournies, et enregistre ce que chaque appel renvoie, lève et affiche. Un portage est terminé quand `gate` passe : chaque cas enregistré correspond, et fmt, clippy, `cargo test`, la vérification `unsafe`, la vérification des stubs et celle des réponses enregistrées collées dans le portage passent toutes. Vos propres tests tournent aussi.

**Vos tests deviennent des cas.** `record --from-tests -- npx jest` (ou `pytest`) exécute la propre suite de l'original et enregistre chaque appel qu'elle fait à l'unité. `--pin-clock` fige l'horloge et les nombres aléatoires des deux côtés, pour que le code qui dépend de la date puisse correspondre.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Porter une application entière : analyze la planifie, vous tranchez les décisions, app record capture son vrai comportement, la porte d'entrée exécute Rust devant l'original, les unités passent une par une jusqu'à ce que plus rien ne retombe sur l'original, puis lift laisse une application en pur Rust">
  </picture>
</p>

**Applications entières : CLI et serveurs.** `app record` enregistre une CLI (code de sortie, stdout, stderr, fichiers écrits) ou, avec `--http`, les réponses d'un serveur à chaque requête. `app front-door` crée `rust/app`, un binaire Rust qui prend le point d'entrée et confie à l'original tout ce qui n'est pas encore porté. `app wire` fait passer les unités une par une, `app replay` doit rester à 100 % pendant que le taux de repli descend à zéro, et `app lift` laisse une application en pur Rust.

**Les effets de bord comptent.** Un portage qui oublie d'envoyer le webhook ou l'e-mail doit échouer. `app record --effects http,smtp` (ou `--effect-env WEBHOOK_URL=http-url`) dirige l'application vers des doublures locales qui enregistrent chaque appel et chaque mail sortants, et `--db-dump "sqlite3 app.db .dump"` enregistre la base de données après chaque requête ; le rejeu vérifie que le portage fait de même.

**L'original continue de changer ?** `app drift` rejoue l'original face à son enregistrement et liste ce qui répond désormais différemment, et la [GitHub Action](install.md#in-ci) lance `gate` à chaque push.

**C'est vous qui décidez.** Quand vous corrigez le comportement du portage, `rules --add "<rule>"` le conserve, et chaque prompt d'agent suivant l'inclut. Tout ce qu'écrit rust-anything se trouve dans `<repo>/rust/`.

**Rapport ou migration, à vous de choisir.** `analyze` et `profile` ne font que rapporter : un plan classé de ce qu'il faut porter, et de ce qui est chaud sous une vraie charge. Ce sont de simples commandes, donc le rapport ne coûte aucun token. `go` migre une unité : il crée le pont vers la fonction la plus chaude, l'enregistre, confie le portage à votre agent, puis le compile, le compare et le passe à `gate`, et ne l'active que si tout correspond (TS/JS, Python et PowerShell ; `disable` le désactive à nouveau).

**Durée d'une exécution.** Une exécution de `go` porte une unité, avec au plus `--rounds` tours d'agent (3 par défaut) : un pour le portage, puis un par diff ou gate en échec. Une application entière, c'est une exécution par unité, vous pouvez donc vous arrêter après n'importe laquelle.

**Modèle bon marché pour le plan, modèle puissant pour le portage.** Le plan n'a besoin d'aucun modèle. Pour le portage, passez n'importe quelle commande d'agent : `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agents.** `rust-anything setup` l'ajoute à Claude Code, Codex, Cursor, Gemini CLI et OpenCode. Tout autre client MCP peut lancer `rust-anything mcp`, et le skill s'installe aussi seul avec `npx skills add erkamyaman/rust-anything` (voir [installation](install.md)).

## Langages

| Langage | Statut | Contrôlé sur |
| --- | --- | --- |
| TypeScript/JavaScript | Vérifié | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Vérifié | `colorsys` de CPython (2,400/2,400 cas) |
| Go | Vérifié | `path` de Go (2,642/2,642) |
| Ruby | Vérifié | `shellwords` de Ruby 3.3 (1,617/1,617) |
| Java | Vérifié | JDK 21 derrière une façade (4,492/4,492) |
| C | Vérifié | libavutil de ffmpeg, glibc |
| C# | Vérifié | Google.Protobuf (2,941/2,941) |
| PHP | Vérifié | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Testé | les fixtures de ce dépôt, Android compris |
| PowerShell | Testé | les fixtures de ce dépôt |

**Vérifié** signifie porté de bout en bout sur du vrai code écrit par quelqu'un d'autre ; **Testé** signifie couvert par les tests de ce dépôt sur des fixtures. Chaque exécution, avec les dates et la façon dont elle a été vérifiée : [STATUS.md](STATUS.md).

## Commandes

| Commande | Ce qu'elle fait |
| --- | --- |
| `rust-anything go -- <workload>` | Profile, crée le pont vers la fonction la plus chaude, l'enregistre, confie le portage à votre agent, le passe à gate, l'active |
| `rust-anything analyze <repo>` | Plan de migration classé : unités, maturité, risques, dépendances |
| `rust-anything profile <repo> -- <workload>` | Ce qui est chaud sous une vraie charge, et ce qu'il faut porter ou regrouper |
| `rust-anything bridge <repo> <unit>` | Crate Rust avec les bindings d'une unité, stubs dans `src/port.rs` |
| `rust-anything record <crate>` | Enregistre ce que l'original renvoie, lève et affiche (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Rejoue chaque cas enregistré face au module Rust |
| `rust-anything bench <crate>` | Chronomètre l'original et Rust sur les cas enregistrés, pont compris |
| `rust-anything fuzz <crate>` | De nouvelles entrées des deux côtés : ce que l'enregistrement n'a pas couvert |
| `rust-anything gate <crate>` | La définition de terminé |
| `rust-anything enable` / `disable <crate>` | Bascule les appelants vers Rust, ou les ramène |
| `rust-anything report <crate>` | `REPORT.md` et une carte pour une pull request |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Porte une CLI ou un serveur entier |
| `rust-anything rules <repo> --add "<rule>"` | Comportement que vous avez corrigé, conservé par chaque portage suivant |
| `rust-anything map <repo>` | Chaque unité, son crate, ses cas et son dernier diff |
| `rust-anything setup` / `doctor` / `mcp` | Installation pour les agents, vérification des runtimes, serveur MCP |

## Licence

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Les portages et le code généré dans votre dépôt vous appartiennent.
