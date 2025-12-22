use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LenzError {
    #[error("Bytecode is empty")]
    EmptyBytecode,

    #[error("Failed to parse instruction at offset {0}")]
    ParseError(usize),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("RPC Error: {0}")]
    RpcError(String),
}

pub type Result<T> = std::result::Result<T, LenzError>;
