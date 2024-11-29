use axum::{routing::get, Router};
use state::AppState;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod structs;
pub mod routes;
pub mod state;
pub fn setup_server() -> Router {
    let mut router = Router::new()
        .route("/", get(|| async { "hello world" }))
        .merge(routes::people::router())
        .with_state(AppState {})
        .layer(TraceLayer::new_for_http());
    if cfg!(feature = "local_dev") {
        #[derive(OpenApi)]
        #[openapi(
                    paths(
                        routes::people::get_people_info_handler,
                        routes::people::create_people_handler,
                    ),
                    components(
                        schemas(
                            routes::people::PeopleReq,
                            routes::people::PeopleResp,
                        ),
                    ),
                    tags(
                        (name = "hello vercel", description = "vercel rust template")
                    )
                )]
        struct ApiDoc;
        router = router
            .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));
    }
    router
}
