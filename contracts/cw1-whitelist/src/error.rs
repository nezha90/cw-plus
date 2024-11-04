use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds")]
    InsufficientFunds{received: Uint128, expected: Uint128},

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

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}


impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

