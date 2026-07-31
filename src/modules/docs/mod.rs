use axum::{Router, routing::get};

mod controller;

use controller::{serve_openapi_specs, serve_swagger_ui};

pub fn docs_router() -> Router {
    let router = Router::new()
        // Route to serve the actual specification file
        .route("/api-docs/openapi.yaml", get(serve_openapi_specs))
        // Route to render the interactive Swagger UI page
        .route("/docs", get(serve_swagger_ui));

    router
}
