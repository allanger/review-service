// use std::fmt::Result;

use sqlx::{Pool, Postgres};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod review {
    tonic::include_proto!("review");
    pub(crate) const REVIEW_DESCRIPTOR: &[u8] =
        tonic::include_file_descriptor_set!("review_descriptor");
}

use review::{ReviewWithId, ReviewWithoutId};

use crate::svc::review::{ReviewInterface, ReviewService};

use self::review::{review_server::Review, Id, ListOptions, ReviewWithIdSimple};

#[derive(Default)]
pub(crate) struct ReviewApi;

#[tonic::async_trait]
impl Review for ReviewApi {
    async fn create(
        &self,
        req: Request<ReviewWithoutId>,
    ) -> Result<Response<ReviewWithId>, Status> {
        let svc = ReviewService;
        let reply = svc.create(req.get_ref()).await.unwrap();
        Ok(Response::new(reply))
    }

    async fn get(&self, req: Request<Id>) -> Result<Response<ReviewWithId>, Status> {
        let svc = ReviewService;
        todo!()
        // let reply = svc.create(req).await;
    }

    type ListStream = ReceiverStream<Result<ReviewWithIdSimple, Status>>;

    async fn list(&self, req: Request<ListOptions>) -> Result<Response<Self::ListStream>, Status> {
        todo!()
    }
}
