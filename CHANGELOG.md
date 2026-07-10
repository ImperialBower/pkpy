# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project tracks [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The crate version is kept in lockstep with the underlying `pkcore` dependency.

## [0.2.1] - 2026-07-10

### Changed

- Bumped `pkcore` dependency from `0.2.0` to `0.2.1`.
- Bumped `pkpy` crate version to `0.2.1` to stay in lockstep with `pkcore`.

### Security (inherited from `pkcore` 0.2.1)

`pkcore` 0.2.1 is a dependency-hygiene patch with **no public API, behavior, or
wire-format changes** — the postcard binary encoding is byte-identical, so solver
caches and hand-history data are unaffected. It carries two supply-chain fixes that
flow through to pkpy's dependency tree:

- **`crossbeam-epoch` 0.9.18 → 0.9.20 (RUSTSEC-2026-0204).** Fixes an invalid pointer
  dereference in `crossbeam-epoch`'s `fmt::Pointer`/`Display` impl. Pulled in
  transitively via `rayon`; a lockfile-only change.
- **`atomic-polyfill` (RUSTSEC-2023-0089) removed from the tree.** `pkcore` now builds
  `postcard` with `default-features = false`, dropping the default `heapless-cas`
  feature that pulled the unmaintained `atomic-polyfill` crate. It is gone from
  pkpy's dependency graph after this bump.

### Migration notes

- Public Rust API of `pkcore` is **unchanged**. No method signatures, types, or
  imports moved. pkpy compiles clean against `pkcore 0.2.1` with no source changes
  (verified via `cargo check` against local `pkcore 0.2.1`).
- No Python-facing behavior changes: the same bindings, method names, and return
  values as under `pkcore 0.2.0`.

---

## [0.2.0] - 2026-07-08

> **Never published to PyPI.** This version was tagged in-tree but not released; its
> changes reach users for the first time bundled into 0.2.1. It is the first pkpy build
> on the `pkcore` 0.2.x line — pkpy migrated **directly from `pkcore` 0.0.54**, skipping
> the 0.1.x series.

### Changed

- Bumped `pkcore` dependency from `0.0.54` to `0.2.0`, and enabled pkcore's **`store`
  feature** explicitly. `pkcore` 0.2.0 moved storage/BCM and solver persistence behind
  cargo features (previously always compiled in); pkpy now opts into `store` so BCM
  loading and `SolverResult` save/load continue to work.
- Bumped `pkpy` crate version to `0.2.0` to stay in lockstep with `pkcore`.
- **Internal migration to pkcore 0.2.0's reorganized module tree** (the `casino` package
  reorg and the `TableNoCell → Table` type rename). Import paths in the Rust binding layer
  were updated (`casino::table::event` → `casino::action` / `casino::table_celled::event`,
  `casino::table::seats::*` → `casino::equity::*`, `casino::table::winnings` →
  `casino::winnings`, `casino::table_no_cell::TableNoCell` → `casino::table::Table`, etc.).

  **No Python-facing class or method names changed.** pkpy deliberately preserves its
  existing Python names — `TableNoCell`, `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell`,
  `TableAction`, `TableLog`, `SeatEquity`, `Seatbit`, `Winnings`, `PotWin`, and the rest —
  so existing Python code imports and calls exactly the same symbols. The pkcore rename is
  invisible from Python.

### Behavior change (inherited from `pkcore` 0.2.0)

- **`DealEval(hole_cards)` is now fallible.** The constructor previously always succeeded;
  it now raises a Python exception when the hole cards are invalid. This follows pkcore
  0.2.0 changing `DealEval::new` to return a `Result` as part of the panic-boundary /
  error de-leak audit work. Wrap `DealEval(...)` in `try/except` if you pass unvalidated
  input; the happy path is unchanged.
- Errors surfaced from the dealer/eval paths are now pkcore's own error enums (the 0.2.0
  "no format-crate leak" change). pkpy still maps them to Python exceptions via the same
  `to_py_err` path, so raised exception messages may differ slightly from 0.0.54.

### Migration notes

- **No Python source changes required** for typical usage: same class names, same methods,
  same imports.
- The one behavioral gotcha is `DealEval(...)` now raising on invalid hole cards instead of
  being infallible.
- Card `Display` / `FromStr` string forms (e.g. `"6♠ 6♥"`) and serialized representations
  are unchanged, so data produced under 0.0.54 remains readable.

---

## [0.0.54] - 2026-04-30

### Changed

- Bumped `pkcore` dependency from `0.0.53` to `0.0.54`.
- Bumped `pkpy` crate version to `0.0.54` to stay in lockstep with `pkcore`.

### Fixed (inherited from `pkcore` 0.0.54)

No pkpy code was modified for this release, but the upstream fix changes
observable behavior on one Python-exposed method, `TableNoCell.to_call()`.

- **Short-stacked big blind — call target now anchored to the configured BB.**
  When the BB is all-in for less than the configured big blind (e.g. BB=100
  but stack=30), `TableNoCell.to_call()` now returns the full configured BB
  (`100`) instead of the amount the BB physically posted (`30`). Other
  players must call the full configured amount; chip conservation is
  preserved at showdown via side-pot stratification (multiway) or
  uncalled-bet returns (heads-up / no second contestant at that tier).
  This matches standard cardroom rules (TDA, WSOP).
- **`act_call` now degrades gracefully when the caller is short.** When a
  caller cannot cover the call target, the action is converted to an
  all-in for the caller's remaining stack rather than erroring on
  insufficient chips. Surfaced through pkpy via `PokerSession.apply_action`.
- **`min_raise` stays anchored to the configured BB** even when the BB is
  all-in for less. Prior behavior could allow under-sized raises in the
  short-BB scenario.

### Migration notes

- Public Rust API of `pkcore` is **unchanged**. No method signatures, types,
  or imports moved. pkpy compiles clean against `pkcore 0.0.54` with no
  source changes.
- If you have Python code that asserts specific chip math against a
  short-stacked-BB scenario built on pkcore 0.0.53 semantics, those
  assertions will need to be updated. The pkpy test suite does not
  currently exercise this scenario, so the in-tree tests remain green.

---

Earlier releases pre-date this changelog. See `git log` for prior history.
