# Release Checklist

`wavefunk-ui` publishes from `main` with semantic versions in `Cargo.toml`.

## Versioning

- Patch releases fix component rendering, asset serving, docs, and packaging defects without changing public constructors or output contracts.
- Minor releases add components, builders, optional feature adapters, or embedded assets in a backward-compatible way.
- Major releases can remove components, rename classes, change markup contracts, or change required framework versions.

## Preflight

Run every command through direnv:

```bash
direnv exec . just release-check
```

For browser release smoke:

```bash
direnv exec . just gallery
```

Open `http://127.0.0.1:3000` in a browser. Check the page loads, embedded CSS/fonts/scripts return 200, toast/echo htmx interactions work, and light/dark plus dense/default variants render without console errors.

## Woodpecker

`.woodpecker/ci.yml` runs on pushes to `main`, pull requests, and manual triggers. It uses `git.wavefunk.io/wavefunk/ci-rust:nightly-2026-01-05` and runs:

```bash
cargo fmt --check
cargo test --all-features --locked
cargo clippy --all-features --all-targets --locked -- -D warnings
cargo package --allow-dirty --locked
cargo publish --dry-run --locked --allow-dirty
```

`.woodpecker/release.yml` runs on `v*` tags. It verifies that the tag matches `Cargo.toml` and then publishes with `cargo publish --locked` using the `cargo_registry_token` Woodpecker secret.

## Publish

1. Update `CHANGELOG.md` and replace `Unreleased` with the release date.
2. Confirm `Cargo.toml` has the intended version and package include list.
3. Commit the release change.
4. Tag the commit as `vX.Y.Z`, matching the manifest version.
5. Push the branch plus tag and let `.woodpecker/release.yml` publish the crate.
