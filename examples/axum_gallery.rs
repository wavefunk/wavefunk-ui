use askama::Template;
use axum::{Router, response::Html, routing::get};
use wavefunk_ui::components::{Alert, Button, FeedbackKind, HtmlAttr, Tag};
use wavefunk_ui::layouts::AppShell;

#[derive(Template)]
#[template(
    source = r#"
<div class="wf-nav-section">Gallery</div>
<a class="wf-nav-item is-active" href="/">Components</a>
"#,
    ext = "html"
)]
struct GalleryNav;

#[derive(Template)]
#[template(
    source = r#"
<section class="wf-panel">
  <div class="wf-panel-head">
    <div class="wf-panel-title">First slice</div>
    {{ tag }}
  </div>
  <div class="wf-panel-body" style="display: grid; gap: var(--space-4);">
    {{ alert }}
    <div>{{ button }}</div>
  </div>
</section>
"#,
    ext = "html"
)]
struct GalleryContent<'a> {
    tag: Tag<'a>,
    alert: Alert<'a>,
    button: Button<'a>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/toast", get(toast))
        .nest("/static/wavefunk", wavefunk_ui::axum::asset_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind gallery server");
    axum::serve(listener, app)
        .await
        .expect("run gallery server");
}

async fn index() -> Html<String> {
    let toast_attrs = [
        HtmlAttr::new("hx-get", "/toast"),
        HtmlAttr::new("hx-swap", "none"),
    ];
    let nav = GalleryNav.render().expect("render gallery nav");
    let content = GalleryContent {
        tag: Tag::status(FeedbackKind::Ok, "Embedded assets"),
        alert: Alert::new(
            FeedbackKind::Info,
            "wavefunk-ui components render as nested Askama values.",
        ),
        button: Button {
            attrs: &toast_attrs,
            ..Button::primary("Toast")
        },
    }
    .render()
    .expect("render gallery content");
    let shell = AppShell {
        nav_html: &nav,
        status_right: "0.1.0",
        ..AppShell::new("wavefunk-ui gallery", "WAVEFUNK UI", &content)
    };

    Html(shell.render().expect("render app shell"))
}

async fn toast() -> ([(axum::http::HeaderName, String); 1], &'static str) {
    (
        [(
            axum::http::HeaderName::from_static("hx-trigger"),
            wavefunk_ui::htmx::toast_header("ok", "Saved."),
        )],
        "",
    )
}
