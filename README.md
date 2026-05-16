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

The crate embeds the runtime assets under the stable public mount path `/static/wavefunk`:

- `css/wavefunk.css`
- `css/fonts/MartianGrotesk-VF.woff2`
- `css/fonts/MartianMono-VF.woff2`
- `js/wavefunk.js`
- `js/htmx.min.js`
- `js/htmx-sse.js`

No runtime `static/` directory is required in consuming binaries.

With Axum, enable the `axum` feature and mount the optional router:

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
let htmx = wavefunk_ui::assets::get("/static/wavefunk/js/htmx.min.js").unwrap();
```

`assets::get` normalizes both crate-relative paths and paths under `assets::DEFAULT_BASE_PATH`, rejects traversal, and returns bytes plus a content type. CSS and JavaScript are served as UTF-8 text, fonts as `font/woff2`, and unknown extensions as `application/octet-stream`.

Framework adapters use `assets::CACHE_CONTROL`, currently `public, max-age=0, must-revalidate`, so deployments can refresh unchanged asset paths safely.

The vendored htmx and htmx SSE assets are covered by `LICENSES.htmx.txt`. The Wave Funk CSS, JavaScript helper, and fonts are distributed with this crate under the package license.

## Component API Contract

Public UI primitives are exported as typed Askama template structs from modules such as `wavefunk_ui::components` and `wavefunk_ui::layouts`.

Component constructors follow a consistent pattern:

- Use `Type::new(...)` for the neutral variant.
- Use named constructors for common variants, such as `Button::primary(...)` and `Tag::status(...)`.
- Use by-value `with_*` methods for optional state, classes, attributes, htmx behavior, and trusted slots.
- Treat struct fields as readable implementation detail. Consumer code should use constructors and builders so new fields can be added without breaking semver.

Askama escapes normal text and attribute values. Use `HtmlAttr` helpers for common htmx attributes:

```rust
use wavefunk_ui::components::{Button, HtmlAttr};

let attrs = [
    HtmlAttr::hx_post("/contacts"),
    HtmlAttr::hx_swap("none"),
];

let button = Button::primary("Save").with_attrs(&attrs);
let html = button.render()?;
```

Any slot that must contain already-rendered markup is explicit:

```rust
use wavefunk_ui::components::{Field, TrustedHtml};

let control = TrustedHtml::new(r#"<input class="wf-input" name="email">"#);
let field = Field::new("Email", control).with_hint("Used for receipts.");
let html = field.render()?;
```

Only pass `TrustedHtml` content that was produced by your own templates or otherwise sanitized. User-provided text should remain normal `&str` data so Askama can escape it.

Component rendering uses Askama's `Result` type. Propagate render errors from request handlers instead of hiding them in shared UI code.

Feature flags are additive. The default feature set stays framework-neutral; framework adapters such as Axum are enabled with feature flags.

## Interaction Primitives

The shared JavaScript in `wavefunk.js` is intentionally generic:

- Popovers open when a trigger inside `.wf-pop-anchor` has `data-popover-toggle`; the matching `.wf-popover` closes when the user clicks outside it.
- Toasts are emitted with an htmx `HX-Trigger` payload for `wfToast`.
- Echo/minibuffer messages are emitted with `wfEcho` and update elements marked with `data-wf-echo`.

Use the htmx helpers to build response headers:

```rust
let (name, value) = wavefunk_ui::htmx::trigger_header_pair(&[
    wavefunk_ui::htmx::Trigger::toast("ok", "Saved."),
    wavefunk_ui::htmx::Trigger::echo("info", "Queued."),
])?;
```

Modal and drawer wrappers render the overlay plus the panel markup. Add `.open()` when server-rendering the visible state, and omit it for the hidden state. Popover wrappers render the `.wf-pop-anchor` plus `.wf-popover`; pass trigger markup that includes `data-popover-toggle`.

## Askama Performance

Use Askama's `render`, `render_into`, or `write_into` methods for template output. Avoid converting templates through `to_string()` or `format!()` in hot paths.

This crate optimizes `askama_derive` in the dev profile so local incremental builds stay practical as the component template set grows.

Askama-derived templates already implement `FastWritable`. `TrustedHtml` implements it manually because component slots pass trusted markup through repeatedly; add manual implementations only for non-template wrapper types that show up in hot render paths and can write directly to `fmt::Write`.

Cached local rebuild check on 2026-05-16: after touching `src/components.rs`, `cargo check --all-features --example axum_gallery` completed in 1.10s real time. The gallery is the template-heavy smoke target for local path override iteration.

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
