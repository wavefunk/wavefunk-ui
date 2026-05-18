use askama::Template;
use axum::{
    Form as AxumForm, Router,
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;
use wavefunk_ui::components::{
    Accordion, AccordionItem, Alert, Avatar, AvatarGroup, AvatarSize, Badge, BreadcrumbItem,
    Breadcrumbs, BulkActionBar, Button, ButtonSize, ButtonVariant, Callout, Card, Checklist,
    ChecklistItem, CodeBlock, CodeGrid, ConfirmAction, ContextSwitcher, ContextSwitcherItem,
    CopyableValue, CredentialStatusItem, CredentialStatusList, CurrentUpload, DataTable,
    DataTableCell, DataTableHeader, DataTableRow, DefinitionItem, DefinitionList, Drawer, Dropzone,
    EmptyState, Faq, FaqItem, FeatureGrid, FeatureItem, Feed, FeedRow, FeedbackKind, Field,
    FieldState, FilterBar, Form, FormActions, FormPanel, FormSection, Framed, Grid, HtmlAttr,
    InlineFormRow, Input, Kbd, MarkdownTextarea, MarketingSection, MarketingStep,
    MarketingStepGrid, Menu, MenuItem, Meter, MeterColor, Minibuffer, MinibufferEcho,
    MinibufferHistoryRow, Modal, Modeline, ModelineSegment, NavItem, NavSection, ObjectFieldset,
    PageHeader, PageLink, Pagination, Panel, Popover, PricingPlan, PricingPlans, Progress,
    RankList, RankRow, ReferenceSelect, RepeatableArray, RepeatableItem, RichTextHost, RowSelect,
    SecretValue, SegmentOption, SegmentedControl, Select, SelectOption, SettingsSection, Sidenav,
    SidenavItem, SidenavSection, Skeleton, SnippetTab, SnippetTabs, SortDirection, Spinner, Split,
    SplitShell, Stat, StatRow, Statusbar, StepItem, Stepper, StrengthMeter, TabItem, Table,
    TableCell, TableColumnWidth, TableFooter, TableHeader, TableRow, TableWrap, Tabs, Testimonial,
    Textarea, Timeline, TimelineItem, Topbar, TreeItem, TreeView, TrustedHtml, UserButton,
    Wordmark,
};
use wavefunk_ui::layouts::{AppShell, HtmxPartial, SidebarProfile};

#[derive(Clone, Copy, Debug)]
struct SectionDef {
    id: &'static str,
    title: &'static str,
    badge: &'static str,
    blurb: &'static str,
}

const SECTIONS: &[SectionDef] = &[
    SectionDef {
        id: "forms",
        title: "Forms and submission",
        badge: "post",
        blurb: "A real Axum form posts to /profile, swaps a result fragment, and emits toast and echo HX-Trigger events.",
    },
    SectionDef {
        id: "data",
        title: "Data tables and filters",
        badge: "get",
        blurb: "Search input requests /fragments/table and replaces only the workflow table fragment.",
    },
    SectionDef {
        id: "feedback",
        title: "Feedback and overlays",
        badge: "ui",
        blurb: "Toast, echo, popover, modal, drawer, skeleton, and minibuffer states wired as they would be in a backend app.",
    },
    SectionDef {
        id: "layout",
        title: "Layout and navigation",
        badge: "shell",
        blurb: "Navigation, tabs, top bars, panels, grids, split panes, and data rows inside the shared app shell.",
    },
    SectionDef {
        id: "extended",
        title: "Extended primitives",
        badge: "more",
        blurb: "The less common primitives: stepper, accordion, FAQ, rank/feed rows, timeline, tree view, framed code, and marketing sections.",
    },
    SectionDef {
        id: "migration",
        title: "Migration-ready patterns",
        badge: "apps",
        blurb: "Reusable shell, admin, generated-form, and runtime patterns for backend applications.",
    },
];

#[derive(Template)]
#[template(
    source = r#"
<a class="wf-btn ghost" href="?mode=dark&density={{ density }}&state={{ state }}">Dark</a>
<a class="wf-btn ghost" href="?mode=light&density={{ density }}&state={{ state }}">Light</a>
<a class="wf-btn ghost" href="?mode={{ mode }}&density=default&state={{ state }}">Default density</a>
<a class="wf-btn ghost" href="?mode={{ mode }}&density={{ density }}&state=open">Open overlays</a>
<a class="wf-btn ghost" href="?mode={{ mode }}&density={{ density }}&state=drawer">Open drawer</a>
<a class="wf-btn ghost" href="?mode={{ mode }}&density={{ density }}&state=loading">Loading</a>
<a class="wf-btn ghost" href="?mode={{ mode }}&density={{ density }}&state=default">Reset state</a>
"#,
    ext = "html"
)]
struct GalleryActions<'a> {
    mode: &'a str,
    density: &'a str,
    state: &'a str,
}

#[derive(Template)]
#[template(
    source = r#"
<div id="gallery-main" class="wf-g wf-gap-5 wf-min-w-0">
  {{ header_html }}
  <section class="wf-panel">
    <div class="wf-panel-head">
      <div class="wf-panel-title">Route contract</div>
    </div>
    <div class="wf-panel-body wf-g wf-gap-4">
      {{ route_notes }}
    </div>
  </section>
  {{ body_html }}
</div>
"#,
    ext = "html"
)]
struct GalleryMain<'a> {
    header_html: TrustedHtml<'a>,
    route_notes: DefinitionList<'a>,
    body_html: TrustedHtml<'a>,
}

#[derive(Debug, Default, Deserialize)]
struct GalleryQuery {
    mode: Option<String>,
    density: Option<String>,
    state: Option<String>,
}

impl GalleryQuery {
    fn mode(&self) -> &'static str {
        match self.mode.as_deref() {
            Some("light") => "light",
            _ => "dark",
        }
    }

    fn default_density(&self) -> bool {
        matches!(self.density.as_deref(), Some("default"))
    }

    fn density(&self) -> &'static str {
        if self.default_density() {
            "default"
        } else {
            "dense"
        }
    }

    fn state(&self) -> &'static str {
        match self.state.as_deref() {
            Some("open") => "open",
            Some("drawer") => "drawer",
            Some("loading") => "loading",
            _ => "default",
        }
    }

    fn open_state(&self) -> bool {
        self.state() == "open"
    }

    fn drawer_state(&self) -> bool {
        self.state() == "drawer"
    }
}

#[derive(Debug, Deserialize)]
struct ProfileForm {
    profile_email: String,
    project: String,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TableQuery {
    q: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/components/{section}", get(component_page))
        .route("/fragments/components/{section}", get(component_fragment))
        .route("/profile", post(save_profile))
        .route("/fragments/table", get(table_fragment))
        .route("/fragments/loading", get(loading_fragment))
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

async fn index(Query(query): Query<GalleryQuery>) -> Html<String> {
    Html(render_shell(section_for("forms"), &query))
}

async fn component_page(
    Path(section): Path<String>,
    Query(query): Query<GalleryQuery>,
) -> Html<String> {
    Html(render_shell(section_for(&section), &query))
}

async fn component_fragment(
    Path(section): Path<String>,
    Query(query): Query<GalleryQuery>,
) -> Html<String> {
    Html(render_gallery_fragment(section_for(&section), &query))
}

async fn save_profile(AxumForm(form): AxumForm<ProfileForm>) -> impl IntoResponse {
    let headers = trigger_headers(&[
        wavefunk_ui::htmx::Trigger::toast("ok", "Profile saved."),
        wavefunk_ui::htmx::Trigger::echo("ok", "POST /profile returned a fragment."),
    ]);

    (headers, Html(profile_result(&form)))
}

async fn table_fragment(Query(query): Query<TableQuery>) -> Html<String> {
    Html(workflow_table_fragment(query.q.as_deref().unwrap_or("")))
}

async fn loading_fragment() -> Html<String> {
    tokio::time::sleep(std::time::Duration::from_millis(350)).await;
    Html(loading_result_fragment())
}

async fn toast() -> (HeaderMap, &'static str) {
    (
        trigger_headers(&[wavefunk_ui::htmx::Trigger::toast("ok", "Saved.")]),
        "",
    )
}

async fn echo() -> (HeaderMap, &'static str) {
    (
        trigger_headers(&[wavefunk_ui::htmx::Trigger::echo("info", "Queued.")]),
        "",
    )
}

fn render_shell(section: SectionDef, query: &GalleryQuery) -> String {
    let content = render_gallery_main(section, query);
    let nav = gallery_nav(section.id);
    let html_attrs = [HtmlAttr::new("data-gallery-shell", "true")];
    let actions = render(
        GalleryActions {
            mode: query.mode(),
            density: query.density(),
            state: query.state(),
        },
        "gallery actions",
    );
    let crumb_items = [
        BreadcrumbItem::link("Components", "/components/forms"),
        BreadcrumbItem::current(section.title),
    ];
    let shell_breadcrumbs = render(Breadcrumbs::new(&crumb_items), "shell breadcrumbs");
    let shell_topbar = render(
        Topbar::new(
            TrustedHtml::new(&shell_breadcrumbs),
            TrustedHtml::new(&actions),
        ),
        "shell topbar",
    );
    let mode_attrs = [HtmlAttr::new("data-mode-toggle", "")];
    let modeline_left = [
        ModelineSegment::chevron("WFUI"),
        ModelineSegment::buffer(section.id),
        ModelineSegment::button(query.mode()).with_attrs(&mode_attrs),
    ];
    let modeline_right = [
        ModelineSegment::position("L1:C1"),
        ModelineSegment::text("Ready").with_feedback(FeedbackKind::Ok),
    ];
    let modeline = render(
        Modeline::new(&modeline_left).with_right(&modeline_right),
        "gallery modeline",
    );
    let history = [MinibufferHistoryRow::new("09:41", "Loaded gallery section")
        .with_feedback(FeedbackKind::Info)];
    let minibuffer = render(
        Minibuffer::new()
            .with_prompt("gallery")
            .with_message(FeedbackKind::Info, "Use htmx links to swap sections.")
            .with_history(&history),
        "gallery minibuffer",
    );
    let footer = format!("{modeline}{minibuffer}");
    let shell = AppShell::new("wavefunk-ui gallery", "WAVEFUNK UI", &content)
        .with_head(TrustedHtml::new(
            r#"<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Crect width='16' height='16' rx='3' fill='black'/%3E%3C/svg%3E"><meta name="theme-color" content="oklch(0.58 0.16 250)">"#,
        ))
        .with_html_attrs(&html_attrs)
        .with_brand_href("/")
        .with_nav(&nav)
        .with_topbar(TrustedHtml::new(&shell_topbar))
        .with_footer(TrustedHtml::new(&footer))
        .with_htmx_sse()
        .with_scripts(TrustedHtml::new(
            r#"<script id="gallery-shell-config">window.wavefunkGalleryShell = true;</script>"#,
        ))
        .with_profile(
            SidebarProfile::new()
                .with_name("Wave Funk")
                .with_email("gallery@wavefunk.test")
                .with_avatar(Avatar::new("WF")),
        )
        .with_mode(query.mode())
        .with_status("Gallery ready", "0.1.0");
    let shell = if query.default_density() {
        shell.default_density()
    } else {
        shell.dense()
    };

    render(shell, "app shell")
}

fn render_gallery_main(section: SectionDef, query: &GalleryQuery) -> String {
    let body = match section.id {
        "data" => data_section(),
        "feedback" => feedback_section(query),
        "layout" => layout_section(),
        "extended" => extended_section(query),
        "migration" => migration_section(),
        _ => forms_section(),
    };
    let full_page_path = format!("/components/{}", section.id);
    let fragment_path = format!("/fragments/components/{}", section.id);
    let route_items = [
        DefinitionItem::new("Full page", &full_page_path),
        DefinitionItem::new("Fragment", &fragment_path),
        DefinitionItem::new("Swap target", "#gallery-main"),
    ];
    let header_badge = render(Badge::muted(section.badge), "section badge");
    let header = render(
        PageHeader::new(section.title)
            .with_subtitle(section.blurb)
            .with_meta(TrustedHtml::new(&header_badge)),
        "page header",
    );

    render(
        GalleryMain {
            header_html: TrustedHtml::new(&header),
            route_notes: DefinitionList::new(&route_items),
            body_html: TrustedHtml::new(&body),
        },
        "gallery main",
    )
}

fn render_gallery_fragment(section: SectionDef, query: &GalleryQuery) -> String {
    let main = render_gallery_main(section, query);
    let nav = gallery_nav(section.id);
    render(
        HtmxPartial::new(section.title, TrustedHtml::new(&main)).with_nav(TrustedHtml::new(&nav)),
        "gallery partial",
    )
}

fn gallery_nav(active: &str) -> String {
    let mut html = String::from(r#"<div class="wf-nav-section">Component examples</div>"#);
    for section in SECTIONS {
        html.push_str(&nav_link(*section, active));
    }
    html.push_str(
        r#"<div class="wf-nav-section">Variants</div>
<a class="wf-nav-item" href="/components/forms?mode=dark&density=dense">Dark dense</a>
<a class="wf-nav-item" href="/components/forms?mode=light&density=dense">Light dense</a>
<a class="wf-nav-item" href="/components/forms?mode=dark&density=default">Dark default</a>
<a class="wf-nav-item" href="/components/forms?mode=light&density=default">Light default</a>"#,
    );
    html
}

fn nav_link(section: SectionDef, active: &str) -> String {
    let class = if section.id == active {
        "wf-nav-item is-active"
    } else {
        "wf-nav-item"
    };
    format!(
        r##"<a class="{class}" href="/components/{id}" hx-get="/fragments/components/{id}" hx-target="#gallery-main" hx-swap="outerHTML" hx-push-url="/components/{id}">{title}</a>"##,
        id = section.id,
        title = section.title
    )
}

fn section_for(value: &str) -> SectionDef {
    SECTIONS
        .iter()
        .copied()
        .find(|section| section.id == value)
        .unwrap_or(SECTIONS[0])
}

fn forms_section() -> String {
    let profile_form = profile_form();
    let loading_demo = loading_demo();
    let dropzone = render(
        Dropzone::new("avatar")
            .with_title("Drop avatar or click")
            .with_hint("Dropzone is shown as a standalone component; the demo form posts URL-encoded fields.")
            .with_accept("image/png,image/jpeg"),
        "dropzone",
    );
    let form_routes = render(
        DefinitionList::new(&[
            DefinitionItem::new("Form submit", "POST /profile"),
            DefinitionItem::new("Result target", "#profile-result"),
            DefinitionItem::new("Loading route", "GET /fragments/loading"),
        ]),
        "form routes",
    );
    let body = format!("{profile_form}{loading_demo}{dropzone}{form_routes}");

    render(
        Panel::new("Backend form flow", TrustedHtml::new(&body)).with_action(TrustedHtml::new(
            r##"<a class="wf-panel-link" href="/components/data" hx-get="/fragments/components/data" hx-target="#gallery-main" hx-swap="outerHTML" hx-push-url="/components/data">Next</a>"##,
        )),
        "forms panel",
    )
}

fn profile_form() -> String {
    let email = render(
        Input::email("profile_email")
            .with_placeholder("you@wavefunk.test")
            .required(),
        "profile email input",
    );
    let email_field = render(
        Field::new("Profile email", TrustedHtml::new(&email))
            .with_hint("Submitted to Axum's Form extractor."),
        "profile email field",
    );
    let project = render(
        Input::new("project")
            .with_value("Wave Funk")
            .with_placeholder("Project"),
        "project input",
    );
    let project_field = render(
        Field::new("Project", TrustedHtml::new(&project))
            .with_hint("The response returns a partial, not a full page.")
            .with_state(FieldState::Success),
        "project field",
    );
    let notes = render(
        Textarea::new("notes")
            .with_rows(3)
            .with_placeholder("Optional note saved with the profile"),
        "profile notes",
    );
    let notes_field = render(Field::new("Notes", TrustedHtml::new(&notes)), "notes field");
    let submit = render(
        Button::primary("Save profile").with_button_type("submit"),
        "submit button",
    );
    let cancel = render(
        Button::new("Reset").with_button_type("reset"),
        "reset button",
    );
    let actions = render(
        FormActions::new(TrustedHtml::new(&submit)).with_secondary(TrustedHtml::new(&cancel)),
        "form actions",
    );
    let saving = render(Progress::indeterminate(), "profile saving progress");
    let body = format!(
        r#"{email_field}{project_field}{notes_field}<div id="profile-saving" class="htmx-indicator">{saving}</div>{actions}<div id="profile-result">{result}</div>"#,
        result = profile_result_initial()
    );
    let section = render(
        FormSection::new("Profile", TrustedHtml::new(&body)).with_description(
            "Submit the form and watch Axum return a target fragment plus HX-Trigger headers.",
        ),
        "profile form section",
    );
    let form_attrs = [
        HtmlAttr::hx_post("/profile"),
        HtmlAttr::hx_target("#profile-result"),
        HtmlAttr::hx_swap("outerHTML"),
        HtmlAttr::new("hx-indicator", "#profile-saving"),
        HtmlAttr::new("data-wf-submit-spinner", "#profile-saving"),
        HtmlAttr::new("data-wf-dirty-guard", "true"),
    ];

    render(
        Form::new(TrustedHtml::new(&section))
            .with_action("/profile")
            .with_method("post")
            .with_attrs(&form_attrs),
        "profile form",
    )
}

fn profile_result_initial() -> String {
    let empty = render(
        EmptyState::new(
            "No submission yet",
            "Submit the form to replace this fragment.",
        )
        .bordered()
        .dense(),
        "profile empty state",
    );
    format!(r#"<div id="profile-result">{empty}</div>"#)
}

fn profile_result(form: &ProfileForm) -> String {
    let notes = form.notes.as_deref().unwrap_or("").trim();
    let message = if notes.is_empty() {
        format!("Saved {} for {}.", form.profile_email, form.project)
    } else {
        format!(
            "Saved {} for {} with note: {notes}",
            form.profile_email, form.project
        )
    };
    let alert = render(Alert::new(FeedbackKind::Ok, &message), "profile result");
    format!(r#"<div id="profile-result">{alert}</div>"#)
}

fn loading_demo() -> String {
    let attrs = [
        HtmlAttr::hx_get("/fragments/loading"),
        HtmlAttr::hx_target("#loading-demo"),
        HtmlAttr::hx_swap("outerHTML"),
        HtmlAttr::new("hx-indicator", "#loading-demo-indicator"),
    ];
    let button = render(
        Button::new("Refresh usage").with_attrs(&attrs),
        "loading button",
    );
    let indicator = render(Spinner::new(), "loading spinner");
    let current = loading_idle_fragment();

    format!(
        r#"<div class="wf-g wf-gap-3"><div class="wf-f wf-gap-3 wf-ai-c">{button}<span id="loading-demo-indicator" class="htmx-indicator">{indicator}</span></div>{current}</div>"#
    )
}

fn loading_idle_fragment() -> String {
    let stats = [
        Stat::new("Latency", "18").with_unit("ms"),
        Stat::new("Requests", "128").with_unit("rpm"),
    ];
    let stat_row = render(StatRow::new(&stats), "idle loading stats");
    format!(r#"<div id="loading-demo">{stat_row}</div>"#)
}

fn loading_result_fragment() -> String {
    let stats = [
        Stat::new("Latency", "24").with_unit("ms"),
        Stat::new("Requests", "144").with_unit("rpm"),
    ];
    let stat_row = render(StatRow::new(&stats), "fresh loading stats");
    let progress = render(Progress::new(86), "fresh progress");
    let echo = render(
        Alert::new(
            FeedbackKind::Info,
            "GET /fragments/loading returned this partial.",
        ),
        "loading echo",
    );
    let minibuffer_echo = render(
        MinibufferEcho::info("GET /fragments/loading refreshed the usage panel."),
        "minibuffer echo source",
    );

    format!(
        r#"{minibuffer_echo}<div id="loading-demo" class="wf-g wf-gap-3">{stat_row}{progress}{echo}</div>"#
    )
}

fn data_section() -> String {
    let stats = [
        Stat::new("Queued", "3"),
        Stat::new("Successful", "18"),
        Stat::new("Failed", "1"),
    ];
    let stat_row = render(StatRow::new(&stats), "workflow stats");
    let data_routes = render(
        DefinitionList::new(&[
            DefinitionItem::new("Filter request", "GET /fragments/table?q=..."),
            DefinitionItem::new("Target", "#workflow-table"),
            DefinitionItem::new("Trigger", "keyup changed delay:250ms, search"),
        ]),
        "data routes",
    );
    let ranks = [
        RankRow::new("Builds", "42", 72),
        RankRow::new("Deploys", "18", 36),
        RankRow::new("Rollbacks", "1", 8),
    ];
    let rank_list = render(RankList::new(&ranks), "rank list");
    let feed_rows = [
        FeedRow::new("09:41", "Deploy", "Published wavefunk-ui"),
        FeedRow::new("09:42", "Cache", "Assets embedded"),
        FeedRow::new("09:45", "Route", "Returned table fragment"),
    ];
    let feed = render(Feed::new(&feed_rows), "feed");
    let body = format!(
        r#"<div class="wf-g wf-gap-4">{stat_row}{data_routes}{table}{rank_list}{feed}</div>"#,
        table = workflow_table_fragment("")
    );

    render(
        Panel::new("Interactive table fragment", TrustedHtml::new(&body)),
        "data panel",
    )
}

fn workflow_table_fragment(query: &str) -> String {
    const WORKFLOWS: &[(&str, &str, &str)] = &[
        ("Build", "12", "Stop"),
        ("Deploy", "7", "View"),
        ("Audit", "3", "Open"),
        ("Release", "2", "Promote"),
    ];

    let needle = query.trim().to_lowercase();
    let filter_attrs = [
        HtmlAttr::hx_get("/fragments/table"),
        HtmlAttr::hx_target("#workflow-table"),
        HtmlAttr::hx_trigger("keyup changed delay:250ms, search"),
        HtmlAttr::hx_swap("outerHTML"),
    ];
    let filter_input = render(
        Input::search("q")
            .with_placeholder("Filter workflows")
            .with_value(query)
            .with_attrs(&filter_attrs),
        "workflow filter",
    );
    let refresh = render(
        Button::new("Refresh")
            .with_size(ButtonSize::Small)
            .with_attrs(&[
                HtmlAttr::hx_get("/fragments/table"),
                HtmlAttr::hx_target("#workflow-table"),
            ]),
        "workflow refresh",
    );
    let filterbar = render(
        FilterBar::new(TrustedHtml::new(&filter_input)).with_actions(TrustedHtml::new(&refresh)),
        "workflow filterbar",
    );
    let headers = [
        DataTableHeader::new("").with_width(TableColumnWidth::Checkbox),
        DataTableHeader::sorted("Name", "name", SortDirection::Ascending),
        DataTableHeader::numeric("Runs").with_width(TableColumnWidth::Small),
        DataTableHeader::new("Actions").action_column(),
    ];
    let matches: Vec<_> = WORKFLOWS
        .iter()
        .copied()
        .filter(|(name, _, _)| needle.is_empty() || name.to_lowercase().contains(&needle))
        .collect();
    let selectors: Vec<String> = matches
        .iter()
        .map(|(name, _, _)| {
            render(
                RowSelect::new("workflow", name, "Select workflow").checked(),
                "workflow row select",
            )
        })
        .collect();
    let mut row_cells = Vec::new();
    for ((name, runs, _action), selector) in matches.iter().zip(selectors.iter()) {
        row_cells.push([
            DataTableCell::html(TrustedHtml::new(selector)),
            DataTableCell::strong(name),
            DataTableCell::numeric(runs),
            DataTableCell::html(TrustedHtml::new(
                r#"<button class="wf-icon-btn danger" type="button" aria-label="Stop">&times;</button>"#,
            )),
        ]);
    }

    let mut rows = Vec::new();
    for cells in &row_cells {
        rows.push(DataTableRow::new(cells));
    }

    let content = if rows.is_empty() {
        render(
            EmptyState::new("No matching workflows", "Try a different filter.")
                .bordered()
                .dense(),
            "empty table",
        )
    } else {
        let table = render(
            DataTable::new(&headers, &rows)
                .interactive()
                .sticky()
                .pin_last(),
            "workflow data table",
        );
        let bulk_delete = render(
            Button::new("Delete")
                .with_variant(ButtonVariant::Danger)
                .with_size(ButtonSize::Small),
            "workflow bulk delete",
        );
        let bulkbar = render(
            BulkActionBar::new("1 selected", TrustedHtml::new(&bulk_delete)),
            "workflow bulkbar",
        );
        let pages = [
            PageLink::link("1", "/components/data").active(),
            PageLink::link("2", "/components/data?page=2"),
        ];
        let pagination = render(Pagination::new(&pages), "workflow pagination");
        let footer = render(
            TableFooter::new(TrustedHtml::new("Showing 1-4 of 4"))
                .with_actions(TrustedHtml::new(&pagination)),
            "workflow footer",
        );
        render(
            TableWrap::new(TrustedHtml::new(&table))
                .with_filterbar_component(TrustedHtml::new(&filterbar))
                .with_bulkbar_component(TrustedHtml::new(&bulkbar))
                .with_footer_component(TrustedHtml::new(&footer)),
            "workflow table wrap",
        )
    };

    format!(r#"<div id="workflow-table">{content}</div>"#)
}

fn feedback_section(query: &GalleryQuery) -> String {
    let toast_attrs = [HtmlAttr::hx_get("/toast"), HtmlAttr::hx_swap("none")];
    let echo_attrs = [HtmlAttr::hx_get("/echo"), HtmlAttr::hx_swap("none")];
    let toast = render(
        Button::primary("Emit toast").with_attrs(&toast_attrs),
        "toast button",
    );
    let echo = render(
        Button::new("Echo minibuffer").with_attrs(&echo_attrs),
        "echo button",
    );
    let menu_items = [
        MenuItem::button("Open").with_kbd("O"),
        MenuItem::link("Settings", "/components/layout"),
        MenuItem::separator(),
        MenuItem::button("Delete").danger(),
    ];
    let menu = render(Menu::new(&menu_items), "menu");
    let popover = render(
        Popover::new(
            TrustedHtml::new(r#"<button class="wf-btn" data-popover-toggle>Menu</button>"#),
            TrustedHtml::new(&menu),
        )
        .with_heading("Actions")
        .open(),
        "popover",
    );
    let modal = Modal::new(
        "Confirm deployment",
        TrustedHtml::new("<p>Server-rendered modal markup with dismiss wiring.</p>"),
    )
    .large()
    .with_footer(TrustedHtml::new(
        r#"<button class="wf-btn primary" data-wf-dismiss="overlay">Confirm</button>"#,
    ));
    let drawer = Drawer::new(
        "Request details",
        TrustedHtml::new("<p>Drawer body can be returned as part of any route.</p>"),
    );
    let (modal, drawer) = if query.open_state() {
        (modal.open(), drawer)
    } else if query.drawer_state() {
        (modal, drawer.open())
    } else {
        (modal, drawer)
    };
    let modal = render(modal, "modal");
    let drawer = render(drawer, "drawer");
    let callout = render(
        Callout::new(
            FeedbackKind::Info,
            TrustedHtml::new(
                "<p>Toast and echo routes return HX-Trigger headers with empty bodies.</p>",
            ),
        )
        .with_title("Headers as UI events"),
        "feedback callout",
    );
    let skeletons = format!(
        r#"<div class="wf-g wf-gap-2">{title}{line}{block}</div>"#,
        title = render(Skeleton::title(), "skeleton title"),
        line = render(Skeleton::line(), "skeleton line"),
        block = render(Skeleton::block(), "skeleton block"),
    );
    let minibuffer = render(
        Minibuffer::new().with_message(FeedbackKind::Info, "Waiting for HX-Trigger"),
        "minibuffer",
    );
    let body = format!(
        r#"{callout}<div class="wf-f wf-wrap wf-gap-3 wf-ai-c">{toast}{echo}{popover}</div>{modal}{drawer}{skeletons}{minibuffer}"#
    );

    render(
        Panel::new("Feedback routes and overlays", TrustedHtml::new(&body)).with_action(
            TrustedHtml::new(r#"<span class="wf-panel-link">/toast + /echo</span>"#),
        ),
        "feedback panel",
    )
}

fn layout_section() -> String {
    let crumbs = [
        BreadcrumbItem::link("Workspace", "/components/layout"),
        BreadcrumbItem::current("Gallery"),
    ];
    let breadcrumbs = render(Breadcrumbs::new(&crumbs), "breadcrumbs");
    let topbar_badge = render(Badge::muted("live"), "topbar badge");
    let topbar = render(
        Topbar::new(
            TrustedHtml::new(&breadcrumbs),
            TrustedHtml::new(&topbar_badge),
        ),
        "topbar",
    );
    let tabs = [
        TabItem::link("Overview", "/components/layout").active(),
        TabItem::link("Settings", "/components/layout?tab=settings"),
    ];
    let tabs = render(Tabs::new(&tabs), "tabs");
    let segments = [
        SegmentOption::new("List", "list").active(),
        SegmentOption::new("Grid", "grid"),
    ];
    let segmented = render(SegmentedControl::new(&segments), "segmented");
    let pages = [
        PageLink::link("1", "/components/layout?page=1").active(),
        PageLink::ellipsis(),
        PageLink::disabled("Next"),
    ];
    let pagination = render(Pagination::new(&pages), "pagination");
    let stats = [
        Stat::new("Requests", "42").with_unit("rpm"),
        Stat::new("Errors", "0"),
    ];
    let stat_row = render(StatRow::new(&stats), "stats");
    let card = render(
        Card::new("Card", TrustedHtml::new("<p>Card body</p>"))
            .with_kicker("Raised")
            .raised(),
        "card",
    );
    let avatar = render(Avatar::new("WF").accent(), "avatar");
    let inner_panel = render(
        Panel::new("Panel", TrustedHtml::new("<p>Panel body</p>")).with_action(TrustedHtml::new(
            r#"<a class="wf-panel-link" href="/components/layout">Open</a>"#,
        )),
        "inner panel",
    );
    let headers = [TableHeader::new("Name"), TableHeader::numeric("Runs")];
    let cells = [TableCell::strong("Build"), TableCell::numeric("12")];
    let rows = [TableRow::new(&cells).selected()];
    let table = render(Table::new(&headers, &rows).interactive(), "layout table");
    let dl = render(
        DefinitionList::new(&[
            DefinitionItem::new("Runtime", "Rust"),
            DefinitionItem::new("Assets", "Embedded"),
        ]),
        "layout definition list",
    );
    let empty = render(
        EmptyState::new("No rows", "Create an item to start.")
            .with_glyph(TrustedHtml::new("&empty;"))
            .bordered(),
        "empty state",
    );
    let grid_card = render(
        Card::new(
            "Grid item",
            TrustedHtml::new("<p>Nested layout primitive.</p>"),
        ),
        "grid card",
    );
    let grid = render(
        Grid::new(TrustedHtml::new(&grid_card)).with_columns(2),
        "grid",
    );
    let split = render(
        Split::new(TrustedHtml::new(
            "<div>Primary pane</div><div>Secondary pane</div>",
        ))
        .vertical(),
        "split",
    );
    let statusbar = render(Statusbar::new("Connected", "v0.1"), "statusbar");
    let nav = format!(
        r#"<div class="wf-g wf-gap-2 wf-max-w-xs">{nav_section}{nav_item}</div>"#,
        nav_section = render(NavSection::new("Workspace"), "nav section"),
        nav_item = render(
            NavItem::new("Dashboard", "/components/layout")
                .active()
                .with_count("3"),
            "nav item",
        ),
    );
    let body = format!(
        r#"{topbar}{nav}{breadcrumbs}{tabs}{segmented}{pagination}{stat_row}<div class="wf-f wf-gap-3 wf-ai-c">{avatar}{card}</div>{inner_panel}{table}{dl}{empty}{grid}{split}{statusbar}"#
    );

    render(
        Panel::new("Layout primitives in an app shell", TrustedHtml::new(&body)),
        "layout panel",
    )
}

fn migration_section() -> String {
    let shell_notes = render(
        DefinitionList::new(&[
            DefinitionItem::new(
                "Shell chrome",
                "Custom head, topbar, breadcrumbs, and scripts",
            ),
            DefinitionItem::new(
                "Partial swaps",
                "HtmxPartial updates title and app navigation",
            ),
            DefinitionItem::new(
                "Runtime hooks",
                "Copy, upload, dirty-form, and submit-spinner helpers",
            ),
        ]),
        "migration shell notes",
    );
    let shell_panel = render(
        Panel::new("Shell extension contract", TrustedHtml::new(&shell_notes)),
        "migration shell panel",
    );
    let split_shell = migration_split_shell_section();
    let navigation = migration_navigation_section();
    let settings = migration_settings_section();
    let sensitive_values = migration_sensitive_value_section();
    let snippets = migration_snippet_section();
    let generated = migration_generated_form_section();

    format!(
        r#"{shell_panel}{split_shell}{navigation}{settings}{sensitive_values}{snippets}{generated}"#
    )
}

fn migration_split_shell_section() -> String {
    let name = render(
        Input::new("project_name").with_placeholder("Project name"),
        "setup project input",
    );
    let name_field = render(
        Field::new("Project name", TrustedHtml::new(&name))
            .with_hint("Consumer applications provide the field names and validation."),
        "setup project field",
    );
    let continue_button = render(
        Button::primary("Continue").with_button_type("submit"),
        "setup continue",
    );
    let form = render(
        Form::new(TrustedHtml::new(&name_field)).with_action("/components/migration/setup"),
        "setup form",
    );
    let form_panel = render(
        FormPanel::new("Setup surface", TrustedHtml::new(&form))
            .with_subtitle("Split shells, form panels, and actions remain generic.")
            .with_actions(TrustedHtml::new(&continue_button)),
        "form panel",
    );
    let visual = render(
        Callout::new(
            FeedbackKind::Info,
            TrustedHtml::new(
                "<p>Use the visual slot for product artwork, status, or preview UI.</p>",
            ),
        )
        .with_title("Visual slot"),
        "split visual callout",
    );
    let footer = render(
        Statusbar::new("Setup ready", "generic shell"),
        "split shell footer",
    );
    let split = render(
        SplitShell::new(TrustedHtml::new(&form_panel))
            .with_top(TrustedHtml::new(
                r#"<a class="wf-btn ghost" href="/components/migration">Back</a>"#,
            ))
            .with_visual(TrustedHtml::new(&visual))
            .with_footer(TrustedHtml::new(&footer))
            .with_mode("dark"),
        "split shell",
    );

    render(
        Panel::new("Setup shell composition", TrustedHtml::new(&split)),
        "split shell panel",
    )
}

fn migration_navigation_section() -> String {
    let contexts = [
        ContextSwitcherItem::link("Production", "/components/migration")
            .with_meta("4 apps")
            .active(),
        ContextSwitcherItem::link("Sandbox", "/components/migration?sandbox=true")
            .with_badge(TrustedHtml::new(r#"<span class="wf-tag">test</span>"#)),
    ];
    let switcher = render(
        ContextSwitcher::new("Workspace", "Production", &contexts)
            .with_meta(TrustedHtml::new(r#"<span class="wf-tag ok">live</span>"#))
            .open(),
        "context switcher",
    );
    let side_items = [
        SidenavItem::link("Overview", "/components/migration").active(),
        SidenavItem::link("Settings", "/components/migration/settings"),
        SidenavItem::link("Reports", "/components/migration/reports")
            .muted()
            .with_badge("Soon"),
        SidenavItem::link("Billing", "/components/migration/billing")
            .disabled()
            .with_coming_soon("coming soon"),
    ];
    let side_sections = [SidenavSection::new("Manage", &side_items)];
    let sidenav = render(Sidenav::new(&side_sections), "sidenav");
    let body = format!(r#"<div class="wf-g wf-gap-4 wf-max-w-sm">{switcher}{sidenav}</div>"#);

    render(
        Panel::new("Switcher and sidenav composition", TrustedHtml::new(&body)),
        "navigation panel",
    )
}

fn migration_settings_section() -> String {
    let email = render(
        Input::email("notification_email").with_value("ops@wavefunk.test"),
        "settings email",
    );
    let save = render(
        Button::primary("Save")
            .with_button_type("submit")
            .with_size(ButtonSize::Small),
        "settings save",
    );
    let inline = render(
        InlineFormRow::new("Notification email", TrustedHtml::new(&email))
            .with_hint("Used for operational notices.")
            .with_action(TrustedHtml::new(&save)),
        "inline form row",
    );
    let copy = render(
        CopyableValue::new(
            "Webhook URL",
            "gallery-webhook",
            "https://example.test/hooks/gallery",
        )
        .with_button_label("Copy URL")
        .secret(),
        "copyable value",
    );
    let statuses = [
        CredentialStatusItem::ok("Email", "Configured"),
        CredentialStatusItem::warn("Signing key", "Rotation due"),
        CredentialStatusItem::info("SSE", "Available"),
    ];
    let status_list = render(
        CredentialStatusList::new(&statuses),
        "credential status list",
    );
    let confirm = render(
        ConfirmAction::new("Remove item", "/components/migration/remove")
            .with_message("Guarded actions stay generic.")
            .with_confirm("Remove this item?"),
        "confirm action",
    );
    let body = format!("{inline}{copy}{status_list}{confirm}");

    render(
        SettingsSection::new("Settings and value display", TrustedHtml::new(&body))
            .with_description("Generic settings, copyable values, and guarded actions.")
            .danger(),
        "settings section",
    )
}

fn migration_sensitive_value_section() -> String {
    let secret = render(
        SecretValue::new("Generated value", "gallery-value", "wf_live_example_value")
            .revealed()
            .with_button_label("Copy value")
            .with_warning("Store generated values before leaving this screen.")
            .with_help(TrustedHtml::new(
                "<p>Lifecycle and disclosure rules belong in the consumer application.</p>",
            )),
        "secret value",
    );
    let checklist_items = [
        ChecklistItem::ok("Domain verified")
            .with_description("Records were checked by the host app."),
        ChecklistItem::warn("Rotation window").with_status_label("review"),
        ChecklistItem::info("Notification channel").with_status_label("optional"),
    ];
    let checklist = render(Checklist::new(&checklist_items), "checklist");
    let codes = ["F4KC-9T7Q", "2F8M-QP1D", "N8K3-VW6R", "7J2H-L0PS"];
    let code_grid = render(
        CodeGrid::new(&codes).with_label("One-time codes"),
        "code grid",
    );
    let strength = render(
        StrengthMeter::new(3, 4, "Strong")
            .with_label("Signal strength")
            .with_feedback(FeedbackKind::Ok)
            .live(),
        "strength meter",
    );
    let body = format!("{secret}{checklist}{code_grid}{strength}");

    render(
        SettingsSection::new("Sensitive value displays", TrustedHtml::new(&body))
            .with_description("Reusable display components without policy or scoring logic."),
        "sensitive display section",
    )
}

fn migration_snippet_section() -> String {
    let block = render(
        CodeBlock::new("cargo add wavefunk-ui --features axum")
            .with_label("Install")
            .with_language("shell")
            .with_copy_target("gallery-install-command"),
        "code block",
    );
    let tabs = [
        SnippetTab::new(
            "Rust",
            r#"let button = wavefunk_ui::components::Button::primary("Save");"#,
        )
        .with_language("rust")
        .active(),
        SnippetTab::new("Shell", "direnv exec . cargo test").with_language("shell"),
    ];
    let snippets = render(SnippetTabs::new("gallery-snippets", &tabs), "snippet tabs");
    let body = format!("{block}{snippets}");

    render(
        Panel::new("Technical content blocks", TrustedHtml::new(&body)),
        "snippet panel",
    )
}

fn migration_generated_form_section() -> String {
    let title = render(
        Input::new("title").with_placeholder("Title"),
        "generated title input",
    );
    let title_field = render(
        Field::new("Title", TrustedHtml::new(&title))
            .with_hint("Generated from consumer metadata."),
        "generated title field",
    );
    let metadata = render(
        ObjectFieldset::new("Metadata", TrustedHtml::new(&title_field))
            .with_description("Nested object group with consumer-owned fields."),
        "object fieldset",
    );
    let link_input = render(
        Input::url("links[0][url]").with_placeholder("https://example.test"),
        "repeatable link input",
    );
    let link_field = render(
        Field::new("URL", TrustedHtml::new(&link_input)),
        "repeatable link field",
    );
    let link_item = render(
        RepeatableItem::new("Link 1", TrustedHtml::new(&link_field)).with_actions(
            TrustedHtml::new(r#"<button class="wf-btn sm" type="button">Remove</button>"#),
        ),
        "repeatable item",
    );
    let links = render(
        RepeatableArray::new("Links", TrustedHtml::new(&link_item))
            .with_description("Repeatable array item structure.")
            .with_action(TrustedHtml::new(
                r#"<button class="wf-btn sm" type="button">Add link</button>"#,
            )),
        "repeatable array",
    );
    let upload = render(
        CurrentUpload::new(
            "Current file",
            "/static/wavefunk/css/wavefunk.css",
            "wavefunk.css",
        )
        .with_meta("Embedded stylesheet"),
        "current upload",
    );
    let references = [
        SelectOption::new("forms", "Forms"),
        SelectOption::new("data", "Data").selected(),
        SelectOption::new("layout", "Layout"),
    ];
    let select = render(
        Select::new("related_section", &references),
        "reference select control",
    );
    let reference = render(
        ReferenceSelect::new("Related section", TrustedHtml::new(&select))
            .with_hint("Reference lookup policy belongs to the consumer."),
        "reference select",
    );
    let markdown = render(
        MarkdownTextarea::new("summary")
            .with_placeholder("Write markdown")
            .with_rows(5),
        "markdown textarea",
    );
    let rich_toolbar = r#"<button class="wf-btn sm" type="button">B</button><button class="wf-btn sm" type="button">I</button>"#;
    let richtext = render(
        RichTextHost::new("gallery-richtext", "rich_body")
            .with_toolbar(TrustedHtml::new(rich_toolbar))
            .with_body(TrustedHtml::new("<p>Trusted rich text slot.</p>")),
        "rich text host",
    );
    let body = format!(
        r#"<div class="wf-g wf-gap-4">{metadata}{links}{upload}{reference}{markdown}{richtext}</div>"#
    );

    render(
        Panel::new("Generated form building blocks", TrustedHtml::new(&body)),
        "generated form panel",
    )
}

fn extended_section(query: &GalleryQuery) -> String {
    let profile_form = profile_form();
    let progress = if query.state() == "loading" {
        render(Progress::indeterminate(), "indeterminate progress")
    } else {
        render(Progress::new(64), "progress")
    };
    let meter = render(
        Meter::new(72)
            .with_size_px(96, 6)
            .with_color(MeterColor::Ok),
        "meter",
    );
    let kbd = render(Kbd::new("Ctrl-K"), "kbd");
    let steps = [
        StepItem::new("Account").done(),
        StepItem::new("Profile")
            .active()
            .with_href("/components/forms"),
        StepItem::new("Invite"),
    ];
    let stepper = render(Stepper::new(&steps), "stepper");
    let accordion_items = [
        AccordionItem::new(
            "What renders this?",
            TrustedHtml::new("<p>Askama templates.</p>"),
        )
        .open(),
        AccordionItem::new("Does it need JS?", TrustedHtml::new("<p>No.</p>")),
    ];
    let accordion = render(Accordion::new(&accordion_items), "accordion");
    let faq_items = [FaqItem::new(
        "Why typed components?",
        TrustedHtml::new("<p>They keep consumer code semver-friendly.</p>"),
    )];
    let faq = render(Faq::new(&faq_items), "faq");
    let avatars = [
        Avatar::new("SN").accent(),
        Avatar::new("WF").with_size(AvatarSize::Small),
    ];
    let avatar_group = render(AvatarGroup::new(&avatars), "avatar group");
    let user_button = render(
        UserButton::new(
            "Sandeep Nambiar",
            "sandeep@wavefunk.test",
            Avatar::new("SN"),
        )
        .compact(),
        "user button",
    );
    let wordmark = render(
        Wordmark::new("Wave Funk").with_mark(TrustedHtml::new(r#"<svg class="wf-mark"></svg>"#)),
        "wordmark",
    );
    let ranks = [
        RankRow::new("Builds", "42", 72),
        RankRow::new("Deploys", "18", 36),
    ];
    let rank_list = render(RankList::new(&ranks), "rank list");
    let feed_rows = [
        FeedRow::new("09:41", "Deploy", "Published wavefunk-ui"),
        FeedRow::new("09:42", "Cache", "Assets embedded"),
    ];
    let feed = render(Feed::new(&feed_rows), "feed");
    let timeline_items = [
        TimelineItem::new("09:40", "Queued", TrustedHtml::new("<p>Build started.</p>")),
        TimelineItem::new(
            "09:42",
            "Published",
            TrustedHtml::new("<p>Release ready.</p>"),
        )
        .active(),
    ];
    let timeline = render(Timeline::new(&timeline_items), "timeline");
    let tree_children = [
        TreeItem::file("components.rs").active(),
        TreeItem::file("layouts.rs"),
    ];
    let tree_child_html = render(TreeView::new(&tree_children).nested(), "nested tree");
    let tree_items = [TreeItem::folder("src").with_children(TrustedHtml::new(&tree_child_html))];
    let tree = render(TreeView::new(&tree_items), "tree");
    let framed = render(
        Framed::new(TrustedHtml::new("<code>direnv exec . just test</code>"))
            .dense()
            .dashed(),
        "framed",
    );
    let features = [
        FeatureItem::new("Typed APIs", "Constructors and builders preserve semver."),
        FeatureItem::new(
            "Embedded assets",
            "CSS, fonts, htmx, and helpers ship together.",
        ),
    ];
    let feature_grid = render(FeatureGrid::new(&features), "feature grid");
    let marketing_steps = [
        MarketingStep::new("Install", "Depend on the crate."),
        MarketingStep::new("Render", "Compose Askama templates."),
    ];
    let marketing_step_grid = render(MarketingStepGrid::new(&marketing_steps), "marketing steps");
    let pricing_plans = [
        PricingPlan::new("Starter", "$9").with_blurb("For small teams."),
        PricingPlan::new("Scale", "$29").with_unit("/mo").featured(),
    ];
    let pricing = render(PricingPlans::new(&pricing_plans), "pricing");
    let testimonial = render(
        Testimonial::new(
            TrustedHtml::new("<p>Fast to wire into Rust apps.</p>"),
            "Wave Funk",
            "Design system",
        ),
        "testimonial",
    );
    let marketing_body = format!("{feature_grid}{marketing_step_grid}{pricing}{testimonial}");
    let marketing = render(
        MarketingSection::new("Marketing primitives", TrustedHtml::new(&marketing_body))
            .with_kicker("Public pages")
            .with_subtitle(
                "Stable page sections can be typed without forcing a full landing-page framework.",
            ),
        "marketing",
    );
    let body = format!(
        r#"{profile_form}<div class="wf-g wf-cols-2 wf-gap-4">{progress}{meter}</div>{kbd}{stepper}{accordion}{faq}<div class="wf-f wf-wrap wf-gap-4 wf-ai-c">{avatar_group}{user_button}{wordmark}</div>{rank_list}{feed}{timeline}{tree}{framed}{marketing}"#
    );

    render(
        Panel::new("Extended component examples", TrustedHtml::new(&body)),
        "extended panel",
    )
}

fn trigger_headers(triggers: &[wavefunk_ui::htmx::Trigger<'_>]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let value = wavefunk_ui::htmx::trigger_header(triggers).expect("render HX-Trigger header");
    headers.insert(
        wavefunk_ui::htmx::HX_TRIGGER_HEADER,
        HeaderValue::from_str(&value).expect("valid HX-Trigger value"),
    );
    headers
}

fn render<T: Template>(template: T, label: &str) -> String {
    template
        .render()
        .unwrap_or_else(|err| panic!("render {label}: {err}"))
}
