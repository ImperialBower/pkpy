# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project tracks [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The crate version is kept in lockstep with the underlying `pkcore` dependency.

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
