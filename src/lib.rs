pub use async_trait::async_trait;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum TransformerError {
    #[error("function is unimplemented")]
    Unimplemented,
    #[error("Error: {0}")]
    Custom(String),
    #[error("unknown data store error")]
    Unknown,
    #[error("skip the data")]
    Skip,
}

pub type TransformerResult<T> = std::result::Result<T, TransformerError>;

enum DataSource {
    Hyper(String, u16),
    Redis,
    Kafka(String, u16, String),
    Unknown,
}

impl DataSource {}

#[async_trait]
pub trait Transformer{
    async fn transform(_inbound_data: &Vec<u8>) -> TransformerResult<Vec<String>> {
        Err(TransformerError::Unimplemented.into())
    }

    async fn init() -> TransformerResult<String> {
        Err(TransformerError::Unimplemented.into())
    }
}