# pkpy `PokerSession` + `TableNoCell` Bindings — Design

**Date:** 2026-04-29
**Scope:** Targeted slice of pkcore 0.0.53 — bind the no-cell session/table primitives needed to drive a multi-hand poker session from Python, with the new 0.0.53 blinds-management methods (`set_blinds`, `forced_at_hand_start`) folded in.
**Target version:** `pkpy 0.0.53` (no version bump; this work happens on top of the just-shipped 0.0.53 dep bump).

## Context

`pkcore 0.0.53` added `PokerSession::set_blinds` and `PokerSession::forced_at_hand_start` to support hand-history pipelines that need stable blinds across a hand. Surfacing those methods in pkpy requires `PokerSession` itself to be bound — and that requires its `TableNoCell` constructor argument, which in turn requires `SeatsNoCell`, `SeatNoCell`, `PlayerNoCell`. None of these are bound today; `src/session.rs` and `src/table_no_cell.rs` are the no-op scaffold modules created by the 0.0.52 bump (commit `b17c327`).

The existing 0.0.52 plan (`docs/superpowers/plans/2026-04-28-pkpy-0.0.52-upgrade.md`) sketched Phase 6 (PokerSession) and Phase 7 (TableNoCell), but (a) was written against pkcore 0.0.52 so it omits the new methods, (b) put the phases in the wrong dependency order (PokerSession before TableNoCell), and (c) guessed a constructor signature that doesn't match 0.0.53 (`PokerSession::new(forced, seats: u8)` was wrong; the actual signature is `PokerSession::new(table: TableNoCell)`). This spec supersedes those two phases.

**Intended outcome:** A Python user can construct a heads-up table, run a hand to completion, observe the event log, change blinds between hands, and access the new 0.0.53 stable-blinds snapshot — all without writing Rust or touching primitives via cumbersome literal-mirror constructors.

## Scope

**In scope:**

- `PokerSession` — full surface (12 methods + 2 field getters), excluding `run_hand<F>` (callback-style; not worth the FFI friction).
- `TableNoCell` — faithful constructor `nlh_from_seats(seats, forced)` plus convenience `heads_up(forced, stacks, names)`. Read-only inspection helpers.
- `SeatsNoCell`, `SeatNoCell`, `PlayerNoCell` — constructors + read-only inspection accessors. **No** `act_*` methods.
- `PlayerAction` — full enum, bound TableAction-style (static constructors + `kind()`/`amount()` accessors).
- `SessionStep` — read-only enum, bound TableAction-style (`kind()`/`seat()` accessors).

**Reused (already bound, no work):** `ForcedBets`, `Winnings`, `PotWin`.

**Out of scope (explicit):**

- Hand-history bindings (Phase 3 of the 0.0.52 plan).
- Player-stats bindings (Phase 4).
- Bot bindings (Phase 5).
- Action methods on `SeatsNoCell` / `SeatNoCell` / `PlayerNoCell` — `act_bet`, `act_raise`, `act_call`, `act_check`, `act_fold`, `act_all_in`, `act_forced_bet`, `bring_it_in`, `close_it_out`, etc. These are accessible through `PokerSession::apply_action`; binding them directly invites confusion with the existing cell-based `Dealer` action API.
- `PokerSession::run_hand<F>` (callback-driven hand runner). Python users can drive the session via `next_step` / `apply_action` / `end_hand` in a loop.
- Any pkpy version bump. This is feature work *under* the 0.0.53 line, not a new release.

## Architecture

The 0.0.52 bump already created the empty scaffold modules. We fill two of them:

| File | Status | Content |
|---|---|---|
| `src/table_no_cell.rs` | scaffold → bindings | `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell`, `TableNoCell` |
| `src/session.rs` | scaffold → bindings | `PlayerAction`, `SessionStep`, `PokerSession` |
| `src/lib.rs` | minor edit | `pub(crate)` confirmation for `ForcedBets` / `Winnings` / `PotWin` reachability from sibling modules. `mod` declarations + register calls already in place. |
| `python/pkpy/__init__.py` | re-export | Add the seven new classes. |
| `tests/test_session.py` | new | TDD tests for `PlayerAction`, `SessionStep`, `PokerSession`. |
| `tests/test_table_no_cell.py` | new | TDD tests for the four primitive types. |

**Build sequence (dependency order):**

1. `PlayerAction` + `SessionStep` in `session.rs` (no dependencies on other new types).
2. `PlayerNoCell` → `SeatNoCell` → `SeatsNoCell` → `TableNoCell` in `table_no_cell.rs` (each depends on the previous).
3. `PokerSession` in `session.rs` (depends on all of the above).

`make ayce` must stay green at each step's commit boundary.

## Conventions

- Existing pkpy convention: `#[pyclass(from_py_object, name = "X")] #[derive(Clone)] pub struct X(PkX);` with `#[pymethods]` block. Follow it everywhere.
- Errors propagate via the existing `to_py_err` helper (`lib.rs:61`, `pub(crate)`).
- Enum bindings follow `TableAction`'s pattern: opaque wrapper, `kind()` returning a `&'static str`, optional payload accessors (`seat()`, `amount()`). Construction-needing enums (`PlayerAction`) add `#[staticmethod]` constructors per variant.
- `raise` is a Python keyword, so the `PlayerAction::Raise` constructor binds as `raise_`. The trailing-underscore convention is standard Python (cf. `class_`, `from_`).
- `__repr__` on every type for usable Python REPL output.
- `__eq__` only on types with semantic equality (`PlayerAction`); not added speculatively elsewhere.

## Type Bindings (detail)

### `PlayerAction` (`src/session.rs`)

```rust
#[pyclass(from_py_object, name = "PlayerAction")]
#[derive(Clone)]
pub struct PlayerAction(pub(crate) PkPlayerAction);

#[pymethods]
impl PlayerAction {
    #[staticmethod] fn fold()                -> Self { Self(PkPlayerAction::Fold) }
    #[staticmethod] fn check()               -> Self { Self(PkPlayerAction::Check) }
    #[staticmethod] fn call()                -> Self { Self(PkPlayerAction::Call) }
    #[staticmethod] fn bet(amount: usize)    -> Self { Self(PkPlayerAction::Bet(amount)) }
    #[staticmethod] fn raise_(amount: usize) -> Self { Self(PkPlayerAction::Raise(amount)) }
    #[staticmethod] fn all_in()              -> Self { Self(PkPlayerAction::AllIn) }

    fn kind(&self) -> &'static str { /* "Fold"|"Check"|"Call"|"Bet"|"Raise"|"AllIn" */ }
    fn amount(&self) -> Option<usize> { /* Some(n) for Bet/Raise; None otherwise */ }
    fn __repr__(&self) -> String { /* e.g. "PlayerAction.Bet(200)" */ }
    fn __eq__(&self, other: &PlayerAction) -> bool { self.0 == other.0 }
}
```

### `SessionStep` (`src/session.rs`)

Read-only opaque wrapper, returned from `PokerSession::next_step`:

```rust
#[pyclass(from_py_object, name = "SessionStep")]
#[derive(Clone)]
pub struct SessionStep(PkSessionStep);

#[pymethods]
impl SessionStep {
    fn kind(&self) -> &'static str { /* "PlayerToAct"|"StreetAdvanced"|"HandComplete" */ }
    fn seat(&self) -> Option<u8> { /* Some(seat) for PlayerToAct; None otherwise */ }
    fn __repr__(&self) -> String { /* e.g. "SessionStep.PlayerToAct(seat=2)" */ }
}
```

### `PlayerNoCell` (`src/table_no_cell.rs`)

```rust
#[pyclass(from_py_object, name = "PlayerNoCell")]
#[derive(Clone)]
pub struct PlayerNoCell(pub(crate) PkPlayerNoCell);

#[pymethods]
impl PlayerNoCell {
    #[new]
    #[pyo3(signature = (handle, chips=0))]
    fn new(handle: String, chips: usize) -> Self {
        if chips == 0 { Self(PkPlayerNoCell::new(handle)) }
        else          { Self(PkPlayerNoCell::new_with_chips(handle, chips)) }
    }

    fn total_chip_count(&self) -> usize { self.0.total_chip_count() }
    fn is_active(&self)       -> bool { self.0.is_active() }
    fn is_all_in(&self)       -> bool { self.0.is_all_in() }
    fn is_in_hand(&self)      -> bool { self.0.is_in_hand() }
    fn is_out(&self)          -> bool { self.0.is_out() }
    fn is_tapped_out(&self)   -> bool { self.0.is_tapped_out() }
    fn is_clear(&self)        -> bool { self.0.is_clear() }
    fn has_bet(&self)         -> bool { self.0.has_bet() }
    fn __repr__(&self) -> String { format!("PlayerNoCell({})", self.0) }
}
```

### `SeatNoCell` (`src/table_no_cell.rs`)

```rust
#[pyclass(from_py_object, name = "SeatNoCell")]
#[derive(Clone)]
pub struct SeatNoCell(pub(crate) PkSeatNoCell);

#[pymethods]
impl SeatNoCell {
    #[new] fn new(player: &PlayerNoCell) -> Self { Self(PkSeatNoCell::new(player.0.clone())) }

    fn is_empty(&self)               -> bool { self.0.is_empty() }
    fn is_active(&self)              -> bool { self.0.is_active() }
    fn is_all_in(&self)              -> bool { self.0.is_all_in() }
    fn is_in_hand(&self)             -> bool { self.0.is_in_hand() }
    fn is_yet_to_act(&self)          -> bool { self.0.is_yet_to_act() }
    fn is_yet_to_act_or_blind(&self) -> bool { self.0.is_yet_to_act_or_blind() }
    fn is_clear(&self)               -> bool { self.0.is_clear() }
    fn __repr__(&self) -> String { format!("SeatNoCell({})", self.0) }
}
```

### `SeatsNoCell` (`src/table_no_cell.rs`)

```rust
#[pyclass(from_py_object, name = "SeatsNoCell")]
#[derive(Clone)]
pub struct SeatsNoCell(pub(crate) PkSeatsNoCell);

#[pymethods]
impl SeatsNoCell {
    #[new] fn new(seats: Vec<SeatNoCell>) -> Self {
        Self(PkSeatsNoCell::new(seats.into_iter().map(|s| s.0).collect()))
    }

    fn size(&self) -> u8 { self.0.size() }
    fn get_seat(&self, idx: u8) -> Option<SeatNoCell> {
        self.0.get_seat(idx).cloned().map(SeatNoCell)
    }
    fn is_seat_in_hand(&self, idx: u8) -> bool { self.0.is_seat_in_hand(idx) }
    fn current_bet(&self) -> usize { self.0.current_bet() }
    fn to_call(&self, player_idx: u8) -> usize { self.0.to_call(player_idx) }
    fn total_chip_count(&self) -> usize { self.0.total_chip_count() }
    fn count_active_in_hand(&self) -> usize { self.0.count_active_in_hand() }
    fn active_in_hand(&self) -> Vec<u8> { self.0.active_in_hand() }
    fn are_dealt(&self)          -> bool { self.0.are_dealt() }
    fn are_clear(&self)          -> bool { self.0.are_clear() }
    fn is_betting_complete(&self) -> bool { self.0.is_betting_complete() }
    fn __repr__(&self) -> String { format!("SeatsNoCell(size={})", self.0.size()) }
}
```

### `TableNoCell` (`src/table_no_cell.rs`)

Faithful pkcore mirror plus a Python-friendly heads-up factory.

```rust
#[pyclass(from_py_object, name = "TableNoCell")]
#[derive(Clone)]
pub struct TableNoCell(pub(crate) PkTableNoCell);

#[pymethods]
impl TableNoCell {
    /// Faithful pkcore mirror.
    #[staticmethod]
    fn nlh_from_seats(seats: &SeatsNoCell, forced: &ForcedBets) -> Self {
        Self(PkTableNoCell::nlh_from_seats(seats.0.clone(), forced.0))
    }

    /// Convenience: heads-up table with two named, equally-stacked players.
    #[staticmethod]
    #[pyo3(signature = (forced, stacks=(1000, 1000), names=("A".to_string(), "B".to_string())))]
    fn heads_up(forced: &ForcedBets, stacks: (usize, usize), names: (String, String)) -> Self {
        let seats = PkSeatsNoCell::new(vec![
            PkSeatNoCell::new(PkPlayerNoCell::new_with_chips(names.0, stacks.0)),
            PkSeatNoCell::new(PkPlayerNoCell::new_with_chips(names.1, stacks.1)),
        ]);
        Self(PkTableNoCell::nlh_from_seats(seats, forced.0))
    }

    fn seat_count(&self) -> u8 { self.0.seats.size() }
    fn determine_small_blind(&self) -> u8 { self.0.determine_small_blind() }
    fn determine_big_blind(&self) -> u8 { self.0.determine_big_blind() }
    fn next_occupied_seat_after(&self, start: u8, n: usize) -> u8 {
        self.0.next_occupied_seat_after(start, n)
    }
    fn seats(&self) -> SeatsNoCell { SeatsNoCell(self.0.seats.clone()) }
    fn __repr__(&self) -> String { format!("TableNoCell(seats={})", self.0.seats.size()) }
}
```

### `PokerSession` (`src/session.rs`)

```rust
#[pyclass(name = "PokerSession")]
pub struct PokerSession(pub(crate) PkPokerSession);

#[pymethods]
impl PokerSession {
    #[new] fn new(table: &TableNoCell) -> Self { Self(PkPokerSession::new(table.0.clone())) }

    /// Convenience: heads-up session in one call.
    #[staticmethod]
    #[pyo3(signature = (forced, stacks=(1000, 1000), names=("A".to_string(), "B".to_string())))]
    fn heads_up(forced: &ForcedBets, stacks: (usize, usize), names: (String, String)) -> Self {
        let table = TableNoCell::heads_up(forced, stacks, names);
        Self(PkPokerSession::new(table.0))
    }

    // 0.0.53 NEW
    fn set_blinds(&mut self, forced: &ForcedBets) { self.0.set_blinds(forced.0) }
    fn forced_at_hand_start(&self) -> ForcedBets { ForcedBets(self.0.forced_at_hand_start()) }

    // Hand lifecycle
    fn start_hand(&mut self) -> PyResult<()>     { self.0.start_hand().map_err(to_py_err) }
    fn end_hand(&mut self)   -> PyResult<Winnings> { self.0.end_hand().map(Winnings).map_err(to_py_err) }
    fn is_hand_in_progress(&self) -> bool { self.0.is_hand_in_progress() }
    fn is_hand_complete(&self)    -> bool { self.0.is_hand_complete() }
    fn next_actor(&mut self)      -> Option<u8> { self.0.next_actor() }
    fn next_step(&mut self)       -> SessionStep { SessionStep(self.0.next_step()) }
    fn apply_action(&mut self, seat: u8, action: &PlayerAction) -> PyResult<()> {
        self.0.apply_action(seat, action.0).map_err(to_py_err)
    }

    // Session-wide
    fn count_funded(&self)         -> usize    { self.0.count_funded() }
    fn eliminate_busted(&mut self) -> Vec<u8>  { self.0.eliminate_busted() }

    // Field accessors (pkcore exposes these as plain pub fields)
    #[getter] fn hand_number(&self)       -> u64            { self.0.hand_number }
    #[getter] fn shuffled_deck_str(&self) -> Option<String> { self.0.shuffled_deck_str.clone() }
}
```

## Test Plan

TDD-style: write failing test → implement → make green. One test file per source module.

### `tests/test_table_no_cell.py`

- `PlayerNoCell` construction with and without chips; default state checks.
- `SeatNoCell` wrapping a `PlayerNoCell`; empty/active state.
- `SeatsNoCell` from a list; `size`, `get_seat(idx)`, `total_chip_count`, `active_in_hand`.
- `TableNoCell.nlh_from_seats` literal-mirror construction.
- `TableNoCell.heads_up` convenience constructor + default-stack/name verification.
- `seat_count`, `determine_small_blind`, `determine_big_blind`.

### `tests/test_session.py`

- `PlayerAction.fold/check/call/bet/raise_/all_in` constructors; `kind()` and `amount()` accessors; `__eq__`.
- `SessionStep.kind()` shapes — `PlayerToAct` returns a `seat()`; `StreetAdvanced` and `HandComplete` return `None`.
- `PokerSession.heads_up(...)` constructs successfully; `start_hand()` succeeds; `next_step()` returns `PlayerToAct`.
- **0.0.53 regression ports** (direct translations of pkcore's three new unit tests at `casino/session.rs:970-1010`):
  - `set_blinds` between hands applies immediately (next `start_hand` posts new blinds).
  - `set_blinds` mid-hand defers — current hand's blinds unchanged until next `start_hand`.
  - `forced_at_hand_start` returns the snapshot taken at the most recent `start_hand`, stable across mid-hand `set_blinds`.
- End-to-end: heads-up hand played to fold → `end_hand` returns a `Winnings` with `len() > 0`.

## Verification

- `make ayce` (fmt + maturin develop + pytest + demo) passes after each commit in the build sequence.
- `pytest tests/test_table_no_cell.py tests/test_session.py -v` — all new tests pass.
- Spot-check via REPL:
  ```python
  from pkpy import PokerSession, ForcedBets, PlayerAction
  s = PokerSession.heads_up(ForcedBets(50, 100))
  s.start_hand()
  print(s.next_step())                 # SessionStep.PlayerToAct(seat=...)
  s.set_blinds(ForcedBets(100, 200))   # deferred
  print(s.forced_at_hand_start())      # still ForcedBets(50, 100)
  ```

## Risks / Open Questions

- **`PlayerNoCell::new_with_chips` vs `new`:** the chips=0 sentinel-based dispatch in the constructor is mildly awkward. Acceptable because pkcore's `PlayerNoCell::new` produces a player with 0 chips anyway, so the two paths are observationally equivalent for `chips=0`. If pkcore later diverges these constructors, switch to two named static methods (`PlayerNoCell.new_handle`, `PlayerNoCell.with_chips`).
- **`TableNoCell.seats.size()` reaches into a public field.** If pkcore later seals that field, change the binding to `self.0.seats().size()` or equivalent accessor.
- **`SessionStep` `Display` impl** — pkcore may not expose `Display` for `SessionStep`. The `__repr__` implementation may need to construct the string explicitly from `kind()` + `seat()` rather than delegating. Verify when implementing; trivial either way.

## Notes for the implementing agent

- Use the `to_py_err` helper that's already `pub(crate)` from the 0.0.52 work — don't re-derive it.
- Re-imports at the top of each new module file follow the existing pattern in `lib.rs:30` (`use pkcore::casino::game::ForcedBets as PkForcedBets;`).
- `from_py_object` on the `#[pyclass]` attribute is the existing pkpy convention; preserve it. (`PokerSession` is the exception — pkcore's `PokerSession` doesn't implement `Clone`, so `from_py_object` won't work; use plain `#[pyclass]` and accept `&PokerSession` references in any future binding that needs one.)
- `python/pkpy/__init__.py` re-exports: add `PokerSession`, `PlayerAction`, `SessionStep`, `TableNoCell`, `SeatsNoCell`, `SeatNoCell`, `PlayerNoCell` to the import block and the `__all__` if there is one.
