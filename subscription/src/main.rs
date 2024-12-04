use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let db_url = "sqlite://interview.db";
    let db = SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to database");

    // add tracing to indicate start of server
    tracing::info!("Starting subscription service");

    let addr: String = format!("0.0.0.0:{}", "50051");

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
