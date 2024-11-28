use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use vercel_serverless_template::setup_server;

mod vercel_layer;
fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
async fn get_index_route() -> impl IntoResponse {
    let pokemon = "pikachu";
    Json(json!({ "message": format!("I choose you, {}!", pokemon) }))
}

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    init_tracing();
    // let app = setup_server();
    let app = Router::new()
        .route("/", get(get_index_route))
        .layer(TraceLayer::new_for_http());
    // let handler = ServiceBuilder::new()
    //     .map_request(process_request)
    //     .map_response(process_response)
    //     .layer(vercel_layer::VercelLayer::default())
    //     .service(app);
    let handler = vercel_runtime::ServiceBuilder::new()
        .map_request(vercel_runtime::process_request)
        .map_response(vercel_runtime::process_response)
        .layer(vercel_layer::LambdaLayer::default())
        .service(app);
    vercel_runtime::run_service(handler).await
}
