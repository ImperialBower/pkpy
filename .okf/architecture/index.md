# Architecture

* [The binding layer](binding-layer.md) - how every Python class is a thin Rust newtype around an upstream pkcore type.
* [Rust module map](module-map.md) - what each file under `src/` contains, including the three registered-but-empty stub modules.
* [The Python package surface](python-surface.md) - how `python/pkcore/__init__.py` re-exports the compiled extension, and why the list must be maintained by hand.
