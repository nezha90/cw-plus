use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::consts::{LOCKED, EARNINGS};

pub fn query_locked(deps: Deps) -> StdResult<Uint128> {
    LOCKED.load(deps.storage)
}

pub fn query_earnings(deps: Deps) -> StdResult<Uint128> {
    EARNINGS.load(deps.storage)
}