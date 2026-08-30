---
type: Architecture
title: The binding layer — newtype wrappers over pkcore
description: How every Python class in this package is a thin Rust newtype around an upstream pkcore type.
tags: [pyo3, rust, architecture, newtype]
timestamp: 2026-08-30T00:00:00Z
---

# The pattern

Every exposed class follows one shape: a tuple struct that holds exactly one
upstream value, with the upstream type imported under a `Pk` prefix.

```rust
use pkcore::card::Card as PkCard;

#[pyclass(name = "Card")]
#[derive(Clone, Copy, ...)]
pub struct Card(PkCard);

#[pymethods]
impl Card {
    // ...
}
```

This is deliberate and consistent:

- **`Pk` prefix on imports.** The Python-facing type keeps the plain name; the
  upstream type is aliased. So `Card` is the binding and `PkCard` is pkcore's.
  A reader can tell in one glance which side of the boundary a value is on.
- **Newtype, not re-implementation.** No poker rule is restated in this repo.
  Invariants stay enforced by pkcore's constructors, so a `Card` handed to
  Python is always valid.
- **`parse()` as the primary constructor.** pkcore leans on Rust's `FromStr`;
  that maps to static `parse()` methods, which reads idiomatically in Python and
  reuses the upstream parser.

# Boundary rules

| Rust | Python |
|---|---|
| `Result::Err` | raised exception, via local `to_py_err` / `dealer_err` helpers |
| `Option<T>` | `T` or `None` |
| `impl Iterator` | a dedicated `#[pyclass]` iterator (e.g. the private `CardsIterator`) |
| player index | **1-based**, matching pkcore's `Outs` and `CaseEvals` convention |

Note the last row. Player 1 is the first hand passed to `HoleCards`. This is a
frequent source of off-by-one confusion for Python readers who expect 0-based
indexing.

# Scale

| Measure | Count |
|---|---|
| `#[pyclass]` structs | 68 |
| Classes registered on the module | 67 |
| Module-level functions | 4 |

The one unregistered class is `CardsIterator`, a private iterator handle that is
only ever produced by `Cards.__iter__` and never constructed from Python.

# Related

- [Module map](/architecture/module-map.md) — which file holds which classes.
- [Python surface](/architecture/python-surface.md) — what `import pkcore` gives you.
- [Project](/project.md)

# Citations

[1] `docs/STACK.md` — "Design Notes" section, in this repository
[2] [PyO3 class documentation](https://pyo3.rs/latest/class.html)
