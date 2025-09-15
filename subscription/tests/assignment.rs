use subscription_service as svc;
use sqlx::SqlitePool;
use tonic::Request;

// Minimal sanity test to ensure test harness works
#[test]
fn sanity_compiles() {
    assert_eq!(1, 1);
}

// Helper to create an in-memory SQLite DB and a handler instance.
async fn setup_handler() -> svc::handlers::api::SubscriptionHandler {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create in-memory sqlite");

    // Ensure schema exists for tests
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS subscriptions (
            email TEXT PRIMARY KEY
        )"#,
    )
    .execute(&pool)
    .await
    .expect("failed to create table");

    svc::handlers::api::SubscriptionHandler::new(pool)
}

// Suggested starting point for candidates:
// - Remove #[ignore] to enable and flesh out the test body
// - Prefer covering pagination and filter behavior
#[tokio::test]
#[ignore]
async fn list_pagination_and_filter() {
    let handler = setup_handler().await;

    // TODO: insert sample rows directly
    // sqlx::query("INSERT INTO subscriptions (email) VALUES (?)")
    //     .bind("a@example.com")
    //     .execute(&handler.pool) // pool is private; consider creating a helper insert or test via subscribe
    //     .await
    //     .unwrap();

    // Tip: you can go through the public gRPC methods to keep it black-box
    // let _ = handler
    //     .subscribe(Request::new(svc::pb::SubscribeRequest { email: "a@example.com".into() }))
    //     .await
    //     .unwrap();

    // TODO: call list_subscriptions with different filters/page sizes once implemented
    // let resp = handler
    //     .list_subscriptions(Request::new(svc::pb::ListSubscriptionsRequest { /* fields once added */ }))
    //     .await
    //     .unwrap()
    //     .into_inner();

    // TODO: assert ordering, page boundaries, and next_page_token behavior
    // assert_eq!(resp.emails.len(), 2);
}

// Suggested starting point for candidates:
// - Remove #[ignore] to enable and validate email handling and idempotency
#[tokio::test]
#[ignore]
async fn subscribe_validation_and_idempotency() {
    let handler = setup_handler().await;

    // TODO: invalid email -> expect InvalidArgument
    // let err = handler
    //     .subscribe(Request::new(svc::pb::SubscribeRequest { email: "not-an-email".into() }))
    //     .await
    //     .expect_err("expected validation error");
    // assert_eq!(err.code(), tonic::Code::InvalidArgument);

    // TODO: valid email twice -> expect AlreadyExists or documented behavior
    // let _ = handler
    //     .subscribe(Request::new(svc::pb::SubscribeRequest { email: "ok@example.com".into() }))
    //     .await
    //     .unwrap();
    // let err = handler
    //     .subscribe(Request::new(svc::pb::SubscribeRequest { email: "ok@example.com".into() }))
    //     .await
    //     .expect_err("expected idempotency/duplicate handling");
    // assert_eq!(err.code(), tonic::Code::AlreadyExists);
}

