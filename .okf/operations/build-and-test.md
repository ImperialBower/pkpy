---
type: Playbook
title: Build and test
description: Every developer entry point runs through the Makefile; the venv is the thing that breaks.
tags: [build, test, maturin, makefile]
timestamp: 2026-08-30T00:00:00Z
---

# Do this

```bash
make setup    # create .venv, install maturin + pytest
make build    # maturin develop — compile the extension into .venv
make test     # build, then pytest (228 tests)
make ayce     # fmt + build + test + demo — the full pass
```

`make ayce` ("all you can eat") is the default target and the one to run before
proposing a change.

# Do not use plain `cargo build`

It compiles the Rust fine and then **fails at the link step**:

```
ld: symbol(s) not found for architecture arm64
Undefined symbols: _PyUnicode_FromStringAndSize, _Py_InitializeEx, ...
```

This is expected, not a regression. The crate is built with
`pyo3/extension-module`, which tells PyO3 *not* to link libpython — the symbols
are resolved by the host CPython at import time. Only `maturin` knows how to
finish the job.

Use `cargo clippy` (`make clippy`) when you want a Rust-only check; it stops
before linking. Note that it currently reports ~161 pedantic warnings, all
pre-existing.

# The venv breaks when the directory moves

A Python venv hardcodes absolute paths in its script shebangs. After the
`pkpy` → `pkcore.py` [rename](/decisions/project-rename.md), the old `.venv`
produced:

```
.venv/bin/pip: bad interpreter:
  /Users/.../ImperialBower/pkpy/.venv/bin/python3: no such file or directory
```

**Fix:** delete and rebuild. It costs seconds.

```bash
rm -rf .venv && make setup
```

Do not use `make clean` for this — it also runs `cargo clean` and throws away the
whole `target/` directory, turning a 10-second fix into a multi-minute rebuild.

# Other targets

| Target | Effect |
|---|---|
| `make demo` | Runs `demo.py`. |
| `make calc` | `examples/calc.py` on "THE HAND". |
| `make gto` | `examples/gto.py`, KK against a range. |
| `make the-hand` | `examples/the_hand.py`. |
| `make fmt` | `cargo fmt`. |
| `make version` | Prints the version parsed out of `Cargo.toml`. |

# Related

- [Release](/operations/release.md)
- [Project](/project.md)
