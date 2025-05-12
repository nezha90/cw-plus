use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not pledged")]
    NotPledged,

    #[error("No funds sent")]
    NoFundsSent,

    InsufficientAmount,

    PledgeInProgress,

    BadRequest,
}
