use axum::body::Body;
use axum::response::Response;

const SWAGGER_YAML: &str = include_str!("./swagger.yaml");

pub async fn serve_openapi_specs() -> Response {
    Response::builder()
        .header("content-type", "text/yaml")
        .body(Body::from(SWAGGER_YAML))
        .unwrap()
}
