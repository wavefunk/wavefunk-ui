use crate::assets;
use std::borrow::Cow;

pub fn asset_router<S>() -> ::axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    ::axum::Router::<S>::new().route("/{*path}", ::axum::routing::get(asset))
}

async fn asset(
    ::axum::extract::Path(path): ::axum::extract::Path<String>,
    request_headers: ::axum::http::HeaderMap,
) -> ::axum::response::Response {
    use ::axum::http::{HeaderMap, HeaderValue, StatusCode, header};
    use ::axum::response::IntoResponse;

    match assets::get(&path) {
        Some(asset) => {
            let etag = assets::etag(&asset.path).expect("embedded asset should have an entity tag");
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(asset.content_type),
            );
            headers.insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static(assets::CACHE_CONTROL),
            );
            headers.insert(
                header::ETAG,
                HeaderValue::from_str(&etag).expect("asset entity tags should be valid headers"),
            );

            if if_none_match_matches(request_headers.get(header::IF_NONE_MATCH), &etag) {
                return (StatusCode::NOT_MODIFIED, headers).into_response();
            }

            (StatusCode::OK, headers, body_from_asset_bytes(asset.bytes)).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn body_from_asset_bytes(bytes: Cow<'static, [u8]>) -> ::axum::body::Body {
    match bytes {
        Cow::Borrowed(bytes) => ::axum::body::Body::from(bytes),
        Cow::Owned(bytes) => ::axum::body::Body::from(bytes),
    }
}

fn if_none_match_matches(header: Option<&::axum::http::HeaderValue>, etag: &str) -> bool {
    header
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value.split(',').any(|candidate| {
                let candidate = candidate.trim();
                candidate == "*" || candidate == etag || candidate.strip_prefix("W/") == Some(etag)
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::axum::body;
    use ::axum::http::{StatusCode, header};

    #[tokio::test]
    async fn asset_response_sets_runtime_headers_and_body() {
        let response = asset(
            ::axum::extract::Path(crate::assets::STYLESHEET_PATH.to_owned()),
            ::axum::http::HeaderMap::new(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/css; charset=utf-8"
        );
        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            crate::assets::CACHE_CONTROL
        );
        let etag = response
            .headers()
            .get(header::ETAG)
            .expect("asset response should expose an entity tag")
            .to_str()
            .expect("etag should be valid ascii");
        assert!(etag.starts_with('"') && etag.ends_with('"'));

        let bytes = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(!bytes.is_empty());
    }

    #[tokio::test]
    async fn missing_asset_returns_not_found_without_asset_headers() {
        let response = asset(
            ::axum::extract::Path("missing.css".to_owned()),
            ::axum::http::HeaderMap::new(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(response.headers().get(header::CONTENT_TYPE).is_none());
        assert!(response.headers().get(header::ETAG).is_none());
    }

    #[tokio::test]
    async fn matching_entity_tag_returns_not_modified() {
        let etag = crate::assets::etag(crate::assets::STYLESHEET_PATH).unwrap();
        let mut headers = ::axum::http::HeaderMap::new();
        headers.insert(header::IF_NONE_MATCH, etag.parse().unwrap());

        let response = asset(
            ::axum::extract::Path(crate::assets::STYLESHEET_PATH.to_owned()),
            headers,
        )
        .await;

        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(response.headers().get(header::ETAG).unwrap(), etag.as_str());
        let bytes = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(bytes.is_empty());
    }

    #[test]
    fn asset_handler_does_not_force_embed_bytes_into_owned_vecs() {
        let source = include_str!("axum.rs");
        let forbidden = concat!("into", "_owned()");

        assert!(
            !source.contains(forbidden),
            "asset responses should move owned debug bytes or borrow embedded bytes without an extra Vec copy"
        );
    }
}
