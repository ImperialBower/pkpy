# pkpy project instructions

## Version rule

`pkpy`'s version in `Cargo.toml` must always match the `pkcore` dependency
version. When `pkcore` is bumped, bump `pkpy`'s own `version` field to the
same value in the same change.
