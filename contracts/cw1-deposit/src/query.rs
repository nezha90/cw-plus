use cosmwasm_std::{Deps, StdResult, Addr};

use crate::types::{STAKING_INFO, CONTRACT_STATE};
use crate::msg::{AllUsersResponse, ContractStateResponse, StakingInfoResponse};
use crate::consts::{START_TIME, END_TIME};

pub fn query_staking_info(deps: Deps, user: Addr) -> StdResult<StakingInfoResponse> {
    // 加载用户的质押信息
    let staking_info = STAKING_INFO.load(deps.storage, &user)?;

    Ok(StakingInfoResponse{staking_info})
}

pub fn query_all_users(deps: Deps) -> StdResult<AllUsersResponse> {
    let users = STAKING_INFO
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<StdResult<Vec<Addr>>>()?;

    Ok(AllUsersResponse{users})
}

pub fn query_contract_state(deps: Deps) -> StdResult<ContractStateResponse> {
    let contract_state = CONTRACT_STATE.load(deps.storage)?;

    let start_time = START_TIME.load(deps.storage)?;

    let end_time = END_TIME.load(deps.storage)?;

    Ok(ContractStateResponse{contract_state, start_time, end_time})
}