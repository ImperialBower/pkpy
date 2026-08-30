---
okf_version: '0.1'
---

# pkcore.py

* [Project](project.md) - PyO3 bindings that expose the pkcore Rust poker library to Python as the pkcore package.

# Areas

* [Architecture](architecture/) - how the binding layer is built and laid out.
* [API surface](api/) - what the pkcore package exposes to Python, by area.
* [Operations](operations/) - building, testing, releasing.
* [Decisions](decisions/) - standing rules and the reasoning behind them.

# Start here

* [The binding layer](architecture/binding-layer.md) - every Python class is a newtype over an upstream pkcore type.
* [Build and test](operations/build-and-test.md) - use the Makefile; plain `cargo build` cannot link.
* [Version lockstep](decisions/version-lockstep.md) - this package's version always equals the pkcore version it wraps.
