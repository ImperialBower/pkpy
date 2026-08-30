---
type: Architecture
title: Rust module map
description: What each file under src/ contains, including the three registered-but-empty stub modules.
tags: [rust, architecture, layout]
timestamp: 2026-08-30T00:00:00Z
---

# Files

| File | Lines | Contents |
|---|---:|---|
| `src/lib.rs` | ~4,090 | The bulk. 60 classes plus the 4 module functions, and the `#[pymodule] fn _pkcore` entry point. |
| `src/session.rs` | 236 | `PlayerAction`, `SessionStep`, `PokerSession` — the guided hand lifecycle. |
| `src/table_no_cell.rs` | 231 | `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell`, `TableNoCell` — the cell-free table types. |
| `src/hand_history.rs` | 7 | **Stub.** |
| `src/stats.rs` | 7 | **Stub.** |
| `src/bot.rs` | 7 | **Stub.** |

# The submodule pattern

`lib.rs` registers its own classes inline, then delegates:

```rust
hand_history::register(m)?;
stats::register(m)?;
bot::register(m)?;
session::register(m)?;
table_no_cell::register(m)?;
```

Each submodule exposes `pub(crate) fn register(m: &Bound<'_, PyModule>)`. This
keeps `lib.rs` from growing a second thousand lines of registration and gives
each upstream area a clear home.

# The three stubs

`hand_history.rs`, `stats.rs` and `bot.rs` are **placeholders, not oversights**.
Each carries a module docstring naming the upstream area it will wrap
(`pkcore`'s `hand_history`, `analysis::player_stats`, and `bot` modules
respectively) and a `register` that adds nothing:

```rust
//! Bindings for pkcore's analysis::player_stats module.

pub(crate) fn register(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
```

They are wired into the module tree so that filling one in requires no plumbing
change — only `#[pyclass]` structs and `add_class` lines. Treat an empty
`register` as a claimed slot.

# Related

- [Binding layer](/architecture/binding-layer.md) — the wrapper pattern used inside these files.
- [Python surface](/architecture/python-surface.md)
