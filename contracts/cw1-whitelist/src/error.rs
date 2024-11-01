use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Bad request")]
    BadRequest,

    #[error("Already exists")]
    AlreadyExists,

    #[error("Not Found")]
    NotFound,

    #[error("Insufficient Capacity")]
    InsufficientCapacity,

    #[error("Shortened Duration")]
    ShortenedDuration,
}
