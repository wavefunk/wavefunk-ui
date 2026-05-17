use askama::Template;
use axum::{Router, extract::Query, response::Html, routing::get};
use serde::Deserialize;
use wavefunk_ui::components::{
    Accordion, AccordionItem, Alert, Avatar, AvatarGroup, AvatarSize, Badge, BreadcrumbItem,
    Breadcrumbs, Button, ButtonGroup, ButtonVariant, Callout, Card, CheckRow, ControlSize,
    DataTable, DataTableCell, DataTableHeader, DataTableRow, DefinitionItem, DefinitionList,
    Drawer, Dropzone, EmptyState, Faq, FaqItem, FeatureGrid, FeatureItem, Feed, FeedRow,
    FeedbackKind, Field, FieldState, Form, FormActions, FormSection, Framed, Grid, HtmlAttr,
    IconButton, Input, InputGroup, Kbd, MarketingSection, MarketingStep, MarketingStepGrid, Menu,
    MenuItem, Meter, MeterColor, Minibuffer, Modal, NavItem, NavSection, PageLink, Pagination,
    Panel, Popover, PricingPlan, PricingPlans, Progress, Range, RankList, RankRow, SegmentOption,
    SegmentedControl, Select, SelectOption, Skeleton, SortDirection, Spinner, Split, SplitButton,
    Stat, StatRow, Statusbar, StepItem, Stepper, Switch, TabItem, Table, TableCell,
    TableColumnWidth, TableHeader, TableRow, TableWrap, Tabs, Tag, Testimonial, Textarea, Timeline,
    TimelineItem, Toast, ToastHost, Tooltip, Topbar, TreeItem, TreeView, TrustedHtml, UserButton,
    Wordmark,
};
use wavefunk_ui::layouts::AppShell;

#[derive(Template)]
#[template(
    source = r#"
<div class="wf-nav-section">Gallery</div>
<a class="wf-nav-item is-active" href="/">Components</a>
<div class="wf-nav-section">Variants</div>
<a class="wf-nav-item" href="/?mode=dark&density=dense">Dark dense</a>
<a class="wf-nav-item" href="/?mode=light&density=dense">Light dense</a>
<a class="wf-nav-item" href="/?mode=dark&density=default">Dark default</a>
<a class="wf-nav-item" href="/?mode=light&density=default">Light default</a>
"#,
    ext = "html"
)]
struct GalleryNav;

#[derive(Template)]
#[template(
    source = r#"
<a class="wf-btn ghost" href="/?mode=dark&density={{ density }}&state={{ state }}">Dark</a>
<a class="wf-btn ghost" href="/?mode=light&density={{ density }}&state={{ state }}">Light</a>
<a class="wf-btn ghost" href="/?mode={{ mode }}&density=default&state={{ state }}">Default density</a>
<a class="wf-btn ghost" href="/?mode={{ mode }}&density={{ density }}&state=open">Open overlays</a>
<a class="wf-btn ghost" href="/?mode={{ mode }}&density={{ density }}&state=drawer">Open drawer</a>
<a class="wf-btn ghost" href="/?mode={{ mode }}&density={{ density }}&state=loading">Loading</a>
<a class="wf-btn ghost" href="/?mode={{ mode }}&density={{ density }}&state=default">Reset state</a>
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
<div class="wf-g wf-gap-5 wf-min-w-0">
  <section class="wf-panel">
    <div class="wf-panel-head">
      <div class="wf-panel-title">Actions and forms</div>
      {{ tag }}
    </div>
    <div class="wf-panel-body wf-g wf-gap-4">
      {{ alert }}
      {{ field }}
      <div class="wf-f wf-wrap wf-gap-2 wf-ai-c">
        {{ button_group }}
        {{ split_button }}
        {{ icon_button }}
        {{ button }}
        {{ echo_button }}
      </div>
      <div class="wf-g wf-gap-3 wf-max-w-md">
        {{ email_input }}
        {{ notes }}
        {{ plan_select }}
        {{ input_group }}
        {{ success_field }}
        <div class="wf-f wf-wrap wf-gap-3 wf-ai-c">
          {{ checkbox }}
          {{ radio }}
          {{ switch_control }}
        </div>
        {{ range }}
      </div>
    </div>
  </section>
  {{ layout_showcase }}
  {{ feedback_showcase }}
  {{ extended_showcase }}
</div>
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
    extended_showcase: ExtendedShowcase<'a>,
}

#[derive(Template)]
#[template(
    source = r#"
<section class="wf-panel">
  <div class="wf-panel-head">
    <div class="wf-panel-title">Layout and data display</div>
    {{ badge }}
  </div>
  <div class="wf-panel-body wf-g wf-gap-4">
    {{ topbar }}
    <div class="wf-g wf-gap-2 wf-max-w-xs">
      {{ nav_section }}
      {{ nav_item }}
    </div>
    {{ breadcrumbs }}
    {{ tabs }}
    {{ segmented }}
    {{ pagination }}
    {{ stat_row }}
    <div class="wf-f wf-gap-3 wf-ai-c">
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
  <div class="wf-panel-body wf-g wf-gap-4">
    {{ callout }}
    {{ toast }}
    {{ toast_host }}
    {{ tooltip }}
    {{ popover }}
    {{ modal }}
    {{ drawer }}
    <div class="wf-g wf-gap-2">
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

#[derive(Template)]
#[template(
    source = r#"
<section class="wf-panel">
  <div class="wf-panel-head">
    <div class="wf-panel-title">Extended components</div>
    {{ badge }}
  </div>
  <div class="wf-panel-body wf-g wf-gap-4">
    {{ form_html }}
    {{ table_wrap_html }}
    <div class="wf-g wf-cols-2 wf-gap-4">
      {{ progress_html }}
      {{ meter_html }}
    </div>
    {{ kbd_html }}
    {{ stepper_html }}
    {{ accordion_html }}
    {{ faq_html }}
    <div class="wf-f wf-wrap wf-gap-4 wf-ai-c">
      {{ avatar_group_html }}
      {{ user_button_html }}
      {{ wordmark_html }}
    </div>
    {{ rank_list_html }}
    {{ feed_html }}
    {{ timeline_html }}
    {{ tree_html }}
    {{ framed_html }}
    {{ marketing_html }}
  </div>
</section>
"#,
    ext = "html"
)]
struct ExtendedShowcase<'a> {
    badge: Badge<'a>,
    form_html: TrustedHtml<'a>,
    table_wrap_html: TrustedHtml<'a>,
    progress_html: TrustedHtml<'a>,
    meter_html: TrustedHtml<'a>,
    kbd_html: Kbd<'a>,
    stepper_html: TrustedHtml<'a>,
    accordion_html: TrustedHtml<'a>,
    faq_html: TrustedHtml<'a>,
    avatar_group_html: TrustedHtml<'a>,
    user_button_html: UserButton<'a>,
    wordmark_html: Wordmark<'a>,
    rank_list_html: TrustedHtml<'a>,
    feed_html: TrustedHtml<'a>,
    timeline_html: TrustedHtml<'a>,
    tree_html: TrustedHtml<'a>,
    framed_html: Framed<'a>,
    marketing_html: TrustedHtml<'a>,
}

impl askama::filters::HtmlSafe for ExtendedShowcase<'_> {}

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

    fn loading_state(&self) -> bool {
        self.state() == "loading"
    }

    fn drawer_state(&self) -> bool {
        self.state() == "drawer"
    }
}

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

async fn index(Query(query): Query<GalleryQuery>) -> Html<String> {
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
    let form_email_input = Input::email("profile_email")
        .with_placeholder("you@wavefunk.test")
        .render()
        .expect("render profile input");
    let form_field = Field::new("Profile email", TrustedHtml::new(&form_email_input))
        .with_hint("Used for account notices.")
        .render()
        .expect("render profile field");
    let form_dropzone = Dropzone::new("avatar")
        .with_title("Drop avatar or click")
        .with_hint("PNG or JPG")
        .with_accept("image/png,image/jpeg")
        .render()
        .expect("render dropzone");
    let form_actions = FormActions::new(TrustedHtml::new(
        r#"<button class="wf-btn primary" type="submit">Save profile</button>"#,
    ))
    .with_secondary(TrustedHtml::new(
        r#"<button class="wf-btn" type="button">Cancel</button>"#,
    ))
    .render()
    .expect("render form actions");
    let form_section_body = format!("{form_field}{form_dropzone}");
    let form_section = FormSection::new("Profile", TrustedHtml::new(&form_section_body))
        .with_description("Composed from typed controls and trusted slots.")
        .with_actions(TrustedHtml::new(&form_actions))
        .render()
        .expect("render form section");
    let form_attrs = [HtmlAttr::hx_post("/profile"), HtmlAttr::hx_swap("none")];
    let form_html = Form::new(TrustedHtml::new(&form_section))
        .with_action("/profile")
        .with_method("post")
        .with_attrs(&form_attrs)
        .render()
        .expect("render form");
    let table_action = IconButton::new(TrustedHtml::new("&times;"), "Stop")
        .with_variant(ButtonVariant::Danger)
        .render()
        .expect("render table action");
    let workflow_headers = [
        DataTableHeader::new("Name").sortable("name", SortDirection::Ascending),
        DataTableHeader::numeric("Runs").with_width(TableColumnWidth::Small),
        DataTableHeader::new("Actions").action_column(),
    ];
    let workflow_cells = [
        DataTableCell::strong("Build"),
        DataTableCell::numeric("12"),
        DataTableCell::html(TrustedHtml::new(&table_action)),
    ];
    let workflow_rows = [DataTableRow::new(&workflow_cells).selected()];
    let workflow_table = DataTable::new(&workflow_headers, &workflow_rows)
        .interactive()
        .sticky()
        .pin_last()
        .render()
        .expect("render workflow table");
    let filter_input = Input::new("q")
        .with_size(ControlSize::Small)
        .with_placeholder("Search")
        .render()
        .expect("render filter input");
    let bulk_button = Button::new("Delete").render().expect("render bulk button");
    let table_wrap = TableWrap::new(TrustedHtml::new(&workflow_table))
        .with_filterbar(TrustedHtml::new(&filter_input))
        .with_bulkbar("1 selected", TrustedHtml::new(&bulk_button))
        .with_footer(TrustedHtml::new("Showing 1-1 of 1"))
        .render()
        .expect("render table wrap");
    let progress_html = if query.loading_state() {
        Progress::indeterminate()
    } else {
        Progress::new(64)
    }
    .render()
    .expect("render progress");
    let meter_html = Meter::new(72)
        .with_size_px(96, 6)
        .with_color(MeterColor::Ok)
        .render()
        .expect("render meter");
    let steps = [
        StepItem::new("Account").done(),
        StepItem::new("Profile").active().with_href("/profile"),
        StepItem::new("Invite"),
    ];
    let stepper_html = Stepper::new(&steps).render().expect("render stepper");
    let accordion_items = [
        AccordionItem::new(
            "What renders this?",
            TrustedHtml::new("<p>Askama templates.</p>"),
        )
        .open(),
        AccordionItem::new("Does it need JS?", TrustedHtml::new("<p>No.</p>")),
    ];
    let accordion_html = Accordion::new(&accordion_items)
        .render()
        .expect("render accordion");
    let faq_items = [FaqItem::new(
        "Why typed components?",
        TrustedHtml::new("<p>They keep consumer code semver-friendly.</p>"),
    )];
    let faq_html = Faq::new(&faq_items).render().expect("render faq");
    let avatars = [
        Avatar::new("SN").accent(),
        Avatar::new("WF").with_size(AvatarSize::Small),
    ];
    let avatar_group_html = AvatarGroup::new(&avatars)
        .render()
        .expect("render avatar group");
    let rank_rows = [
        RankRow::new("Builds", "42", 72),
        RankRow::new("Deploys", "18", 36),
    ];
    let rank_list_html = RankList::new(&rank_rows)
        .render()
        .expect("render rank list");
    let feed_rows = [
        FeedRow::new("09:41", "Deploy", "Published wavefunk-ui"),
        FeedRow::new("09:42", "Cache", "Assets embedded"),
    ];
    let feed_html = Feed::new(&feed_rows).render().expect("render feed");
    let timeline_items = [
        TimelineItem::new("09:40", "Queued", TrustedHtml::new("<p>Build started.</p>")),
        TimelineItem::new(
            "09:42",
            "Published",
            TrustedHtml::new("<p>Release ready.</p>"),
        )
        .active(),
    ];
    let timeline_html = Timeline::new(&timeline_items)
        .render()
        .expect("render timeline");
    let tree_children = [
        TreeItem::file("components.rs").active(),
        TreeItem::file("layouts.rs"),
    ];
    let tree_child_html = TreeView::new(&tree_children)
        .nested()
        .render()
        .expect("render nested tree");
    let tree_items = [TreeItem::folder("src").with_children(TrustedHtml::new(&tree_child_html))];
    let tree_html = TreeView::new(&tree_items).render().expect("render tree");
    let features = [
        FeatureItem::new("Typed APIs", "Constructors and builders preserve semver."),
        FeatureItem::new(
            "Embedded assets",
            "CSS, fonts, htmx, and helpers ship together.",
        ),
    ];
    let feature_grid = FeatureGrid::new(&features)
        .render()
        .expect("render feature grid");
    let marketing_steps = [
        MarketingStep::new("Install", "Depend on the crate."),
        MarketingStep::new("Render", "Compose Askama templates."),
    ];
    let marketing_step_grid = MarketingStepGrid::new(&marketing_steps)
        .render()
        .expect("render marketing steps");
    let pricing_plans = [
        PricingPlan::new("Starter", "$9").with_blurb("For small teams."),
        PricingPlan::new("Scale", "$29").with_unit("/mo").featured(),
    ];
    let pricing = PricingPlans::new(&pricing_plans)
        .render()
        .expect("render pricing");
    let testimonial = Testimonial::new(
        TrustedHtml::new("<p>Fast to wire into Rust apps.</p>"),
        "Wave Funk",
        "Design system",
    )
    .render()
    .expect("render testimonial");
    let marketing_body = format!("{feature_grid}{marketing_step_grid}{pricing}{testimonial}");
    let marketing_html =
        MarketingSection::new("Marketing primitives", TrustedHtml::new(&marketing_body))
            .with_kicker("Public pages")
            .with_subtitle(
                "Stable page sections can be typed without forcing a full landing-page framework.",
            )
            .render()
            .expect("render marketing section");
    let popover = Popover::new(
        TrustedHtml::new(r#"<button class="wf-btn" data-popover-toggle>Menu</button>"#),
        TrustedHtml::new(&menu_html),
    )
    .with_heading("Actions");
    let modal = Modal::new("Confirm", TrustedHtml::new("<p>Modal body.</p>")).with_footer(
        TrustedHtml::new(r#"<button class="wf-btn primary">Confirm</button>"#),
    );
    let drawer = Drawer::new("Details", TrustedHtml::new("<p>Drawer body.</p>"));
    let (popover, modal, drawer) = if query.open_state() {
        (popover.open(), modal.open(), drawer)
    } else if query.drawer_state() {
        (popover, modal, drawer.open())
    } else {
        (popover, modal, drawer)
    };
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
            popover,
            modal,
            drawer,
            skeleton_title: Skeleton::title(),
            skeleton_line: Skeleton::line(),
            spinner: Spinner::large(),
            minibuffer: Minibuffer::new().with_message(FeedbackKind::Info, "Ready"),
        },
        extended_showcase: ExtendedShowcase {
            badge: Badge::muted("new"),
            form_html: TrustedHtml::new(&form_html),
            table_wrap_html: TrustedHtml::new(&table_wrap),
            progress_html: TrustedHtml::new(&progress_html),
            meter_html: TrustedHtml::new(&meter_html),
            kbd_html: Kbd::new("Ctrl-K"),
            stepper_html: TrustedHtml::new(&stepper_html),
            accordion_html: TrustedHtml::new(&accordion_html),
            faq_html: TrustedHtml::new(&faq_html),
            avatar_group_html: TrustedHtml::new(&avatar_group_html),
            user_button_html: UserButton::new(
                "Sandeep Nambiar",
                "sandeep@wavefunk.test",
                Avatar::new("SN"),
            )
            .compact(),
            wordmark_html: Wordmark::new("Wave Funk")
                .with_mark(TrustedHtml::new(r#"<svg class="wf-mark"></svg>"#)),
            rank_list_html: TrustedHtml::new(&rank_list_html),
            feed_html: TrustedHtml::new(&feed_html),
            timeline_html: TrustedHtml::new(&timeline_html),
            tree_html: TrustedHtml::new(&tree_html),
            framed_html: Framed::new(TrustedHtml::new("<code>direnv exec . just test</code>"))
                .dense()
                .dashed(),
            marketing_html: TrustedHtml::new(&marketing_html),
        },
    }
    .render()
    .expect("render gallery content");
    let actions = GalleryActions {
        mode: query.mode(),
        density: query.density(),
        state: query.state(),
    }
    .render()
    .expect("render gallery actions");
    let shell = AppShell::new("wavefunk-ui gallery", "WAVEFUNK UI", &content)
        .with_nav(&nav)
        .with_actions(&actions)
        .with_mode(query.mode())
        .with_status("Gallery ready", "0.1.0");
    let shell = if query.default_density() {
        shell.default_density()
    } else {
        shell.dense()
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
