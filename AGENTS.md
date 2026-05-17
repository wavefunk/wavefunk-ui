# wavefunk-ui Agent Notes

## Project Shape

This is the public `wavefunk-ui` crate. It provides Askama and htmx UI primitives plus embedded Wave Funk CSS, fonts, and JavaScript for self-contained Rust binaries.

Keep the committed package publishable from day one. `Cargo.toml` should depend on published crate versions, and consumer repos should commit `wavefunk-ui = "x.y.z"` instead of local path dependencies.

## Local Consumer Iteration

Wave Funk consumers can iterate against this repo with a gitignored `.cargo/config.toml` in the consumer project:

```toml
paths = [
    "../ui",
]
```

Do not commit local path overrides. Release and CI runs should resolve `wavefunk-ui` from crates.io.

## Asset Source Of Truth

This repo is canonical for the embedded Wave Funk runtime assets. Keep CSS, fonts, and JavaScript changes in `static/wavefunk/` here and publish them through this crate.

Do not sync CSS or fonts from `../design`, and do not add recipes or scripts that copy design-repo assets over this crate's checked-in assets. If design work needs to change shared runtime styling, make the CSS change in this repo and let consumers pick it up by updating `wavefunk-ui`.

## Development Environment

Use the Nix flake and direnv setup in this repo. The devshell is intentionally Rust-only:

- No Node.js
- No Tailwind
- No Playwright
- No Dolt

Run every command for this repo through direnv, including `git`, `br`, `just`, and `cargo`:

```bash
direnv exec . git status --short --branch
direnv exec . br ready --json
direnv exec . just test
```

Do not run bare repo commands outside `direnv exec .`; the direnv environment supplies the expected toolchain and Git author/committer identity.

For browser checks, use the globally installed `agent-browser`. Do not add browser runtime dependencies to `flake.nix` for this repo.

## Component API Contract

Expose typed Askama template structs from modules such as `components` and `layouts`. Use consistent `Type::new(...)` constructors, named variant constructors, and by-value `with_*` builders for optional state. Consumer examples should use the constructors and builders, not struct literals, so new fields can be added without semver breakage.

Use `HtmlAttr` helpers for common htmx attributes. Attribute values and normal text are escaped by Askama. If a component needs an inner markup slot, require `TrustedHtml` or an equivalent explicit trusted wrapper; do not add anonymous `&str` fields that templates render with `|safe`.

Keep feature flags additive. The default crate should stay framework-neutral, with adapters such as Axum behind opt-in features.

Render errors should remain visible to consumers through Askama's render result. Do not hide rendering failures in shared component code.

## Issue Tracking

Use `br` for task tracking. Do not use `bd` or Dolt-backed workflows.

Useful commands:

```bash
br ready --json
br show <id> --json
br update <id> --claim --json
br create "Issue title" -t task -p 2 --description="Detailed context" --json
br close <id> --reason "Done" --json
br sync --flush-only --json
```

## Verification

Run these before calling a code change complete:

```bash
just fmt
just test
just clippy
cargo package --allow-dirty
```

## Askama Performance

Use Askama's own `render`, `render_into`, or `write_into` methods for template output. Avoid `to_string()` or `format!()` for templates in hot paths.

`Cargo.toml` keeps `profile.dev.package.askama_derive.opt-level = 3` so local incremental rebuilds remain workable as this crate accumulates templates.

Askama-derived templates already implement `FastWritable`. `TrustedHtml` implements it manually because component slots pass trusted markup through repeatedly; add manual implementations only for non-template wrapper types that show up in hot render paths and can write directly to `fmt::Write`.

Cached local rebuild check on 2026-05-16: after touching `src/components.rs`, `cargo check --all-features --example axum_gallery` completed in 1.10s real time. The gallery is the template-heavy smoke target for local path override iteration.
