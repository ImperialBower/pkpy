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
