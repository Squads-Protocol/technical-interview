use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;
use tonic::transport::Server;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    // Load .env if present and initialize logging
    let _ = dotenvy::dotenv();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://interview.db".to_string());
    let db = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Ensure table exists to reduce setup friction in interviews
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS subscriptions (
            email TEXT PRIMARY KEY
        )"#,
    )
    .execute(&db)
    .await
    .expect("Failed to ensure subscriptions table exists");

    // add tracing to indicate start of server
    tracing::info!("Starting subscription service");

    let port = std::env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr: String = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr.clone())
        .await
        .expect("Failed to bind address");
    let incoming =
        TcpIncoming::from_listener(listener, true, None).expect("Failed to create incoming");

    let subscription_svc =
        subscription_service::pb::subscription_service_server::SubscriptionServiceServer::new(
            subscription_service::handlers::api::SubscriptionHandler::new(db),
        );

    tracing::info!("Server listening on {}", addr.clone());
    Server::builder()
        .add_service(subscription_svc)
        .serve_with_incoming(incoming)
        .await
        .expect("Failed to serve");
}
