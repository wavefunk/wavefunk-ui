use rust_embed::RustEmbed;
use std::borrow::Cow;

pub const DEFAULT_BASE_PATH: &str = "/static/wavefunk";
pub const STYLESHEET_PATH: &str = "css/wavefunk.css";
pub const SCRIPT_PATH: &str = "js/wavefunk.js";
pub const HTMX_SCRIPT_PATH: &str = "js/htmx.min.js";
pub const HTMX_SSE_SCRIPT_PATH: &str = "js/htmx-sse.js";
pub const FONT_SANS_PATH: &str = "css/fonts/MartianGrotesk-VF.woff2";
pub const FONT_MONO_PATH: &str = "css/fonts/MartianMono-VF.woff2";
pub const CACHE_CONTROL: &str = "public, max-age=0, must-revalidate";

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
    let default_base_path = DEFAULT_BASE_PATH.trim_start_matches('/');
    let path = path
        .strip_prefix(default_base_path)
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
    fn serves_key_runtime_assets_with_content_types() {
        let cases = [
            (STYLESHEET_PATH, "text/css; charset=utf-8"),
            (SCRIPT_PATH, "text/javascript; charset=utf-8"),
            (HTMX_SCRIPT_PATH, "text/javascript; charset=utf-8"),
            (HTMX_SSE_SCRIPT_PATH, "text/javascript; charset=utf-8"),
            (FONT_SANS_PATH, "font/woff2"),
            (FONT_MONO_PATH, "font/woff2"),
        ];

        for (path, content_type) in cases {
            let asset = get(path).unwrap_or_else(|| panic!("{path} should be embedded"));
            assert_eq!(asset.path, path);
            assert_eq!(asset.content_type, content_type);
            assert!(!asset.bytes.is_empty());
        }
    }

    #[test]
    fn accepts_default_mount_paths() {
        let asset = get("/static/wavefunk/js/htmx.min.js").expect("public mount path should work");
        assert_eq!(asset.path, HTMX_SCRIPT_PATH);
    }

    #[test]
    fn lists_packaged_runtime_assets() {
        let paths = iter().collect::<Vec<_>>();

        assert!(paths.iter().any(|path| path.as_ref() == STYLESHEET_PATH));
        assert!(paths.iter().any(|path| path.as_ref() == SCRIPT_PATH));
        assert!(paths.iter().any(|path| path.as_ref() == HTMX_SCRIPT_PATH));
        assert!(
            paths
                .iter()
                .any(|path| path.as_ref() == HTMX_SSE_SCRIPT_PATH)
        );
        assert!(paths.iter().any(|path| path.as_ref() == FONT_SANS_PATH));
        assert!(paths.iter().any(|path| path.as_ref() == FONT_MONO_PATH));
    }

    #[test]
    fn exposes_shared_cache_policy() {
        assert_eq!(CACHE_CONTROL, "public, max-age=0, must-revalidate");
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(get("../Cargo.toml").is_none());
        assert!(get("css/../Cargo.toml").is_none());
        assert!(get("css//wavefunk.css").is_none());
        assert!(get("/static/wavefunk/../Cargo.toml").is_none());
        assert!(get(r"css\wavefunk.css").is_none());
    }
}
