use crate::prelude::*;
mod extract;
mod transaction;

use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

pub mod v1 {
    tonic::include_proto!("protobuf");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("auction_v1_descriptor");
}

// spec_service returns reflection server to allow reading proto definition at runtime.
pub fn spec_service() -> Result<ServerReflectionServer<impl ServerReflection>> {
    let spec = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(v1::FILE_DESCRIPTOR_SET)
        .build()?;
    Ok(spec)
}

pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

use extract::ExtractImpl;
use transaction::TransactionImpl;


pub fn transaction_service() -> TransactionServer<impl Transaction> {
    TransactionServer::new(TransactionImpl::new())
}

pub fn extract_service() -> ExtractServer<impl Server> {
    ExtractServer::new(ExtractImpl::new())
}