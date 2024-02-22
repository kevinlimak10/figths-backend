use super::RpcResult;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};
use crate::api::v1::extract_server::Extract;

#[derive(Debug, Default)]
pub struct ExtractImpl {}

#[async_trait]
impl Extract for ExtractImpl {
    async fn obter_extrato(&self, rq: Request<ExtratoRequest>) -> RpcResult<ExtratoResponse> {
        println!("Extrato!!");
        Err(Status::unimplemented("todo!"))
    }

}