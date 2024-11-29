use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use vercel_serverless_template::setup_server;
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "vermicelli=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();
    let app = setup_server();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
