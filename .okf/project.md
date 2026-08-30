---
type: Project
title: pkcore.py
description: PyO3 bindings that expose the pkcore Rust poker library to Python as the pkcore package.
resource: https://github.com/ImperialBower/pkcore.py
tags: [poker, pyo3, rust, python, bindings]
timestamp: 2026-08-30T00:00:00Z
---

# What this is

`pkcore.py` is a **binding layer**, not a poker engine. Every rule, evaluator and
solver lives upstream in the [`pkcore`](https://crates.io/crates/pkcore) Rust
crate. This repository wraps that crate's public types in PyO3 classes and ships
them as a Python extension module.

Because of that, the project holds almost no poker logic of its own. Its real
content is the *translation* decisions: how Rust names, errors and ownership are
presented to Python. See [the binding layer](/architecture/binding-layer.md).

# Names

Three different names refer to the same thing, and they do not match. This trips
people up constantly.

| Where | Name |
|---|---|
| PyPI distribution | `pkcore.py` |
| Python import | `pkcore` |
| Cargo crate (`Cargo.toml`) | `pkcore-py` — Cargo names cannot contain a dot |
| Compiled extension module | `pkcore._pkcore` (cdylib, `[lib] name = "_pkcore"`) |
| GitHub repository | `ImperialBower/pkcore.py` |

The crate is **not published to crates.io**. Only the wheel is published, to
PyPI. See [release](/operations/release.md) and
[the project rename](/decisions/project-rename.md).

# Version

`pkcore.py`'s own version always equals the `pkcore` version it wraps. This is a
hard project rule — see [version lockstep](/decisions/version-lockstep.md).
Current: **0.11.0**.

# Where things live

| Path | Contents |
|---|---|
| `src/` | Rust binding code — see [module map](/architecture/module-map.md) |
| `python/pkcore/__init__.py` | Re-export surface and package docstring |
| `tests/` | pytest suites (228 tests) |
| `examples/`, `demo.py` | Runnable demonstrations |
| `docs/STACK.md` | Why PyO3 + maturin, and the design notes behind them |
| `Makefile` | Every developer entry point — see [build and test](/operations/build-and-test.md) |

# Citations

[1] [pkcore on crates.io](https://crates.io/crates/pkcore)
[2] [PyO3 user guide](https://pyo3.rs)
[3] `docs/STACK.md` in this repository
