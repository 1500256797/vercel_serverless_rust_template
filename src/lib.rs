use axum::{routing::get, Router};
use state::AppState;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod api;
pub mod routes;
pub mod state;

pub fn setup_server() -> Router {
//     #[derive(OpenApi)]
//     #[openapi(
//         paths(
//             routes::people::get_people_info_handler,
//             routes::people::create_people_handler,
//         ),
//         components(
//             schemas(
//                 routes::people::PeopleReq,
//                 routes::people::PeopleResp,
//             ),
//         ),
//         tags(
//             (name = "hello_axum", description = "axum 模版工程、集成pg、redis、swagger、sqlx")
//         )
//     )]
//     struct ApiDoc;

    return Router::new()
        .route("/", get(|| async { "hello world" }))
        .merge(routes::people::router())
        .with_state(AppState {})
        .layer(TraceLayer::new_for_http());
}
