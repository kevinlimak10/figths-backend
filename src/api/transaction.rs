use super::RpcResult;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct TransactionImpl {}

#[async_trait]
impl Transaction for TransactionImpl {
    async fn processar_transacao(
        &self,
        request: Request<TransacaoRequest>,
    ) -> RpcResult<TransacaoResponse> {
        Err(Status::unimplemented("todo!"))
    }
}