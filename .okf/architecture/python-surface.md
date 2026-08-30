---
type: Architecture
title: The Python package surface
description: How python/pkcore/__init__.py re-exports the compiled extension, and why the list must be maintained by hand.
tags: [python, packaging, api]
timestamp: 2026-08-30T00:00:00Z
---

# Two layers

`maturin` builds a **mixed** project (`python-source = "python"` in
`pyproject.toml`). That means two things ship in the wheel:

1. The compiled extension, `pkcore._pkcore` (from `[lib] name = "_pkcore"`).
2. The pure-Python package `python/pkcore/`, whose `__init__.py` re-exports
   names out of that extension.

So `from pkcore import Card` resolves through `__init__.py`, which does
`from pkcore._pkcore import (...)`.

# The hand-maintained list

`__init__.py` names every re-exported symbol explicitly. There is **no**
`import *`. That is good for tooling and discoverability, but it means the list
is a second place that must change whenever a class is added.

> **Failure mode:** add a `#[pyclass]` and an `m.add_class::<Thing>()` in Rust,
> forget `__init__.py`, and `from pkcore import Thing` raises `ImportError`
> while `from pkcore._pkcore import Thing` works fine. The tests will not
> necessarily catch it.

Adding a class is therefore a **three-part** edit:

1. The `#[pyclass]` struct and its `#[pymethods]`.
2. `m.add_class::<Thing>()?` in the owning module's registration.
3. The name in `python/pkcore/__init__.py`.

# Package docstring

`__init__.py` also carries the package docstring, which contains the canonical
short example (parse cards, build a `Game`, count `Outs`). It is what
`help(pkcore)` prints, so keep it runnable.

# Related

- [Module map](/architecture/module-map.md) — where step 2 happens.
- [API surface index](/api/index.md)
