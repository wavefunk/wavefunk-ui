use crate::assets;

pub fn asset_url(base_path: &str, path: &str) -> String {
    let base_path = base_path.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{base_path}/{path}")
}

pub fn stylesheet_link(base_path: &str) -> String {
    format!(
        r#"<link rel="stylesheet" href="{}">"#,
        asset_url(base_path, assets::STYLESHEET_PATH)
    )
}

pub fn script_link(base_path: &str) -> String {
    format!(
        r#"<script src="{}" defer></script>"#,
        asset_url(base_path, assets::SCRIPT_PATH)
    )
}

pub fn htmx_script_link(base_path: &str) -> String {
    format!(
        r#"<script src="{}" defer></script>"#,
        asset_url(base_path, assets::HTMX_SCRIPT_PATH)
    )
}

pub fn htmx_sse_script_link(base_path: &str) -> String {
    format!(
        r#"<script src="{}" defer></script>"#,
        asset_url(base_path, assets::HTMX_SSE_SCRIPT_PATH)
    )
}
