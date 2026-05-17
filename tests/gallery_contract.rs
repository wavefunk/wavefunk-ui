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
