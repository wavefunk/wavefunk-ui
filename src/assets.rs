use rust_embed::RustEmbed;
use std::borrow::Cow;

pub const DEFAULT_BASE_PATH: &str = "/static/wavefunk";
pub const STYLESHEET_PATH: &str = "css/wavefunk.css";
pub const SCRIPT_PATH: &str = "js/wavefunk.js";
pub const HTMX_SCRIPT_PATH: &str = "js/htmx.min.js";
pub const HTMX_SSE_SCRIPT_PATH: &str = "js/htmx-sse.js";

#[derive(RustEmbed)]
#[folder = "static/wavefunk"]
struct EmbeddedAssets;

#[derive(Clone, Debug)]
pub struct Asset {
    pub path: String,
    pub bytes: Cow<'static, [u8]>,
    pub content_type: &'static str,
}

pub fn get(path: &str) -> Option<Asset> {
    let path = normalize_path(path)?;
    EmbeddedAssets::get(path).map(|file| Asset {
        path: path.to_owned(),
        bytes: file.data,
        content_type: content_type(path),
    })
}

pub fn iter() -> impl Iterator<Item = Cow<'static, str>> {
    EmbeddedAssets::iter()
}

pub fn content_type(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, ext)| ext) {
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("woff2") => "font/woff2",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

pub fn normalize_path(path: &str) -> Option<&str> {
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path
        .strip_prefix(DEFAULT_BASE_PATH)
        .and_then(|path| path.strip_prefix('/'))
        .unwrap_or(path);

    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return None;
    }

    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_vendored_stylesheet() {
        let asset = get(STYLESHEET_PATH).expect("stylesheet should be embedded");
        assert_eq!(asset.content_type, "text/css; charset=utf-8");
        assert!(!asset.bytes.is_empty());
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(get("../Cargo.toml").is_none());
        assert!(get("css/../Cargo.toml").is_none());
        assert!(get("css//wavefunk.css").is_none());
    }
}
