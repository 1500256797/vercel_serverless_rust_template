use std::net::SocketAddr;

use dotenv::dotenv;
use tracing_subscriber;
use vercel_serverless_template::setup_server;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = setup_server();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
