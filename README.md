# wavefunk-ui

Common Askama and htmx UI components for Wave Funk Rust applications.

The crate vendors the Wave Funk CSS, fonts, pinned htmx scripts, and small JavaScript helpers and embeds them into the Rust library. Consumers can ship self-contained binaries by mounting the provided asset handler instead of copying a runtime `static/` directory or loading htmx from a CDN.

## Install

```toml
[dependencies]
wavefunk-ui = "0.1"
```

For Axum asset serving:

```toml
[dependencies]
wavefunk-ui = { version = "0.1", features = ["axum"] }
```

## Embedded Assets

The public asset mount is expected to be `/static/wavefunk`.

```rust
let app = axum::Router::new()
    .nest("/static/wavefunk", wavefunk_ui::axum::asset_router());
```

Templates can use:

```rust
wavefunk_ui::html::stylesheet_link(wavefunk_ui::assets::DEFAULT_BASE_PATH);
wavefunk_ui::html::htmx_script_link(wavefunk_ui::assets::DEFAULT_BASE_PATH);
wavefunk_ui::html::script_link(wavefunk_ui::assets::DEFAULT_BASE_PATH);
```

The raw framework-neutral API is also available:

```rust
let css = wavefunk_ui::assets::get("css/wavefunk.css").unwrap();
```

## Askama Performance

Use Askama's `render`, `render_into`, or `write_into` methods for template output. Avoid converting templates through `to_string()` or `format!()` in hot paths.

This crate optimizes `askama_derive` in the dev profile so local incremental builds stay practical as the component template set grows.

## Local Consumer Iteration

Committed Wave Funk consumers should depend on the crates.io version:

```toml
wavefunk-ui = "0.1"
```

For local iteration, add a gitignored `.cargo/config.toml` in the consumer repo:

```toml
paths = [
    "../ui",
]
```

Do not commit the local path override. Release and CI should resolve the published crate from crates.io.
