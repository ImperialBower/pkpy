# Technology Stack

## PyO3

[PyO3](https://pyo3.rs) is the standard Rust library for writing Python extension modules. It provides:

- Rust macros (`#[pyclass]`, `#[pymethods]`, `#[pymodule]`) that generate the C-level Python type structures CPython expects
- Automatic memory management bridging Rust's ownership model with Python's garbage collector
- Type conversion between Rust primitives and Python objects (`&str` ↔ `str`, `Vec<T>` ↔ `list`, `Option<T>` ↔ `None | T`, etc.)
- Safe error propagation — Rust `Result::Err` values become Python exceptions

When you call `Card.parse("As")` in Python, PyO3 converts the Python string to a `&str`, passes it into the Rust `FromStr` implementation, and wraps the returned `Card` in a Python object. All of this happens in-process with no serialization.

The extension module is compiled to a `.so` / `.dylib` / `.pyd` shared library (platform-dependent) that CPython imports like any other C extension.

## Maturin

[Maturin](https://maturin.rs) is the build tool for PyO3 projects. It handles:

- Detecting the active Python interpreter and its include paths
- Invoking `cargo build` with the correct flags to link against CPython's shared library
- Packaging the compiled `.so` into a standard Python wheel (`.whl`)
- `maturin develop` for editable installs during development (equivalent to `pip install -e .`)

Maturin replaces the older `setuptools-rust` approach and is the recommended tool for new PyO3 projects.

## Design Notes

**Why not ctypes or cffi?** Those require a C-compatible ABI layer and manual memory management. PyO3 operates at the Python C API level and handles memory safety through Rust's ownership model. It also provides much richer type integration (Python exceptions, iterators, `__str__`, `__eq__`, etc.) with very little boilerplate.

**Why not pydantic-style dataclasses?** pkcore types carry invariants that are enforced by Rust's type system at construction time (e.g., a `Card` is always a valid CKC `u32`). Reimplementing those in Python would either duplicate the logic or lose the guarantees. Wrapping the Rust types directly means the invariants are never violated.

**String parsing as the primary constructor:** pkcore's Rust API uses `FromStr` extensively, and that maps naturally to static `parse()` class methods in Python. This keeps the Python API idiomatic while reusing the battle-tested Rust parsing logic.

**Player indices are 1-based:** This matches pkcore's convention in `Outs` and `CaseEvals`, where player 1 is the first hand passed to `HoleCards`.
