#[test]
fn gallery_example_uses_reusable_classes_for_static_layout() {
    let source = include_str!("../examples/axum_gallery.rs");

    assert!(
        !source.contains(r#"style=""#),
        "examples/axum_gallery.rs should use reusable classes instead of inline style attributes"
    );
}

#[test]
fn gallery_example_exposes_state_switches() {
    let source = include_str!("../examples/axum_gallery.rs");

    assert!(source.contains("state=open"));
    assert!(source.contains("state=drawer"));
    assert!(source.contains("state=loading"));
    assert!(source.contains("Open overlays"));
}

#[test]
fn axum_gallery_exposes_real_htmx_backend_routes() {
    let source = include_str!("../examples/axum_gallery.rs");

    for route in [
        r#".route("/components/{section}", get(component_page))"#,
        r#".route("/fragments/components/{section}", get(component_fragment))"#,
        r#".route("/profile", post(save_profile))"#,
        r#".route("/fragments/table", get(table_fragment))"#,
        r#".route("/fragments/loading", get(loading_fragment))"#,
    ] {
        assert!(source.contains(route), "missing route: {route}");
    }

    for htmx_attr in [
        r##"hx-target="#gallery-main""##,
        r#"hx-push-url"#,
        r#"HtmlAttr::hx_post("/profile")"#,
        r#"HtmlAttr::hx_get("/fragments/table")"#,
        r##"HtmlAttr::new("hx-indicator", "#profile-saving")"##,
        r#"HtmlAttr::hx_trigger("keyup changed delay:250ms, search")"#,
    ] {
        assert!(
            source.contains(htmx_attr),
            "missing htmx example: {htmx_attr}"
        );
    }

    for primitive in [
        "PageHeader::new(section.title)",
        "HtmxPartial::new(section.title, TrustedHtml::new(&main))",
        ".with_nav(TrustedHtml::new(&nav))",
        "FilterBar::new(TrustedHtml::new(&filter_input))",
        "RowSelect::new(\"workflow\", name, \"Select workflow\")",
        "BulkActionBar::new(\"1 selected\", TrustedHtml::new(&bulk_delete))",
        "TableFooter::new(TrustedHtml::new(\"Showing 1-4 of 4\"))",
    ] {
        assert!(
            source.contains(primitive),
            "gallery should exercise reusable page/partial primitive: {primitive}"
        );
    }

    for section in [
        "Forms and submission",
        "Data tables and filters",
        "Feedback and overlays",
        "Layout and navigation",
        "Extended primitives",
        "Migration-ready patterns",
    ] {
        assert!(
            source.contains(section),
            "sidebar/page should expose {section}"
        );
    }

    assert!(
        source.contains("Modal::new(\n        \"Confirm deployment\"")
            && source.contains(")\n    .large()"),
        "gallery should exercise large modal sizing"
    );
}

#[test]
fn gallery_example_exposes_migration_readiness_components() {
    let source = include_str!("../examples/axum_gallery.rs");

    for primitive in [
        "SplitShell::new(",
        "FormPanel::new(",
        "Modeline::new(",
        "ModelineSegment::",
        "ContextSwitcher::new(",
        "Sidenav::new(",
        "SidenavSection::new(",
        "SidenavItem::link(",
        "SecretValue::new(",
        "Checklist::new(",
        "CodeGrid::new(",
        "CodeBlock::new(",
        "SnippetTabs::new(",
        "StrengthMeter::new(",
        "SettingsSection::new(",
        "MinibufferEcho::info(",
        "InlineFormRow::new(",
        "CopyableValue::new(",
        "CredentialStatusList::new(",
        "ConfirmAction::new(",
        "ObjectFieldset::new(",
        "RepeatableArray::new(",
        "CurrentUpload::new(",
        "ReferenceSelect::new(",
        "MarkdownTextarea::new(",
        "RichTextHost::new(",
        r#"HtmlAttr::new("data-wf-dirty-guard", "true")"#,
        r##"HtmlAttr::new("data-wf-submit-spinner", "#profile-saving")"##,
    ] {
        assert!(
            source.contains(primitive),
            "gallery should exercise migration primitive: {primitive}"
        );
    }
}

#[test]
fn gallery_example_exposes_app_shell_extension_points() {
    let source = include_str!("../examples/axum_gallery.rs");

    for extension in [
        ".with_head(TrustedHtml::new(",
        ".with_topbar(TrustedHtml::new(&shell_topbar))",
        ".with_htmx_sse()",
        ".with_scripts(TrustedHtml::new(",
        "gallery-shell-config",
    ] {
        assert!(
            source.contains(extension),
            "gallery should demonstrate AppShell extension point: {extension}"
        );
    }
}

#[test]
fn app_shell_contains_mobile_overflow_guards() {
    let css = include_str!("../static/wavefunk/css/03-layout.css");
    let utilities = include_str!("../static/wavefunk/css/05-utilities.css");
    let components = include_str!("../static/wavefunk/css/04-components.css");

    assert!(css.contains("max-width: 100vw"));
    assert!(css.contains("overflow-x: auto"));
    assert!(css.contains("scrollbar-width: none"));
    assert!(utilities.contains(".wf-g > * { min-width: 0; }"));
    assert!(utilities.contains(".wf-f > * { min-width: 0; }"));
    assert!(components.contains(".wf-step { min-width: 0; }"));
    assert!(components.contains(".wf-stepper { flex-direction: column; }"));
}

#[test]
fn dropzone_hidden_input_does_not_force_scroll_width() {
    let css = include_str!("../static/wavefunk/css/04-components.css");

    assert!(css.contains(".wf-dropzone-input {\n  position: absolute; inset: 0;"));
    assert!(css.contains("width: 100%; height: 100%;"));
    assert!(css.contains("cursor: pointer"));
    assert!(css.contains("font-size: 0"));
}

#[test]
fn panel_row_components_extend_separators_to_panel_borders() {
    let css = include_str!("../static/wavefunk/css/04-components.css");
    let rule_start = css
        .find(".wf-panel-bleed,")
        .expect("panel row components should share a panel bleed selector");
    let selector_end = css[rule_start..]
        .find('{')
        .expect("panel bleed selector should have a declaration block");
    let selectors = &css[rule_start..rule_start + selector_end];

    for selector in [
        ".wf-table",
        ".wf-tablewrap",
        ".wf-dl",
        ".wf-stepper",
        ".wf-accordion",
        ".wf-faq",
        ".wf-statusbar",
        ".wf-rank",
        ".wf-feed",
        ".wf-empty.bordered",
    ] {
        assert!(
            selectors.contains(selector),
            "{selector} should bleed to panel borders when it draws row separators"
        );
    }

    assert!(css.contains("margin-inline: calc(0px - var(--wf-panel-pad-x, var(--space-4)))"));
    assert!(css.contains("width: calc(100% + var(--wf-panel-pad-x, var(--space-4)) + var(--wf-panel-pad-x, var(--space-4)))"));
    assert!(css.contains(".wf-panel-body > .wf-accordion .wf-accordion-trigger"));
    assert!(css.contains(".wf-panel-body > .wf-faq .wf-faq-row"));
}

#[test]
fn panel_bleed_preserves_inner_content_inset() {
    let css = include_str!("../static/wavefunk/css/04-components.css");

    for rule in [
        ".wf-panel-body > .wf-table:not(.flush) th:first-child",
        ".wf-panel-body > .wf-tablewrap .wf-table:not(.flush) th:first-child",
        ".wf-panel-body > .wf-tablewrap > .wf-filterbar",
        ".wf-panel-body > .wf-dl dt",
        ".wf-panel-body > .wf-rank .wf-rank-row",
        ".wf-panel-body > .wf-feed .wf-feed-row",
        ".wf-panel-body > .wf-statusbar",
        ".wf-panel-body > .wf-minibuffer",
        ".wf-panel-body > .wf-stepper > .wf-step:first-child",
    ] {
        assert!(
            css.contains(rule),
            "{rule} should restore content inset after panel bleed"
        );
    }

    assert!(css.contains("calc(20px + var(--wf-panel-pad-x, var(--space-4)))"));
    assert!(css.contains("calc(16px + var(--wf-panel-pad-x, var(--space-4)))"));
    assert!(css.contains("calc(12px + var(--wf-panel-pad-x, var(--space-4)))"));
}

#[test]
fn embedded_assets_are_canonical_to_this_crate() {
    let manifest = include_str!("../Cargo.toml");
    let justfile = include_str!("../justfile");
    let readme = include_str!("../README.md");
    let agent_notes = include_str!("../AGENTS.md");

    assert!(manifest.contains(r#""/AGENTS.md""#));
    assert!(manifest.contains(r#""/justfile""#));
    assert!(
        !justfile.contains("vendor-design"),
        "justfile should not expose a recipe that overwrites crate CSS from another repo"
    );
    assert!(
        !justfile.contains("../design"),
        "justfile should not copy CSS or fonts from ../design"
    );
    assert!(readme.contains("source of truth for Wave Funk runtime assets"));
    assert!(agent_notes.contains("Do not sync CSS or fonts from `../design`"));
}
