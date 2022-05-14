use log::info;
use tonic::transport::Server;

use crate::api::review::{
    review::{self, review_server::ReviewServer},
    ReviewApi,
};
pub(crate) mod api;
pub(crate) mod repo;
pub(crate) mod svc;
pub(crate) mod third_party;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init database pool and run migrations
    env_logger::init();
    let pool = third_party::postgres::init_db_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("migrations are ready");

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(review::REVIEW_DESCRIPTOR)
        .build()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let review_svc = ReviewApi;

    info!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(service)
        .add_service(ReviewServer::new(review_svc))
        .serve(addr)
        .await?;

    Ok(())
}
