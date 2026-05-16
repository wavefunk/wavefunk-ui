use crate::assets;

pub fn asset_router<S>() -> ::axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    ::axum::Router::<S>::new().route("/{*path}", ::axum::routing::get(asset))
}

async fn asset(
    ::axum::extract::Path(path): ::axum::extract::Path<String>,
) -> ::axum::response::Response {
    use ::axum::http::{HeaderMap, HeaderValue, StatusCode, header};
    use ::axum::response::IntoResponse;

    match assets::get(&path) {
        Some(asset) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(asset.content_type),
            );
            headers.insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static(assets::CACHE_CONTROL),
            );
            (StatusCode::OK, headers, asset.bytes.into_owned()).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
