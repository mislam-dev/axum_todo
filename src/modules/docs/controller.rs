use axum::response::{Html, Response};

// 1. Compile the hardcoded YAML file directly into the binary
const SWAGGER_YAML: &str = include_str!("./swagger.yaml");

// 2. Handler to serve the raw YAML file with the correct MIME type
pub async fn serve_openapi_specs() -> Response {
    Response::builder()
        .header("content-type", "text/yaml")
        .body(axum::body::Body::from(SWAGGER_YAML))
        .unwrap()
}

// 3. Handler to serve a CDN-backed HTML shell for Swagger UI
pub async fn serve_swagger_ui() -> Html<String> {
    Html(format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>Swagger UI</title>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css" />
    </head>
    <body>
        <div id="swagger-ui"></div>
        <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-bundle.js"></script>
        <script>
            window.onload = () => {{
                window.ui = SwaggerUIBundle({{
                    url: '/api-docs/openapi.yaml',
                    dom_id: '#swagger-ui',
                }});
            }};
        </script>
    </body>
    </html>
    "#
    ))
}
