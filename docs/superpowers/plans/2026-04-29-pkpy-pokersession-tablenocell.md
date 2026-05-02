# pkpy PokerSession + TableNoCell Bindings Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bind the no-cell session/table primitives from pkcore 0.0.53 to Python so a user can drive a multi-hand poker session end-to-end, with the new 0.0.53 blinds-management methods (`set_blinds`, `forced_at_hand_start`) folded in.

**Architecture:** Two pkpy scaffold modules (`src/session.rs`, `src/table_no_cell.rs`) get filled in. Build bottom-up by dependency: enums first (no deps), then primitives in dependency order (`PlayerNoCell` → `SeatNoCell` → `SeatsNoCell` → `TableNoCell`), then `PokerSession` last. Each task ships with TDD-style tests; `make ayce` stays green at every commit boundary.

**Tech Stack:** Rust 1.94+ (edition 2021), pyo3 0.28, maturin 1.7+, pkcore 0.0.53 (default features), pytest.

**Reference spec:** `docs/superpowers/specs/2026-04-29-pkpy-pokersession-tablenocell-design.md`

**Note on git commits:** This project's owner runs all `git` state-changing commands manually (per global CLAUDE.md). Each task ends with a "suggested commit" command — the executor should print/log it for the owner rather than running `git add` / `git commit` directly.

---

## File Map

| File | Action | Purpose |
|---|---|---|
| `src/session.rs` | Modify (currently 7-line scaffold) | Bind `PlayerAction`, `SessionStep`, `PokerSession` |
| `src/table_no_cell.rs` | Modify (currently 7-line scaffold) | Bind `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell`, `TableNoCell` |
| `python/pkpy/__init__.py` | Modify | Re-export the seven new classes |
| `tests/test_session.py` | Create | Tests for `PlayerAction`, `SessionStep`, `PokerSession` |
| `tests/test_table_no_cell.py` | Create | Tests for `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell`, `TableNoCell` |

`src/lib.rs` requires no changes — `mod session;` / `mod table_no_cell;` and their register calls were added in commit `b17c327`. The needed types `ForcedBets`, `Winnings`, `PotWin`, `to_py_err` are already at crate-visible scope.

---

## Conventions

All new pyo3 bindings follow the established pkpy pattern, exemplified by `TableAction` (`src/lib.rs:2683`) and `Dealer` (`src/lib.rs:2823`):

```rust
#[pyclass(from_py_object, name = "X")]
#[derive(Clone)]
pub struct X(pub(crate) PkX);

#[pymethods]
impl X {
    // methods
}
```

`PokerSession` is the one exception — `pkcore::casino::session::PokerSession` does **not** derive `Clone`, so it uses plain `#[pyclass]` (no `from_py_object`).

Errors map via the existing `pub(crate) fn to_py_err` helper at `src/lib.rs:61`.

Each new module starts with a `pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()>` that adds every class. The `#[pymodule]` block at `src/lib.rs:3937` already calls `session::register(m)?` and `table_no_cell::register(m)?`.

---

## Phase 1 — `PlayerAction` enum

### Task 1: Bind `PlayerAction`

**Files:**
- Modify: `src/session.rs`
- Create: `tests/test_session.py`

- [ ] **Step 1: Write failing tests for `PlayerAction`**

Create `tests/test_session.py`:

```python
"""Tests for pkpy poker session bindings."""

import pytest

from pkpy import PlayerAction


class TestPlayerAction:
    def test_fold(self):
        a = PlayerAction.fold()
        assert a.kind() == "Fold"
        assert a.amount() is None

    def test_check(self):
        a = PlayerAction.check()
        assert a.kind() == "Check"
        assert a.amount() is None

    def test_call(self):
        a = PlayerAction.call()
        assert a.kind() == "Call"
        assert a.amount() is None

    def test_bet(self):
        a = PlayerAction.bet(200)
        assert a.kind() == "Bet"
        assert a.amount() == 200

    def test_raise_(self):
        a = PlayerAction.raise_(400)
        assert a.kind() == "Raise"
        assert a.amount() == 400

    def test_all_in(self):
        a = PlayerAction.all_in()
        assert a.kind() == "AllIn"
        assert a.amount() is None

    def test_equality(self):
        assert PlayerAction.bet(200) == PlayerAction.bet(200)
        assert PlayerAction.bet(200) != PlayerAction.bet(300)
        assert PlayerAction.fold() == PlayerAction.fold()
        assert PlayerAction.fold() != PlayerAction.check()

    def test_repr_contains_kind(self):
        assert "Bet" in repr(PlayerAction.bet(200))
        assert "200" in repr(PlayerAction.bet(200))
        assert "Fold" in repr(PlayerAction.fold())
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_session.py -v`
Expected: ImportError — `PlayerAction` does not exist in `pkpy`.

- [ ] **Step 3: Implement `PlayerAction` binding**

Replace the contents of `src/session.rs` with:

```rust
//! Bindings for pkcore's casino::session module.

use pkcore::casino::action::PlayerAction as PkPlayerAction;
use pyo3::prelude::*;

/// A player's action in a poker hand.
///
/// Construct via the static methods (`fold()`, `check()`, `call()`,
/// `bet(n)`, `raise_(n)`, `all_in()`). Inspect via `kind()` and `amount()`.
///
/// `raise` is a Python keyword, hence the trailing-underscore naming
/// convention for that constructor.
#[pyclass(from_py_object, name = "PlayerAction")]
#[derive(Clone)]
pub struct PlayerAction(pub(crate) PkPlayerAction);

#[pymethods]
impl PlayerAction {
    #[staticmethod]
    fn fold() -> Self {
        Self(PkPlayerAction::Fold)
    }

    #[staticmethod]
    fn check() -> Self {
        Self(PkPlayerAction::Check)
    }

    #[staticmethod]
    fn call() -> Self {
        Self(PkPlayerAction::Call)
    }

    #[staticmethod]
    fn bet(amount: usize) -> Self {
        Self(PkPlayerAction::Bet(amount))
    }

    #[staticmethod]
    #[pyo3(name = "raise_")]
    fn raise_(amount: usize) -> Self {
        Self(PkPlayerAction::Raise(amount))
    }

    #[staticmethod]
    fn all_in() -> Self {
        Self(PkPlayerAction::AllIn)
    }

    fn kind(&self) -> &'static str {
        match self.0 {
            PkPlayerAction::Fold => "Fold",
            PkPlayerAction::Check => "Check",
            PkPlayerAction::Call => "Call",
            PkPlayerAction::Bet(_) => "Bet",
            PkPlayerAction::Raise(_) => "Raise",
            PkPlayerAction::AllIn => "AllIn",
        }
    }

    fn amount(&self) -> Option<usize> {
        match self.0 {
            PkPlayerAction::Bet(n) | PkPlayerAction::Raise(n) => Some(n),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match self.0 {
            PkPlayerAction::Bet(n) => format!("PlayerAction.Bet({n})"),
            PkPlayerAction::Raise(n) => format!("PlayerAction.Raise({n})"),
            other => format!("PlayerAction.{other:?}"),
        }
    }

    fn __eq__(&self, other: &PlayerAction) -> bool {
        self.0 == other.0
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerAction>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `PlayerAction` to `python/pkpy/__init__.py`**

Add `PlayerAction` to the `from pkpy._pkpy import (...)` import block (alphabetical order).

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_session.py -v`
Expected: 8 passed.

- [ ] **Step 6: Suggested commit (owner runs)**

```
git add src/session.rs python/pkpy/__init__.py tests/test_session.py && git commit -m "feat: bind PlayerAction enum with TableAction-style accessors"
```

---

## Phase 2 — `SessionStep` enum

### Task 2: Bind `SessionStep`

**Files:**
- Modify: `src/session.rs`
- Modify: `tests/test_session.py`

- [ ] **Step 1: Write failing tests for `SessionStep`**

Append to `tests/test_session.py`:

```python
class TestSessionStep:
    """SessionStep is read-only — produced by PokerSession.next_step().

    These tests construct one indirectly via a session in Phase 7. For now,
    we just confirm the type exists in the module so import doesn't fail.
    """

    def test_import(self):
        from pkpy import SessionStep
        # Class must exist; instances are created by PokerSession.next_step.
        assert SessionStep is not None
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_session.py::TestSessionStep -v`
Expected: ImportError — `SessionStep` does not exist in `pkpy`.

- [ ] **Step 3: Add `SessionStep` binding to `src/session.rs`**

Append to `src/session.rs` (after the `PlayerAction` block, before `register`):

```rust
use pkcore::casino::session::SessionStep as PkSessionStep;

/// A snapshot of where a hand is in its lifecycle.
///
/// Returned by `PokerSession.next_step()`. Read-only; inspect via `kind()`
/// and (for `PlayerToAct`) `seat()`.
#[pyclass(from_py_object, name = "SessionStep")]
#[derive(Clone)]
pub struct SessionStep(pub(crate) PkSessionStep);

#[pymethods]
impl SessionStep {
    fn kind(&self) -> &'static str {
        match self.0 {
            PkSessionStep::PlayerToAct(_) => "PlayerToAct",
            PkSessionStep::StreetAdvanced => "StreetAdvanced",
            PkSessionStep::HandComplete => "HandComplete",
        }
    }

    fn seat(&self) -> Option<u8> {
        match self.0 {
            PkSessionStep::PlayerToAct(s) => Some(s),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match self.0 {
            PkSessionStep::PlayerToAct(s) => format!("SessionStep.PlayerToAct(seat={s})"),
            PkSessionStep::StreetAdvanced => "SessionStep.StreetAdvanced".to_string(),
            PkSessionStep::HandComplete => "SessionStep.HandComplete".to_string(),
        }
    }
}
```

Update the `register` function at the bottom of `src/session.rs`:

```rust
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerAction>()?;
    m.add_class::<SessionStep>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `SessionStep` to `python/pkpy/__init__.py`**

Add `SessionStep` to the import block (alphabetical order).

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_session.py -v`
Expected: 9 passed (8 from Task 1 + 1 new).

- [ ] **Step 6: Suggested commit**

```
git add src/session.rs python/pkpy/__init__.py tests/test_session.py && git commit -m "feat: bind SessionStep enum (read-only, returned by next_step)"
```

---

## Phase 3 — `PlayerNoCell`

### Task 3: Bind `PlayerNoCell`

**Files:**
- Modify: `src/table_no_cell.rs`
- Create: `tests/test_table_no_cell.py`

- [ ] **Step 1: Write failing tests for `PlayerNoCell`**

Create `tests/test_table_no_cell.py`:

```python
"""Tests for pkpy no-cell table primitive bindings."""

import pytest

from pkpy import PlayerNoCell


class TestPlayerNoCell:
    def test_construct_default_chips(self):
        p = PlayerNoCell("Alice")
        assert p.total_chip_count() == 0
        assert p.is_clear()

    def test_construct_with_chips(self):
        p = PlayerNoCell("Alice", chips=1000)
        assert p.total_chip_count() == 1000

    def test_construct_with_positional_chips(self):
        p = PlayerNoCell("Alice", 1000)
        assert p.total_chip_count() == 1000

    def test_state_predicates_default(self):
        p = PlayerNoCell("Alice", chips=1000)
        assert not p.is_all_in()
        assert not p.has_bet()

    def test_repr_contains_handle(self):
        r = repr(PlayerNoCell("Alice", chips=1000))
        assert "Alice" in r
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_table_no_cell.py -v`
Expected: ImportError — `PlayerNoCell` does not exist in `pkpy`.

- [ ] **Step 3: Implement `PlayerNoCell` binding**

Replace the contents of `src/table_no_cell.rs` with:

```rust
//! Bindings for pkcore's casino::table_no_cell module.

use pkcore::casino::table_no_cell::PlayerNoCell as PkPlayerNoCell;
use pyo3::prelude::*;

/// A no-cell player record (handle + chip stack + state flags).
///
/// Constructed standalone or via `PlayerNoCell(handle, chips=N)`. Wrapped
/// in `SeatNoCell` for table assembly.
#[pyclass(from_py_object, name = "PlayerNoCell")]
#[derive(Clone)]
pub struct PlayerNoCell(pub(crate) PkPlayerNoCell);

#[pymethods]
impl PlayerNoCell {
    #[new]
    #[pyo3(signature = (handle, chips=0))]
    fn new(handle: String, chips: usize) -> Self {
        if chips == 0 {
            Self(PkPlayerNoCell::new(handle))
        } else {
            Self(PkPlayerNoCell::new_with_chips(handle, chips))
        }
    }

    fn total_chip_count(&self) -> usize {
        self.0.total_chip_count()
    }

    fn is_active(&self) -> bool {
        self.0.is_active()
    }

    fn is_all_in(&self) -> bool {
        self.0.is_all_in()
    }

    fn is_in_hand(&self) -> bool {
        self.0.is_in_hand()
    }

    fn is_out(&self) -> bool {
        self.0.is_out()
    }

    fn is_tapped_out(&self) -> bool {
        self.0.is_tapped_out()
    }

    fn is_clear(&self) -> bool {
        self.0.is_clear()
    }

    fn has_bet(&self) -> bool {
        self.0.has_bet()
    }

    fn __repr__(&self) -> String {
        format!("PlayerNoCell({})", self.0)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `PlayerNoCell` to `python/pkpy/__init__.py`**

Add `PlayerNoCell` to the import block.

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_table_no_cell.py -v`
Expected: 5 passed.

- [ ] **Step 6: Suggested commit**

```
git add src/table_no_cell.rs python/pkpy/__init__.py tests/test_table_no_cell.py && git commit -m "feat: bind PlayerNoCell with construction and state inspection"
```

---

## Phase 4 — `SeatNoCell`

### Task 4: Bind `SeatNoCell`

**Files:**
- Modify: `src/table_no_cell.rs`
- Modify: `tests/test_table_no_cell.py`

- [ ] **Step 1: Append failing tests for `SeatNoCell`**

Append to `tests/test_table_no_cell.py`:

```python
class TestSeatNoCell:
    def test_construct_from_player(self):
        from pkpy import SeatNoCell
        seat = SeatNoCell(PlayerNoCell("Alice", chips=1000))
        assert not seat.is_empty()

    def test_default_state_predicates(self):
        from pkpy import SeatNoCell
        seat = SeatNoCell(PlayerNoCell("Alice", chips=1000))
        # Before any hand starts, the seat should not be in a hand.
        assert not seat.is_in_hand()
        assert not seat.is_all_in()

    def test_repr_contains_handle(self):
        from pkpy import SeatNoCell
        r = repr(SeatNoCell(PlayerNoCell("Alice", chips=1000)))
        assert "Alice" in r
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_table_no_cell.py::TestSeatNoCell -v`
Expected: ImportError — `SeatNoCell` does not exist in `pkpy`.

- [ ] **Step 3: Append `SeatNoCell` binding to `src/table_no_cell.rs`**

Add to the `use` block at the top:

```rust
use pkcore::casino::table_no_cell::SeatNoCell as PkSeatNoCell;
```

Append (after the `PlayerNoCell` block, before `register`):

```rust
/// A seat at a no-cell table, wrapping a `PlayerNoCell`.
#[pyclass(from_py_object, name = "SeatNoCell")]
#[derive(Clone)]
pub struct SeatNoCell(pub(crate) PkSeatNoCell);

#[pymethods]
impl SeatNoCell {
    #[new]
    fn new(player: &PlayerNoCell) -> Self {
        Self(PkSeatNoCell::new(player.0.clone()))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn is_active(&self) -> bool {
        self.0.is_active()
    }

    fn is_all_in(&self) -> bool {
        self.0.is_all_in()
    }

    fn is_in_hand(&self) -> bool {
        self.0.is_in_hand()
    }

    fn is_yet_to_act(&self) -> bool {
        self.0.is_yet_to_act()
    }

    fn is_yet_to_act_or_blind(&self) -> bool {
        self.0.is_yet_to_act_or_blind()
    }

    fn is_clear(&self) -> bool {
        self.0.is_clear()
    }

    fn __repr__(&self) -> String {
        format!("SeatNoCell({})", self.0)
    }
}
```

Update `register`:

```rust
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    m.add_class::<SeatNoCell>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `SeatNoCell` to `python/pkpy/__init__.py`**

Add `SeatNoCell` to the import block.

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_table_no_cell.py -v`
Expected: 8 passed (5 from Task 3 + 3 new).

- [ ] **Step 6: Suggested commit**

```
git add src/table_no_cell.rs python/pkpy/__init__.py tests/test_table_no_cell.py && git commit -m "feat: bind SeatNoCell with construction and state inspection"
```

---

## Phase 5 — `SeatsNoCell`

### Task 5: Bind `SeatsNoCell`

**Files:**
- Modify: `src/table_no_cell.rs`
- Modify: `tests/test_table_no_cell.py`

- [ ] **Step 1: Append failing tests for `SeatsNoCell`**

Append to `tests/test_table_no_cell.py`:

```python
class TestSeatsNoCell:
    def _two_seats(self):
        from pkpy import SeatNoCell
        return [
            SeatNoCell(PlayerNoCell("Alice", chips=1000)),
            SeatNoCell(PlayerNoCell("Bob", chips=2000)),
        ]

    def test_construct_from_list(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.size() == 2

    def test_total_chip_count(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.total_chip_count() == 3000

    def test_get_seat(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        seat = seats.get_seat(0)
        assert seat is not None
        assert not seat.is_empty()

    def test_get_seat_out_of_range(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.get_seat(99) is None

    def test_default_betting_state(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        # Before any hand starts, no betting state.
        assert seats.current_bet() == 0
        assert seats.count_active_in_hand() == 0
        assert not seats.are_dealt()

    def test_repr_includes_size(self):
        from pkpy import SeatsNoCell
        r = repr(SeatsNoCell(self._two_seats()))
        assert "size=2" in r
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_table_no_cell.py::TestSeatsNoCell -v`
Expected: ImportError — `SeatsNoCell` does not exist in `pkpy`.

- [ ] **Step 3: Append `SeatsNoCell` binding to `src/table_no_cell.rs`**

Add to the `use` block:

```rust
use pkcore::casino::table_no_cell::SeatsNoCell as PkSeatsNoCell;
```

Append (after the `SeatNoCell` block, before `register`):

```rust
/// A vector of `SeatNoCell` representing a table's seats.
#[pyclass(from_py_object, name = "SeatsNoCell")]
#[derive(Clone)]
pub struct SeatsNoCell(pub(crate) PkSeatsNoCell);

#[pymethods]
impl SeatsNoCell {
    #[new]
    fn new(seats: Vec<SeatNoCell>) -> Self {
        Self(PkSeatsNoCell::new(seats.into_iter().map(|s| s.0).collect()))
    }

    fn size(&self) -> u8 {
        self.0.size()
    }

    fn get_seat(&self, idx: u8) -> Option<SeatNoCell> {
        self.0.get_seat(idx).cloned().map(SeatNoCell)
    }

    fn is_seat_in_hand(&self, idx: u8) -> bool {
        self.0.is_seat_in_hand(idx)
    }

    fn current_bet(&self) -> usize {
        self.0.current_bet()
    }

    fn to_call(&self, player_idx: u8) -> usize {
        self.0.to_call(player_idx)
    }

    fn total_chip_count(&self) -> usize {
        self.0.total_chip_count()
    }

    fn count_active_in_hand(&self) -> usize {
        self.0.count_active_in_hand()
    }

    fn active_in_hand(&self) -> Vec<u8> {
        self.0.active_in_hand()
    }

    fn are_dealt(&self) -> bool {
        self.0.are_dealt()
    }

    fn are_clear(&self) -> bool {
        self.0.are_clear()
    }

    fn is_betting_complete(&self) -> bool {
        self.0.is_betting_complete()
    }

    fn __repr__(&self) -> String {
        format!("SeatsNoCell(size={})", self.0.size())
    }
}
```

Update `register`:

```rust
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    m.add_class::<SeatNoCell>()?;
    m.add_class::<SeatsNoCell>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `SeatsNoCell` to `python/pkpy/__init__.py`**

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_table_no_cell.py -v`
Expected: 14 passed (8 from prior tasks + 6 new).

- [ ] **Step 6: Suggested commit**

```
git add src/table_no_cell.rs python/pkpy/__init__.py tests/test_table_no_cell.py && git commit -m "feat: bind SeatsNoCell with construction and inspection helpers"
```

---

## Phase 6 — `TableNoCell`

### Task 6: Bind `TableNoCell`

**Files:**
- Modify: `src/table_no_cell.rs`
- Modify: `src/lib.rs` — confirm `ForcedBets` reachability (read-only check; no edit expected)
- Modify: `tests/test_table_no_cell.py`

- [ ] **Step 1: Confirm `ForcedBets` is `pub(crate)`-reachable**

Read `src/lib.rs:2272` — the `pub struct ForcedBets(PkForcedBets);` declaration. Confirm it's `pub` (or `pub(crate)`) so a sibling module can `use crate::ForcedBets`. If the executor finds it is not reachable, fix that line first by changing `pub struct ForcedBets` to `pub(crate) struct ForcedBets` (or leaving it `pub` if already `pub`); no other change.

- [ ] **Step 2: Append failing tests for `TableNoCell`**

Append to `tests/test_table_no_cell.py`:

```python
class TestTableNoCell:
    def test_nlh_from_seats(self):
        from pkpy import ForcedBets, SeatNoCell, SeatsNoCell, TableNoCell
        seats = SeatsNoCell([
            SeatNoCell(PlayerNoCell("Alice", chips=1000)),
            SeatNoCell(PlayerNoCell("Bob", chips=1000)),
        ])
        forced = ForcedBets(50, 100)
        table = TableNoCell.nlh_from_seats(seats, forced)
        assert table.seat_count() == 2

    def test_heads_up_defaults(self):
        from pkpy import ForcedBets, TableNoCell
        forced = ForcedBets(50, 100)
        table = TableNoCell.heads_up(forced)
        assert table.seat_count() == 2
        # Default stacks are (1000, 1000).
        seats = table.seats()
        assert seats.total_chip_count() == 2000

    def test_heads_up_custom_stacks_and_names(self):
        from pkpy import ForcedBets, TableNoCell
        forced = ForcedBets(50, 100)
        table = TableNoCell.heads_up(forced, stacks=(500, 1500), names=("X", "Y"))
        seats = table.seats()
        assert seats.total_chip_count() == 2000
        assert seats.size() == 2

    def test_blind_position_lookups(self):
        from pkpy import ForcedBets, TableNoCell
        table = TableNoCell.heads_up(ForcedBets(50, 100))
        # In heads-up, button is small blind. We don't assert specific seat
        # numbers here — just that the methods return seat indices in range.
        sb = table.determine_small_blind()
        bb = table.determine_big_blind()
        assert sb < table.seat_count()
        assert bb < table.seat_count()

    def test_repr_includes_seat_count(self):
        from pkpy import ForcedBets, TableNoCell
        r = repr(TableNoCell.heads_up(ForcedBets(50, 100)))
        assert "seats=2" in r
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `make build && pytest tests/test_table_no_cell.py::TestTableNoCell -v`
Expected: ImportError — `TableNoCell` does not exist in `pkpy`.

- [ ] **Step 4: Append `TableNoCell` binding to `src/table_no_cell.rs`**

Add to the `use` block:

```rust
use crate::ForcedBets;
use pkcore::casino::table_no_cell::TableNoCell as PkTableNoCell;
```

Append (after the `SeatsNoCell` block, before `register`):

```rust
/// A no-Cell poker table — same semantics as `TableCelled` but without
/// the interior mutability indirection. Wrapped by `PokerSession` for
/// multi-hand session management.
#[pyclass(from_py_object, name = "TableNoCell")]
#[derive(Clone)]
pub struct TableNoCell(pub(crate) PkTableNoCell);

#[pymethods]
impl TableNoCell {
    /// Construct a NLH table from existing seats and forced-bet config.
    /// Faithful pkcore mirror.
    #[staticmethod]
    fn nlh_from_seats(seats: &SeatsNoCell, forced: &ForcedBets) -> Self {
        Self(PkTableNoCell::nlh_from_seats(seats.0.clone(), forced.0))
    }

    /// Convenience: heads-up table with two named, equally-stacked players.
    /// Default stacks are (1000, 1000); default names are ("A", "B").
    #[staticmethod]
    #[pyo3(signature = (forced, stacks=(1000, 1000), names=("A".to_string(), "B".to_string())))]
    fn heads_up(
        forced: &ForcedBets,
        stacks: (usize, usize),
        names: (String, String),
    ) -> Self {
        let seats = PkSeatsNoCell::new(vec![
            PkSeatNoCell::new(PkPlayerNoCell::new_with_chips(names.0, stacks.0)),
            PkSeatNoCell::new(PkPlayerNoCell::new_with_chips(names.1, stacks.1)),
        ]);
        Self(PkTableNoCell::nlh_from_seats(seats, forced.0))
    }

    fn seat_count(&self) -> u8 {
        self.0.seats.size()
    }

    fn seats(&self) -> SeatsNoCell {
        SeatsNoCell(self.0.seats.clone())
    }

    fn determine_small_blind(&self) -> u8 {
        self.0.determine_small_blind()
    }

    fn determine_big_blind(&self) -> u8 {
        self.0.determine_big_blind()
    }

    fn next_occupied_seat_after(&self, start: u8, n: usize) -> u8 {
        self.0.next_occupied_seat_after(start, n)
    }

    fn __repr__(&self) -> String {
        format!("TableNoCell(seats={})", self.0.seats.size())
    }
}
```

Update `register`:

```rust
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    m.add_class::<SeatNoCell>()?;
    m.add_class::<SeatsNoCell>()?;
    m.add_class::<TableNoCell>()?;
    Ok(())
}
```

- [ ] **Step 5: Add `TableNoCell` to `python/pkpy/__init__.py`**

- [ ] **Step 6: Run tests to verify they pass**

Run: `make build && pytest tests/test_table_no_cell.py -v`
Expected: 19 passed (14 from prior tasks + 5 new).

- [ ] **Step 7: Suggested commit**

```
git add src/table_no_cell.rs python/pkpy/__init__.py tests/test_table_no_cell.py && git commit -m "feat: bind TableNoCell with nlh_from_seats and heads_up factories"
```

---

## Phase 7 — `PokerSession` (incl. 0.0.53 NEW methods)

### Task 7: Bind `PokerSession`

**Files:**
- Modify: `src/session.rs`
- Modify: `tests/test_session.py`

- [ ] **Step 1: Append failing tests for `PokerSession`**

Append to `tests/test_session.py`:

```python
class TestPokerSession:
    def _heads_up(self, sb=50, bb=100, stacks=(1000, 1000)):
        from pkpy import ForcedBets, PokerSession
        return PokerSession.heads_up(ForcedBets(sb, bb), stacks=stacks)

    def test_construct_from_table(self):
        from pkpy import ForcedBets, PokerSession, TableNoCell
        table = TableNoCell.heads_up(ForcedBets(50, 100))
        session = PokerSession(table)
        assert session.hand_number == 0
        assert session.shuffled_deck_str is None

    def test_heads_up_factory(self):
        session = self._heads_up()
        assert session.hand_number == 0
        assert not session.is_hand_in_progress()

    def test_start_hand_increments_hand_number(self):
        session = self._heads_up()
        session.start_hand()
        assert session.hand_number == 1
        assert session.is_hand_in_progress()

    def test_next_step_after_start_is_player_to_act(self):
        session = self._heads_up()
        session.start_hand()
        step = session.next_step()
        assert step.kind() == "PlayerToAct"
        assert step.seat() is not None

    def test_count_funded(self):
        session = self._heads_up()
        assert session.count_funded() == 2

    def test_apply_action_fold_ends_hand(self):
        from pkpy import PlayerAction
        session = self._heads_up()
        session.start_hand()
        actor = session.next_actor()
        assert actor is not None
        session.apply_action(actor, PlayerAction.fold())
        winnings = session.end_hand()
        assert not winnings.is_empty()
        assert len(winnings) >= 1

    # ── 0.0.53 regression ports ──────────────────────────────────────────
    # Direct translations of pkcore unit tests at casino/session.rs:970-1010.

    def test_set_blinds_between_hands_applies_immediately(self):
        from pkpy import ForcedBets
        session = self._heads_up()
        session.set_blinds(ForcedBets(100, 200))
        # Before any hand starts, the snapshot reflects the *new* blinds
        # because PokerSession::new captures the table's current forced
        # bets, and set_blinds (with no hand in progress) overwrites them.
        # We check the snapshot via forced_at_hand_start AFTER start_hand,
        # which is the documented stable surface.
        session.start_hand()
        assert session.forced_at_hand_start().small_blind == 100
        assert session.forced_at_hand_start().big_blind == 200

    def test_set_blinds_during_hand_defers_to_next_hand(self):
        from pkpy import ForcedBets, PlayerAction
        session = self._heads_up()
        session.start_hand()
        # Mid-hand: bump blinds.
        session.set_blinds(ForcedBets(100, 200))
        # forced_at_hand_start still reflects what was posted this hand.
        assert session.forced_at_hand_start().small_blind == 50
        assert session.forced_at_hand_start().big_blind == 100

    def test_deferred_blinds_take_effect_on_next_start_hand(self):
        from pkpy import ForcedBets, PlayerAction
        session = self._heads_up()
        session.start_hand()
        session.set_blinds(ForcedBets(100, 200))
        # Finish the hand by folding the next actor.
        actor = session.next_actor()
        session.apply_action(actor, PlayerAction.fold())
        session.end_hand()
        # Next hand picks up the deferred blinds.
        session.start_hand()
        assert session.forced_at_hand_start().small_blind == 100
        assert session.forced_at_hand_start().big_blind == 200

    def test_forced_at_hand_start_stable_during_hand(self):
        from pkpy import ForcedBets
        session = self._heads_up()
        session.start_hand()
        snap1 = session.forced_at_hand_start()
        session.set_blinds(ForcedBets(400, 800))
        snap2 = session.forced_at_hand_start()
        assert snap1.small_blind == snap2.small_blind == 50
        assert snap1.big_blind == snap2.big_blind == 100
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `make build && pytest tests/test_session.py::TestPokerSession -v`
Expected: ImportError — `PokerSession` does not exist in `pkpy`.

- [ ] **Step 3: Append `PokerSession` binding to `src/session.rs`**

Add to the `use` block at the top:

```rust
use crate::table_no_cell::TableNoCell;
use crate::{ForcedBets, Winnings, to_py_err};
use pkcore::casino::session::PokerSession as PkPokerSession;
```

Append (after the `SessionStep` block, before `register`):

```rust
/// A multi-hand poker session wrapping a `TableNoCell`.
///
/// Drive a hand: `start_hand()` → loop on `next_step()`, calling
/// `apply_action(seat, PlayerAction.X)` for each `PlayerToAct` —
/// `end_hand()` to settle. Use `set_blinds` (deferred mid-hand) to
/// adjust forced bets between hands.
#[pyclass(name = "PokerSession")]
pub struct PokerSession(pub(crate) PkPokerSession);

#[pymethods]
impl PokerSession {
    #[new]
    fn new(table: &TableNoCell) -> Self {
        Self(PkPokerSession::new(table.0.clone()))
    }

    /// Convenience: heads-up session in one call. Mirrors
    /// `TableNoCell.heads_up`'s defaults.
    #[staticmethod]
    #[pyo3(signature = (forced, stacks=(1000, 1000), names=("A".to_string(), "B".to_string())))]
    fn heads_up(
        forced: &ForcedBets,
        stacks: (usize, usize),
        names: (String, String),
    ) -> Self {
        let table = TableNoCell::heads_up(forced, stacks, names);
        Self(PkPokerSession::new(table.0))
    }

    // ── 0.0.53 NEW ───────────────────────────────────────────────────────

    fn set_blinds(&mut self, forced: &ForcedBets) {
        self.0.set_blinds(forced.0);
    }

    fn forced_at_hand_start(&self) -> ForcedBets {
        ForcedBets(self.0.forced_at_hand_start())
    }

    // ── Hand lifecycle ──────────────────────────────────────────────────

    fn start_hand(&mut self) -> PyResult<()> {
        self.0.start_hand().map_err(to_py_err)
    }

    fn end_hand(&mut self) -> PyResult<Winnings> {
        self.0.end_hand().map(Winnings).map_err(to_py_err)
    }

    fn is_hand_in_progress(&self) -> bool {
        self.0.is_hand_in_progress()
    }

    fn is_hand_complete(&self) -> bool {
        self.0.is_hand_complete()
    }

    fn next_actor(&mut self) -> Option<u8> {
        self.0.next_actor()
    }

    fn next_step(&mut self) -> SessionStep {
        SessionStep(self.0.next_step())
    }

    fn apply_action(&mut self, seat: u8, action: &PlayerAction) -> PyResult<()> {
        self.0.apply_action(seat, action.0).map_err(to_py_err)
    }

    // ── Session-wide ────────────────────────────────────────────────────

    fn count_funded(&self) -> usize {
        self.0.count_funded()
    }

    fn eliminate_busted(&mut self) -> Vec<u8> {
        self.0.eliminate_busted()
    }

    // ── Field accessors ─────────────────────────────────────────────────

    #[getter]
    fn hand_number(&self) -> u64 {
        self.0.hand_number
    }

    #[getter]
    fn shuffled_deck_str(&self) -> Option<String> {
        self.0.shuffled_deck_str.clone()
    }
}
```

Update `register`:

```rust
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerAction>()?;
    m.add_class::<SessionStep>()?;
    m.add_class::<PokerSession>()?;
    Ok(())
}
```

- [ ] **Step 4: Add `PokerSession` to `python/pkpy/__init__.py`**

- [ ] **Step 5: Run tests to verify they pass**

Run: `make build && pytest tests/test_session.py -v`
Expected: 19 passed (9 from Tasks 1-2 + 10 new).

- [ ] **Step 6: Suggested commit**

```
git add src/session.rs python/pkpy/__init__.py tests/test_session.py && git commit -m "feat: bind PokerSession (incl. 0.0.53 set_blinds and forced_at_hand_start)"
```

---

## Phase 8 — Wrap-up gate

### Task 8: Full `make ayce` and final review

**Files:**
- (Verification only — read-only checks)

- [ ] **Step 1: Run the full ayce gate**

Run: `make ayce`
Expected:
- `cargo fmt` clean.
- `maturin develop` succeeds with no warnings introduced by the new bindings.
- `pytest` passes all tests including the 19 new ones (187 prior + 19 = 206 total minimum, modulo any existing tests that count differently).
- `demo.py` still runs unchanged.

- [ ] **Step 2: Verify Python re-exports**

Run from a Python REPL inside the venv:

```python
from pkpy import (
    PlayerAction, SessionStep, PokerSession,
    PlayerNoCell, SeatNoCell, SeatsNoCell, TableNoCell,
    ForcedBets,
)
print(PlayerAction.bet(200))
print(PokerSession.heads_up(ForcedBets(50, 100)))
```

Expected: no ImportError; the prints show the `__repr__` strings.

- [ ] **Step 3: Run clippy on the new modules**

Run: `make clippy`
Expected: no new warnings introduced by `src/session.rs` or `src/table_no_cell.rs`.

- [ ] **Step 4: No commit (verification gate)**

If everything passes, the implementation is done. If any step fails, the cause is in one of Tasks 1-7; fix it there and re-run `make ayce`.

---

## Notes for the implementing agent

- **`PkPokerSession::new` takes `TableNoCell` by value** — the binding clones the underlying `PkTableNoCell` to satisfy the Python ownership model. This is fine: `TableNoCell` is `Clone`.
- **`PkPokerSession` is not `Clone`** — that's why `PokerSession` uses plain `#[pyclass]` (without `from_py_object`).
- **`ForcedBets` is `Copy`** — the existing pkpy binding at `src/lib.rs:2272` already takes advantage of this; pass `forced.0` directly without `.clone()`.
- **`PkPlayerAction`, `PkSessionStep` are `Copy`** — same.
- **`PkPlayerNoCell::new(handle)` and `PkPlayerNoCell::new_with_chips(handle, chips)` are observationally equivalent at `chips=0`** — pkcore's `new` produces a 0-chip player. The `if chips == 0` branch in the binding's `#[new]` exists only to keep both pkcore constructors reachable from the binding code in case they diverge later; it does not affect runtime behavior at chips=0.
- **Re-imports at the top of each module file** — follow the existing pattern (`use pkcore::X as PkX;`). This is the convention in `src/lib.rs:30`.
- **`from_py_object` on `#[pyclass]`** — preserve it for everything except `PokerSession`. It's the existing pkpy convention.
- **Testing strategy** — TDD throughout. If a test fails for a reason that's not "the type doesn't exist yet" during the failing-test step, stop and investigate before implementing.
- **Verification** — run `make ayce` after each commit. Tasks are designed so that intermediate commits leave the build green even though only some types are exposed.
- **If `make build` produces deprecation warnings from pkcore 0.0.53**, address them inline if they touch our wrappers. Do not silence them globally.
