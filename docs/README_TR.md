<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="rust-anything yengeci">
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
Uygulamanızı Rust ile yeniden yazın ve hâlâ aynı davrandığını kanıtlayın.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Dokümantasyon:</b> <a href="https://rustanything.ai/tr">https://rustanything.ai/tr</a></p>

<p align="center"><a href="translations.md">🌐 Diğer diller</a></p>

<p align="center">Bir kod tabanını Rust'a mı taşıyorsunuz? Rust geçişleri için danışmanlık veriyorum: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Bu çeviri topluluk tarafından sürdürülmektedir ve <a href="../README.md">İngilizce README</a>'nin gerisinde kalabilir.</sub></p>

## Sponsorlar

<p align="center">Türkiye'de 🇹🇷 <a href="https://github.com/erkamyaman">erKam Yaman</a> tarafından yapıldı<br>
Rust Anything ücretsiz ve açık kaynaktır. İşinize yarıyorsa, sponsor olmanız geliştirmeyi sürdürür.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Şirket sponsoru: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsorlar: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Sponsor_olun-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Sponsor olun"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Şirket olarak sponsorluk ya da başka bir konu mu? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Bir kod tabanını Rust'a mı taşıyorsunuz? Rust geçişleri için danışmanlık veriyorum: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Hızlı başlangıç

Bunu kodlama ajanınıza yapıştırın:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Ya da kendiniz kurun:

```sh
cargo install rust-anything   # ya da: npm install -g rust-anything
rust-anything setup           # Claude Code, Codex, Cursor, Gemini CLI ve OpenCode'a ekler
rust-anything doctor          # orijinalinizin ihtiyaç duyduğu çalışma ortamlarını denetler, her biri için çözümüyle
```

> [!WARNING]
> Kayıt, orijinalin gerçek kodunu çalıştırır. Bir kopya üzerinde çalışın ve sunucuyu bir test veritabanına yönlendirin.
> `record --sandbox` ve `app record --sandbox` onu ağ erişimi olmadan, repo salt okunur hâlde ve CPU ile bellek sınırlarıyla çalıştırır (Linux'ta bubblewrap, macOS'te sandbox-exec; `doctor` bunun olup olmadığını kontrol eder).

> [!TIP]
> **Ajanlar:** kurulum adımları, tüm komutlar ve port etme akışı için [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) dosyasını okuyun.

## Nasıl çalışır

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/tr-dark.svg">
    <img src="assets/how-it-works/tr-light.svg" width="880" alt="analyze neyin port edileceğini planlar; rust-anything go köprü kurar, kaydeder, portu ajanınıza yazdırır ve gate'ten geçirir, kaydedilen her vaka eşleşene kadar tekrarlar; ardından enable çağıranları Rust'a geçirir">
  </picture>
</p>

**Ne zaman bitmiş sayılır.** Testleriniz değil, davranış. `record` orijinali üretilmiş ve verilmiş girdilerle çalıştırır ve her çağrının döndürdüğünü, fırlattığını ve yazdırdığını kaydeder. Bir port, `gate` geçtiğinde biter: kaydedilen her vaka eşleşir; fmt, clippy, `cargo test`, `unsafe` kontrolü, stub kontrolü ve porta yapıştırılmış kayıtlı cevap kontrolü de geçer. Kendi testleriniz de çalışır.

**Testleriniz vakaya dönüşür.** `record --from-tests -- npx jest` (ya da `pytest`) orijinalin kendi test takımını çalıştırır ve birime yaptığı her çağrıyı kaydeder. `--pin-clock` iki tarafta da saati ve rastgele sayıları sabitler, böylece tarihe bağlı kod da eşleşebilir.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works-app-dark.svg">
    <img src="assets/how-it-works-app-light.svg" width="880" alt="Bütün bir uygulamayı port etmek: analyze planlar, kararları siz verirsiniz, app record gerçek davranışını kaydeder, front door Rust'ı orijinalin önünde çalıştırır, birimler hiçbir şey orijinale düşmeyene kadar birer birer taşınır, sonra lift geriye saf bir Rust uygulaması bırakır">
  </picture>
</p>

**Bütün uygulamalar: CLI'lar ve sunucular.** `app record` bir CLI'ı (çıkış kodu, stdout, stderr, yazılan dosyalar) ya da `--http` ile bir sunucunun her isteğe verdiği cevapları kaydeder. `app front-door`, giriş noktasını üstlenen ve henüz port edilmemiş her şeyi orijinale devreden bir Rust ikilisi olan `rust/app` iskeletini kurar. `app wire` birimleri birer birer taşır, fallback oranı sıfıra inerken `app replay` %100'de kalmalıdır ve `app lift` geriye saf bir Rust uygulaması bırakır.

**Yan etkiler de sayılır.** Webhook'u ya da e-postayı göndermeyi unutan bir port başarısız olmalıdır. `app record --effects http,smtp` (ya da `--effect-env WEBHOOK_URL=http-url`) uygulamayı giden her çağrıyı ve e-postayı kaydeden yerel yedeklere yönlendirir, `--db-dump "sqlite3 app.db .dump"` ise her istekten sonra veritabanını kaydeder; replay portun da aynısını yaptığını kontrol eder.

**Orijinal değişmeye devam mı ediyor?** `app drift` orijinali kendi kaydına karşı yeniden oynatır ve artık farklı cevap verenleri listeler; [GitHub Action](install.md#in-ci) da her push'ta `gate` çalıştırır.

**Kararları siz verirsiniz.** Portun davranışını düzelttiğinizde `rules --add "<rule>"` bunu saklar ve sonraki her ajan istemi onu içerir. rust-anything'in yazdığı her şey `<repo>/rust/` içinde durur.

**Rapor mu, taşıma mı, siz seçin.** `analyze` ve `profile` yalnızca rapor verir: neyin port edileceğine dair sıralı bir plan ve gerçek bir iş yükü altında neyin sıcak olduğu. Bunlar düz komutlardır, bu yüzden rapor hiç token harcamaz. `go` bir birimi taşır: en sıcak fonksiyon için köprü kurar, onu kaydeder, portu ajanınıza verir, sonra derler, karşılaştırır ve `gate`'ten geçirir; yalnızca her şey eşleşirse devreye alır (TS/JS, Python ve PowerShell; `disable` yeniden kapatır).

**Bir çalıştırma ne kadar sürer.** Bir `go` çalıştırması bir birimi port eder ve en fazla `--rounds` ajan turu kullanır (varsayılan 3): biri port için, sonra başarısız her diff ya da gate için bir tane. Bütün bir uygulama birim başına bir çalıştırmadır, yani herhangi birinden sonra durabilirsiniz.

**Plan için ucuz model, port için güçlü model.** Plan için model gerekmez. Port için herhangi bir ajan komutu verin: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Ajanlar.** `rust-anything setup` onu Claude Code, Codex, Cursor, Gemini CLI ve OpenCode'a ekler. Diğer MCP istemcileri `rust-anything mcp` çalıştırabilir; skill ayrıca `npx skills add erkamyaman/rust-anything` ile tek başına da kurulur (bkz. [kurulum](install.md)).

## Diller

| Dil | Durum | Nerede denendi |
| --- | --- | --- |
| TypeScript/JavaScript | Doğrulandı | ESLint, Prettier, json-server, Fuse.js, vercel/ms |
| Python | Doğrulandı | CPython'ın `colorsys` modülü (2,400/2,400 vaka) |
| Go | Doğrulandı | Go'nun `path` paketi (2,642/2,642) |
| Ruby | Doğrulandı | Ruby 3.3'ten `shellwords` (1,617/1,617) |
| Java | Doğrulandı | Bir cephe arkasında JDK 21 (4,492/4,492) |
| C | Doğrulandı | ffmpeg'in libavutil'i, glibc |
| C# | Doğrulandı | Google.Protobuf (2,941/2,941) |
| PHP | Doğrulandı | PEAR `XML_Util` (1,838/1,838) |
| Kotlin | Test edildi | bu reponun fixture'ları, Android dahil |
| PowerShell | Test edildi | bu reponun fixture'ları |
| Swift | Test edildi | bu reponun fixture'ları |

**Doğrulandı**, başkasının yazdığı gerçek kod üzerinde baştan sona port edildi demektir; **Test edildi**, bu reponun fixture'lar üzerindeki testleriyle kapsandı demektir. Her çalıştırma, tarihleri ve nasıl kontrol edildiğiyle: [STATUS.md](STATUS.md).

## Komutlar

| Komut | Ne yapar |
| --- | --- |
| `rust-anything go -- <workload>` | Profil çıkarır, en sıcak fonksiyon için köprü kurar, onu kaydeder, portu ajanınıza verir, gate'ten geçirir, devreye alır |
| `rust-anything analyze <repo>` | Sıralı taşıma planı: birimler, hazırlık, riskler, bağımlılıklar |
| `rust-anything profile <repo> -- <workload>` | Gerçek bir iş yükü altında neyin sıcak olduğu, neyin port edileceği ya da toplu çağrılacağı |
| `rust-anything bridge <repo> <unit>` | Bir birim için bağlamaları olan Rust crate'i, stub'lar `src/port.rs` içinde |
| `rust-anything record <crate>` | Orijinalin döndürdüğünü, fırlattığını ve yazdırdığını kaydeder (`--inputs`, `--from-tests`, `--pin-clock`) |
| `rust-anything diff <crate>` | Kaydedilen her vakayı Rust modülüne karşı yeniden oynatır |
| `rust-anything bench <crate>` | Orijinali ve Rust'ı kaydedilen vakalarda ölçer, köprü dahil |
| `rust-anything fuzz <crate>` | İki taraftan da yeni girdiler: kaydın kapsamadığı şeyler |
| `rust-anything gate <crate>` | Bitmiş sayılmanın tanımı |
| `rust-anything enable` / `disable <crate>` | Çağıranları Rust'a geçirir ya da geri alır |
| `rust-anything report <crate>` | `REPORT.md` ve bir pull request için kart |
| `rust-anything app record` / `front-door` / `wire` / `replay` / `lift` / `drift` | Bütün bir CLI'ı ya da sunucuyu port eder |
| `rust-anything rules <repo> --add "<rule>"` | Düzelttiğiniz davranış, sonraki her portta korunur |
| `rust-anything map <repo>` | Her birim, crate'i, vakaları ve son diff'i |
| `rust-anything log <repo>` | Her devreye alma ya da kapatma, her wire, lift ve gate sonucu, `rust/audit.jsonl` dosyasından |
| `rust-anything setup` / `doctor` / `mcp` | Ajanlar için kurulum, çalışma ortamı kontrolü, MCP sunucusu |

## Lisans

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Reponuzdaki portlar ve üretilmiş kod sizindir.
