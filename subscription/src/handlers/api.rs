use crate::pb::subscription_service_server::SubscriptionService;
use crate::pb::{
    ListSubscriptionsRequest, ListSubscriptionsResponse, PingRequest, PingResponse,
    SubscribeRequest, SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
};
use tonic::{Request, Response, Status};

pub struct SubscriptionHandler {
    pool: sqlx::SqlitePool,
}
impl SubscriptionHandler {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl SubscriptionService for SubscriptionHandler {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            message: format!("Pong {}", request.get_ref().message),
        }))
    }

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeResponse>, Status> {
        let req = request.into_inner();
        let email = req.email;
        sqlx::query!(
            r#"
            INSERT INTO subscriptions (email)
            VALUES (?)
            "#,
            email
        )
        .execute(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, "Failed to insert subscription");
            Status::internal("Failed to insert subscription")
        })?;

        Ok(Response::new(SubscribeResponse {}))
    }

    async fn list_subscriptions(
        &self,
        _request: Request<ListSubscriptionsRequest>,
    ) -> Result<Response<ListSubscriptionsResponse>, Status> {
        todo!()
    }

    async fn unsubscribe(
        &self,
        _request: Request<UnsubscribeRequest>,
    ) -> Result<Response<UnsubscribeResponse>, Status> {
        todo!()
    }
}
