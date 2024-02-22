use super::RpcResult;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};
use crate::api::v1::transaction_server::Transaction;

#[derive(Debug, Default)]
pub struct TransactionImpl {}

#[async_trait]
impl Transaction for TransactionImpl {
    async fn processar_transacao(
        &self,
        request: Request<TransacaoRequest>,
    ) -> RpcResult<TransacaoResponse> {
        println!("Transaction!!");
        Err(Status::unimplemented("todo!"))
    }
}