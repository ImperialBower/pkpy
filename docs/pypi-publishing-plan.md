# PyPI Publishing Plan for pkpy

pkpy is a maturin-based PyO3 extension that ships compiled Rust code as a Python package. Publishing it to PyPI requires building platform-specific binary wheels for every target platform, then uploading them alongside an `sdist`. This is more involved than a pure-Python package.

---

## 1. Fix Metadata Inconsistencies

### Version mismatch
`Cargo.toml` is at `0.0.32` and `pyproject.toml` is at `0.1.0`. These must agree. Maturin can read the version from `Cargo.toml` automatically — either remove the `version` key from `[project]` in `pyproject.toml` (maturin will inherit it) or keep them in sync manually.

**Action:** Remove `version = "0.1.0"` from `pyproject.toml` and let maturin derive it from `Cargo.toml`.

### Complete `pyproject.toml` metadata
PyPI renders `[project]` metadata on the package page. Add the missing fields:

```toml
[project]
# version — removed; maturin reads from Cargo.toml
authors = [{ name = "Imperial Bower", email = "you@example.com" }]
keywords = ["poker", "cards", "hand-evaluation", "game", "rust"]
readme = "README.md"

[project.urls]
Homepage    = "https://github.com/ImperialBower/pkpy"
Repository  = "https://github.com/ImperialBower/pkpy"
"Bug Tracker" = "https://github.com/ImperialBower/pkpy/issues"
```

### Expand classifiers
The existing classifiers only mention the implementation. Add development status and Python version classifiers:

```toml
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)",
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Programming Language :: Python :: 3.13",
    "Topic :: Games/Entertainment",
    "Topic :: Scientific/Engineering",
]
```

---

## 2. Build Multi-Platform Wheels

Binary packages must ship pre-built wheels for every platform you want to support, or users will need a Rust toolchain to install from source. The standard targets are:

| Platform | Target | Notes |
|---|---|---|
| Linux (manylinux) | `x86_64`, `aarch64` | Use maturin's `manylinux` Docker images |
| macOS | `x86_64`, `arm64` (M-series) | Use `universal2` wheel or separate wheels |
| Windows | `x86_64` | Most common; `i686` is optional |

### Add a release workflow: `.github/workflows/release.yml`

Use `PyO3/maturin-action` — the official GitHub Action for building maturin packages across platforms. A typical structure:

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  build:
    name: Build wheels
    strategy:
      matrix:
        include:
          # Linux x86_64
          - os: ubuntu-latest
            target: x86_64
          # Linux aarch64
          - os: ubuntu-latest
            target: aarch64
          # macOS x86_64
          - os: macos-13
            target: x86_64
          # macOS arm64
          - os: macos-latest
            target: aarch64
          # Windows x86_64
          - os: windows-latest
            target: x86_64
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.13"
      - uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist
          manylinux: auto   # only applies on Linux
          sccache: true     # optional: Rust build cache
      - uses: actions/upload-artifact@v4
        with:
          name: wheels-${{ matrix.os }}-${{ matrix.target }}
          path: dist

  sdist:
    name: Build sdist
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: PyO3/maturin-action@v1
        with:
          command: sdist
          args: --out dist
      - uses: actions/upload-artifact@v4
        with:
          name: sdist
          path: dist

  publish:
    name: Publish to PyPI
    runs-on: ubuntu-latest
    needs: [build, sdist]
    permissions:
      id-token: write   # required for OIDC trusted publishing
    steps:
      - uses: actions/download-artifact@v4
        with:
          pattern: wheels-*
          merge-multiple: true
          path: dist
      - uses: actions/download-artifact@v4
        with:
          name: sdist
          path: dist
      - uses: pypa/gh-action-pypi-publish@release/v1
```

---

## 3. Set Up PyPI Authentication

### Option A: Trusted Publisher (recommended, no secrets needed)
PyPI supports OpenID Connect (OIDC) publishing directly from GitHub Actions — no API token or secret required. Configure it at `pypi.org/manage/account/publishing/` by registering:

- **Owner:** `ImperialBower`
- **Repo:** `pkpy`
- **Workflow:** `release.yml`
- **Environment:** (leave blank or set to `pypi`)

The `pypa/gh-action-pypi-publish` action handles the OIDC exchange automatically when `permissions: id-token: write` is set.

### Option B: API token (fallback)
Create a scoped token on PyPI and store it as a GitHub Actions secret named `PYPI_API_TOKEN`. Add to the publish step:

```yaml
      - uses: pypa/gh-action-pypi-publish@release/v1
        with:
          password: ${{ secrets.PYPI_API_TOKEN }}
```

---

## 4. Validate on TestPyPI First

Before publishing to the real index:

1. Register the same trusted publisher on `test.pypi.org`.
2. In the publish step, set `repository-url: https://test.pypi.org/legacy/`.
3. Install and smoke-test the package:
   ```bash
   pip install --index-url https://test.pypi.org/simple/ pkpy
   python -c "from pkpy import Card; print(Card.parse('As'))"
   ```
4. Confirm that wheels resolve correctly on each platform before pushing to production PyPI.

---

## 5. Release Process (Tag-Driven)

1. Bump version in `Cargo.toml` (and `Cargo.lock` will update on next build).
2. Commit: `git commit -m "Bump to vX.Y.Z"`.
3. Tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
4. The release workflow triggers, builds all wheels, and publishes to PyPI.

For pre-releases, use `v0.1.0a1` / `v0.1.0b1` style tags. PyPI will mark them as pre-release automatically.

---

## 6. Source Distribution (sdist) Considerations

The sdist contains the Rust source and `Cargo.toml`, so a user with a Rust toolchain can `pip install pkpy` from source as a fallback. However:

- The sdist must include `Cargo.lock` for reproducible builds — check that `.gitignore` does not exclude it.
- If `Cargo.lock` is not vendored, `pip install` from sdist will download crates from crates.io, which requires network access at install time. This is usually acceptable.

---

## 7. README Rendering on PyPI

PyPI renders `README.md` as the package description. Confirm:

- `readme = "README.md"` is in `pyproject.toml` (see step 1).
- All image URLs in `README.md` are absolute GitHub URLs (relative paths do not render on PyPI).
- The CI badge URL is already absolute — no change needed there.

---

## 8. Pre-publish Checklist

- [ ] Version in `Cargo.toml` matches the intended release
- [ ] `pyproject.toml` has `readme`, `authors`, `keywords`, and `[project.urls]`
- [ ] `pyproject.toml` does not declare a conflicting `version`
- [ ] Release workflow file exists at `.github/workflows/release.yml`
- [ ] Trusted publisher registered on PyPI (or `PYPI_API_TOKEN` secret set)
- [ ] TestPyPI smoke test passed for all target platforms
- [ ] All existing tests pass on CI
- [ ] `CHANGELOG` or release notes updated (optional but recommended)
- [ ] Package name `pkpy` is available on PyPI (verify at `pypi.org/project/pkpy`)
