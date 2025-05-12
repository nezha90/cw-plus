use cosmwasm_std::{Deps, StdResult, Addr};

use crate::status::pledge::PLEDGE_RULE;
use crate::node::level::Level;
use crate::msg::{ContractStateResponse};
use crate::status::global::{CONTRACT_STATE};


pub fn query_contract_state(deps: Deps) -> StdResult<ContractStateResponse> {
    let contract_state = CONTRACT_STATE.load(deps.storage)?;

    Ok(ContractStateResponse{contract_state})
}