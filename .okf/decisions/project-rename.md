---
type: Decision
title: Rename from pkpy to pkcore.py
description: The package was renamed and relicensed; the old pkpython distribution is dead.
tags: [naming, packaging, license, breaking]
timestamp: 2026-08-30T00:00:00Z
---

# What changed

| Thing | Was | Is |
|---|---|---|
| PyPI distribution | `pkpython` | `pkcore.py` |
| Python import | `pkpy` | `pkcore` |
| Cargo crate | `pkpy` | `pkcore-py` |
| Extension module | `pkpy._pkpy` | `pkcore._pkcore` |
| Repository | `ImperialBower/pkpy` | `ImperialBower/pkcore.py` |
| License | `GPL-3.0-or-later` | `MIT OR Apache-2.0` |

```python
from pkcore import Card   # was: from pkpy import Card
```

`pkpython` on PyPI receives no further releases. The old GitHub URL redirects.

# Why the crate name differs

Cargo package names cannot contain a dot, so the crate is `pkcore-py` while the
distribution is `pkcore.py`. This mismatch is permanent and intentional. The
crate is not published to crates.io, so it never has to be typed by a consumer.

# Why the relicense

To match upstream `pkcore`, which is `MIT OR Apache-2.0`. A GPL binding layer
over a permissive core was a needless constraint on consumers. `LICENSE` was
replaced by `LICENSE-MIT` and `LICENSE-APACHE`, with `Cargo.toml`,
`pyproject.toml` classifiers and the README badges updated to match.

# Lingering effects

- Any pre-existing `.venv` breaks, because venv shebangs hardcode the old
  absolute path. See [build and test](/operations/build-and-test.md).
- `README.md` still carries a migration note for `pkpython` users. Keep it.

# Related

- [Project](/project.md)
