#[test]
fn shared_runtime_exposes_migration_helpers_by_data_attribute() {
    let js = include_str!("../static/wavefunk/js/wavefunk.js");

    for snippet in [
        "data-wf-copy",
        "navigator.clipboard.writeText",
        "wfFallbackCopyText",
        "data-wf-snippet-tab",
        "wfActivateSnippetTab",
        "data-upload-zone",
        "data-upload-input",
        "data-wf-submit-spinner",
        "data-wf-dirty-guard",
        "beforeunload",
        "data-wf-active-nav",
        "htmx:afterSwap",
        "page-title",
        "document.title",
        "wfEcho",
    ] {
        assert!(
            js.contains(snippet),
            "wavefunk.js missing helper: {snippet}"
        );
    }
}
