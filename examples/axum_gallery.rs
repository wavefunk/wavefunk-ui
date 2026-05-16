use askama::Template;
use axum::{Router, response::Html, routing::get};
use wavefunk_ui::components::{
    Alert, Button, ButtonGroup, CheckRow, FeedbackKind, Field, FieldState, HtmlAttr, IconButton,
    Input, InputGroup, Range, Select, SelectOption, SplitButton, Switch, Tag, Textarea,
    TrustedHtml,
};
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
    {{ field }}
    <div style="display: flex; flex-wrap: wrap; gap: var(--space-2); align-items: center;">
      {{ button_group }}
      {{ split_button }}
      {{ icon_button }}
      {{ button }}
    </div>
    <div style="display: grid; gap: var(--space-3); max-width: 560px;">
      {{ email_input }}
      {{ notes }}
      {{ plan_select }}
      {{ input_group }}
      {{ success_field }}
      <div style="display: flex; flex-wrap: wrap; gap: var(--space-3); align-items: center;">
        {{ checkbox }}
        {{ radio }}
        {{ switch_control }}
      </div>
      {{ range }}
    </div>
  </div>
</section>
"#,
    ext = "html"
)]
struct GalleryContent<'a> {
    tag: Tag<'a>,
    alert: Alert<'a>,
    field: Field<'a>,
    button_group: ButtonGroup<'a>,
    split_button: SplitButton<'a>,
    icon_button: IconButton<'a>,
    button: Button<'a>,
    email_input: Input<'a>,
    notes: Textarea<'a>,
    plan_select: Select<'a>,
    input_group: InputGroup<'a>,
    success_field: Field<'a>,
    checkbox: CheckRow<'a>,
    radio: CheckRow<'a>,
    switch_control: Switch<'a>,
    range: Range<'a>,
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
    let toast_attrs = [HtmlAttr::hx_get("/toast"), HtmlAttr::hx_swap("none")];
    let button_group_items = [
        Button::new("Draft"),
        Button::primary("Publish").with_attrs(&toast_attrs),
        Button::new("Archive"),
    ];
    let plan_options = [
        SelectOption::new("starter", "Starter"),
        SelectOption::new("team", "Team").selected(),
        SelectOption::new("enterprise", "Enterprise"),
    ];
    let grouped_input = Input::url("site_url")
        .with_placeholder("wavefunk.test")
        .render()
        .expect("render grouped input");
    let success_input = Input::new("project")
        .with_value("Substrukt")
        .render()
        .expect("render success input");
    let nav = GalleryNav.render().expect("render gallery nav");
    let content = GalleryContent {
        tag: Tag::status(FeedbackKind::Ok, "Embedded assets"),
        alert: Alert::new(
            FeedbackKind::Info,
            "wavefunk-ui components render as nested Askama values.",
        ),
        field: Field::new(
            "Email",
            TrustedHtml::new(r#"<input class="wf-input" name="email" type="email">"#),
        )
        .with_hint("Trusted slots keep form markup explicit."),
        button_group: ButtonGroup::new(&button_group_items),
        split_button: SplitButton::new(Button::primary("Deploy"), Button::new("More")),
        icon_button: IconButton::new(TrustedHtml::new("&times;"), "Dismiss"),
        button: Button::primary("Toast").with_attrs(&toast_attrs),
        email_input: Input::email("email").with_placeholder("you@wavefunk.test"),
        notes: Textarea::new("notes")
            .with_placeholder("Notes")
            .with_rows(4),
        plan_select: Select::new("plan", &plan_options),
        input_group: InputGroup::new(TrustedHtml::new(&grouped_input))
            .with_prefix("https://")
            .with_suffix(".wavefunk.test"),
        success_field: Field::new("Project", TrustedHtml::new(&success_input))
            .with_hint("Field success variant")
            .with_state(FieldState::Success),
        checkbox: CheckRow::checkbox("notify", "yes", "Notify me").checked(),
        radio: CheckRow::radio("mode", "dense", "Dense").checked(),
        switch_control: Switch::new("enabled").checked(),
        range: Range::new("signal")
            .with_bounds("0", "100")
            .with_value("64"),
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
