use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::IntoResponse;

const SWAGGER_YAML: &str = include_str!("./swagger.yaml");

pub async fn serve_openapi_specs() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/yaml"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600, must-revalidate"),
    );

    (StatusCode::OK, headers, SWAGGER_YAML)
}
