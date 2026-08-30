# pkcore.py project instructions

## Version rule

`pkcore.py`'s version in `Cargo.toml` must always match the `pkcore` dependency
version. When `pkcore` is bumped, bump `pkcore.py`'s own `version` field to the
same value in the same change.

## Changelog rule

The newest heading in `CHANGELOG.md` must be the version being bumped to — never
`## [Unreleased]`. A version bump writes its heading as
`## [<version>] - <YYYY-MM-DD>` in the same change that edits `Cargo.toml`.

Do not open an empty `## [Unreleased]` section above it. Work that is not yet
bumped belongs under the current version's heading; the next bump renames that
heading. This keeps `make release` runnable at any commit, since it refuses to
tag unless `CHANGELOG.md` already contains `## [<version>]`.
