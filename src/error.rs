#[derive(Debug, thiserror::Error)]
pub enum Error {
    //---snip---//
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
    #[error(transparent)]
    TonicReflectionError(#[from] tonic_reflection::server::Error),
}