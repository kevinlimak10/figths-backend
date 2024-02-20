use super::RpcResult;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ExtractImpl {}

#[async_trait]
impl Extract for ExtractImpl {
    async fn obter_extrato(&self, rq: Request<ExtratoRequest>) -> RpcResult<ExtratoResponse> {
        Err(Status::unimplemented("todo!"))
    }

}