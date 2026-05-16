use askama::Template;
use axum::{Router, response::Html, routing::get};
use wavefunk_ui::components::{
    Alert, Avatar, Badge, BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Callout, Card,
    CheckRow, DefinitionItem, DefinitionList, Drawer, EmptyState, FeedbackKind, Field, FieldState,
    Grid, HtmlAttr, IconButton, Input, InputGroup, Menu, MenuItem, Minibuffer, Modal, NavItem,
    NavSection, PageLink, Pagination, Panel, Popover, Range, SegmentOption, SegmentedControl,
    Select, SelectOption, Skeleton, Spinner, Split, SplitButton, Stat, StatRow, Statusbar, Switch,
    TabItem, Table, TableCell, TableHeader, TableRow, Tabs, Tag, Textarea, Toast, ToastHost,
    Tooltip, Topbar, TrustedHtml,
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
      {{ echo_button }}
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
    {{ layout_showcase }}
    {{ feedback_showcase }}
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
    echo_button: Button<'a>,
    email_input: Input<'a>,
    notes: Textarea<'a>,
    plan_select: Select<'a>,
    input_group: InputGroup<'a>,
    success_field: Field<'a>,
    checkbox: CheckRow<'a>,
    radio: CheckRow<'a>,
    switch_control: Switch<'a>,
    range: Range<'a>,
    layout_showcase: LayoutShowcase<'a>,
    feedback_showcase: FeedbackShowcase<'a>,
}

#[derive(Template)]
#[template(
    source = r#"
<section class="wf-panel">
  <div class="wf-panel-head">
    <div class="wf-panel-title">Layout and data display</div>
    {{ badge }}
  </div>
  <div class="wf-panel-body" style="display: grid; gap: var(--space-4);">
    {{ topbar }}
    <div style="display: grid; gap: var(--space-2); max-width: 280px;">
      {{ nav_section }}
      {{ nav_item }}
    </div>
    {{ breadcrumbs }}
    {{ tabs }}
    {{ segmented }}
    {{ pagination }}
    {{ stat_row }}
    <div style="display: flex; gap: var(--space-3); align-items: center;">
      {{ avatar }}
      {{ card }}
    </div>
    {{ panel }}
    {{ table }}
    {{ definition_list }}
    {{ empty_state }}
    {{ grid }}
    {{ split }}
    {{ statusbar }}
  </div>
</section>
"#,
    ext = "html"
)]
struct LayoutShowcase<'a> {
    panel: Panel<'a>,
    card: Card<'a>,
    stat_row: StatRow<'a>,
    badge: Badge<'a>,
    avatar: Avatar<'a>,
    breadcrumbs: Breadcrumbs<'a>,
    tabs: Tabs<'a>,
    segmented: SegmentedControl<'a>,
    pagination: Pagination<'a>,
    nav_section: NavSection<'a>,
    nav_item: NavItem<'a>,
    topbar: Topbar<'a>,
    statusbar: Statusbar<'a>,
    empty_state: EmptyState<'a>,
    table: Table<'a>,
    definition_list: DefinitionList<'a>,
    grid: Grid<'a>,
    split: Split<'a>,
}

impl askama::filters::HtmlSafe for LayoutShowcase<'_> {}

#[derive(Template)]
#[template(
    source = r#"
<section class="wf-panel">
  <div class="wf-panel-head">
    <div class="wf-panel-title">Feedback and overlays</div>
    {{ spinner }}
  </div>
  <div class="wf-panel-body" style="display: grid; gap: var(--space-4);">
    {{ callout }}
    {{ toast }}
    {{ toast_host }}
    {{ tooltip }}
    {{ popover }}
    {{ modal }}
    {{ drawer }}
    <div style="display: grid; gap: var(--space-2);">
      {{ skeleton_title }}
      {{ skeleton_line }}
    </div>
    {{ minibuffer }}
  </div>
</section>
"#,
    ext = "html"
)]
struct FeedbackShowcase<'a> {
    callout: Callout<'a>,
    toast: Toast<'a>,
    toast_host: ToastHost<'a>,
    tooltip: Tooltip<'a>,
    popover: Popover<'a>,
    modal: Modal<'a>,
    drawer: Drawer<'a>,
    skeleton_title: Skeleton,
    skeleton_line: Skeleton,
    spinner: Spinner,
    minibuffer: Minibuffer<'a>,
}

impl askama::filters::HtmlSafe for FeedbackShowcase<'_> {}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/toast", get(toast))
        .route("/echo", get(echo))
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
    let echo_attrs = [HtmlAttr::hx_get("/echo"), HtmlAttr::hx_swap("none")];
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
    let stats = [
        Stat::new("Requests", "42").with_unit("rpm"),
        Stat::new("Errors", "0"),
    ];
    let crumbs = [
        BreadcrumbItem::link("Workspace", "/"),
        BreadcrumbItem::current("Gallery"),
    ];
    let tabs = [
        TabItem::link("Overview", "/").active(),
        TabItem::link("Settings", "/settings"),
    ];
    let segments = [
        SegmentOption::new("List", "list").active(),
        SegmentOption::new("Grid", "grid"),
    ];
    let pages = [
        PageLink::link("1", "/page/1").active(),
        PageLink::ellipsis(),
        PageLink::disabled("Next"),
    ];
    let menu_items = [
        MenuItem::button("Open"),
        MenuItem::link("Settings", "/settings"),
        MenuItem::separator(),
        MenuItem::button("Delete").danger(),
    ];
    let table_headers = [TableHeader::new("Name"), TableHeader::numeric("Runs")];
    let table_cells = [TableCell::strong("Build"), TableCell::numeric("12")];
    let table_rows = [TableRow::new(&table_cells).selected()];
    let definition_items = [
        DefinitionItem::new("Runtime", "Rust"),
        DefinitionItem::new("Assets", "Embedded"),
    ];
    let grouped_input = Input::url("site_url")
        .with_placeholder("wavefunk.test")
        .render()
        .expect("render grouped input");
    let success_input = Input::new("project")
        .with_value("Substrukt")
        .render()
        .expect("render success input");
    let breadcrumbs_html = Breadcrumbs::new(&crumbs)
        .render()
        .expect("render breadcrumbs");
    let topbar_badge_html = Badge::muted("live").render().expect("render topbar badge");
    let card_for_grid = Card::new("Card", TrustedHtml::new("<p>Grid item</p>"))
        .render()
        .expect("render grid card");
    let menu_html = Menu::new(&menu_items).render().expect("render menu");
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
        echo_button: Button::new("Echo").with_attrs(&echo_attrs),
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
        layout_showcase: LayoutShowcase {
            panel: Panel::new("Panel", TrustedHtml::new("<p>Panel body</p>")).with_action(
                TrustedHtml::new(r#"<a class="wf-panel-link" href="/panel">Open</a>"#),
            ),
            card: Card::new("Card", TrustedHtml::new("<p>Card body</p>"))
                .with_kicker("Raised")
                .raised(),
            stat_row: StatRow::new(&stats),
            badge: Badge::muted("layout"),
            avatar: Avatar::new("WF").accent(),
            breadcrumbs: Breadcrumbs::new(&crumbs),
            tabs: Tabs::new(&tabs),
            segmented: SegmentedControl::new(&segments),
            pagination: Pagination::new(&pages),
            nav_section: NavSection::new("Workspace"),
            nav_item: NavItem::new("Dashboard", "/").active().with_count("3"),
            topbar: Topbar::new(
                TrustedHtml::new(&breadcrumbs_html),
                TrustedHtml::new(&topbar_badge_html),
            ),
            statusbar: Statusbar::new("Connected", "v0.1"),
            empty_state: EmptyState::new("No rows", "Create an item to start.")
                .with_glyph(TrustedHtml::new("&empty;"))
                .bordered(),
            table: Table::new(&table_headers, &table_rows).interactive(),
            definition_list: DefinitionList::new(&definition_items),
            grid: Grid::new(TrustedHtml::new(&card_for_grid)).with_columns(2),
            split: Split::new(TrustedHtml::new(
                "<div>Primary pane</div><div>Secondary pane</div>",
            ))
            .vertical(),
        },
        feedback_showcase: FeedbackShowcase {
            callout: Callout::new(
                FeedbackKind::Info,
                TrustedHtml::new("<p>Callout body for shared UI notices.</p>"),
            )
            .with_title("Notice"),
            toast: Toast::new(FeedbackKind::Ok, "Saved."),
            toast_host: ToastHost::new(),
            tooltip: Tooltip::new("Copy id", TrustedHtml::new(r#"<button>copy</button>"#)),
            popover: Popover::new(
                TrustedHtml::new(r#"<button class="wf-btn" data-popover-toggle>Menu</button>"#),
                TrustedHtml::new(&menu_html),
            )
            .with_heading("Actions"),
            modal: Modal::new("Confirm", TrustedHtml::new("<p>Modal body.</p>")).with_footer(
                TrustedHtml::new(r#"<button class="wf-btn primary">Confirm</button>"#),
            ),
            drawer: Drawer::new("Details", TrustedHtml::new("<p>Drawer body.</p>")),
            skeleton_title: Skeleton::title(),
            skeleton_line: Skeleton::line(),
            spinner: Spinner::large(),
            minibuffer: Minibuffer::new().with_message(FeedbackKind::Info, "Ready"),
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
    let (name, value) =
        wavefunk_ui::htmx::trigger_header_pair(&[wavefunk_ui::htmx::Trigger::toast(
            "ok", "Saved.",
        )])
        .expect("render HX-Trigger header");

    (
        [(
            axum::http::HeaderName::from_bytes(name.as_bytes()).expect("valid header name"),
            value,
        )],
        "",
    )
}

async fn echo() -> ([(axum::http::HeaderName, String); 1], &'static str) {
    let (name, value) =
        wavefunk_ui::htmx::trigger_header_pair(&[wavefunk_ui::htmx::Trigger::echo(
            "info", "Queued.",
        )])
        .expect("render HX-Trigger header");

    (
        [(
            axum::http::HeaderName::from_bytes(name.as_bytes()).expect("valid header name"),
            value,
        )],
        "",
    )
}
