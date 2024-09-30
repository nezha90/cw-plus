use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // 自定义的资源错误
    #[error("Insufficient resources: requested {requested}, but only {available} available.")]
    InsufficientResources { requested: u64, available: u64 },

    #[error("Over-release: attempting to release {requested}, but only {used} resources have been used.")]
    OverRelease { requested: u64, used: u64 },

    #[error("Resource type does not exist.")]
    ResourceTypeNotFound,

    #[error("Undefined")]
    OtherError,

    #[error("Resource overflow")]
    ResourceOverflow,

    #[error("not found")]
    NotFound,

    #[error("Already Exists")]
    AlreadyExists,
}
