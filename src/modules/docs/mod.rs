use axum::{Router, routing::get};

mod controller;

use controller::serve_openapi_specs;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::app::AppState;

pub fn docs_router() -> Router<AppState> {
    let swagger_config = Config::from("/api-docs/openapi.yaml");
    let swagger_ui = SwaggerUi::new("/docs").config(swagger_config);

    let router = Router::new()
        // Route to serve the actual specification file
        .route("/api-docs/openapi.yaml", get(serve_openapi_specs))
        .merge(swagger_ui);

    router
}
