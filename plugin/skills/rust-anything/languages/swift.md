# Swift ports

A unit is a Swift file's top-level functions (not methods, not functions in an `extension`). The
bridge is a JSON dispatcher over the C ABI: a generated `swift/<Crate>.swift` redeclares each
function with its original labels, defaults and `throws`, and reaches Rust through the C module in
`swift/<Crate>Native/` (`module.modulemap` and a header). There is no `shadow`, `--batch`,
`bench` or `profile` yet: once `diff` passes, compile the wrapper into the target in place of the
original file, with `-I swift` and `-L swift` (or a SwiftPM system-library target). Closures,
`inout`, variadics, generics, `async`, `rethrows`, tuples, `Set`, `Character`, `UInt`/`UInt64`,
Foundation types (`Date`, `Data`, `URL`) and the project's own structs, classes and enums stay in
Swift. Needs `swiftc` (Xcode or a Swift 5.9+ toolchain).

## Constructs

- **Errors**: Swift throws values, and `record` keeps `String(describing: error)` and the error's
  type name: `ScoreError.outOfRange(150)` is recorded as `outOfRange(150)` of type `ScoreError`.
  Return the same text: `Err(format!("ScoreError: outOfRange({n})"))`. `diff` compares the type
  and the text. The wrapper throws `<Crate>Error` with `typeName` and `message`, so code that
  catches `ScoreError` itself must catch that instead.
- **Traps**: Swift stops the program on `Int` overflow, `!` on `nil`, an index out of range,
  `Int(x)` of a `Double` out of range and division by zero; `record` keeps such a case as a trap
  (`"error_type": "trap"`). The crate builds with `overflow-checks = true` in release, so `a + b`
  panics on overflow, and the wrapper turns a Rust panic into `fatalError`: `diff` compares a trap
  with a trap. Never write `wrapping_add` where Swift writes `+`; `&+`, `&-` and `&*` do wrap.
- **Numbers**: `Int`, `Int32` and the other signed and narrow widths are `i64` in the port; an
  input that does not fit the declared width is not recorded. `Float` is `f64` in the port while
  Swift computes in single precision: compute in `f32` and widen. `1 << 70` is `0` in Swift and a
  panic in Rust: write `checked_shl(n).unwrap_or(0)`.
- **Optionals and collections**: `T?` is `Option<T>`, `[T]` is `Vec<T>`, and `[String: T]` is
  `serde_json::Value`. A `Dictionary` iterates in a different order on every run; results are
  recorded with their keys sorted, so a port may build them in any order, but code that returns
  the order of iteration (`Array(d.keys)`) answers differently on every run.

## Traps

- **Graphemes**: `String.count`, indices, `prefix`, `reversed` and `first` work on grapheme
  clusters (`"👍🏽".count` and `"e\u{301}".count` are 1). Count with `unicode-segmentation`'s
  `graphemes(true)`, not `chars()`. `==` compares canonical equivalence (`"é" == "e\u{301}"` is
  true): compare NFC forms (`unicode-normalization`).
- **Casing**: `lowercased()` maps each character on its own, with no final-sigma rule (`"ΣΑΣ"`
  becomes `"σασ"`), where Rust's `str::to_lowercase` writes `ς` at the end of a word: collect
  `chars().flat_map(char::to_lowercase)` instead.
- **Double to text**: `"\(x)"` and `String(x)` print the shortest digits that read back the same,
  with `.0` on whole numbers (`1.0`), an exponent from 1e16 up and below 1e-4 (`1e+16`, `1e-05`),
  and `inf`, `-inf`, `nan`. Rust's `{}` prints `1`, `10000000000000000` and `0.00001`: write the
  rules out, and record whole, tiny and huge values.
- **Splitting**: `split(separator: " ")` compares whole characters, so a space followed by a
  combining mark is not a separator, and empty pieces are dropped.
- **Clock and random** (`Date()`, `.random(in:)`, `UUID()`): `--pin-clock` does not cover Swift;
  pass the time and the seed in as parameters.
