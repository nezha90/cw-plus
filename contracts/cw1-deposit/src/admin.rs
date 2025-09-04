use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdError, BankMsg, Coin, Addr};

use crate::types::{CONTRACT_STATE, STAKING_INFO};
use crate::consts::{DENOM, END_TIME};
use crate::ContractError;
use crate::state::ADMIN_LIST;

pub fn fund_rewards(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError>  {
    // 获取转账金额
    let amount = info.funds
        .iter()
        .find(|c| c.denom == DENOM)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);

    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("No funds sent")))
    }

    // 更新全局状态中的可发放收益
    let mut state = CONTRACT_STATE.load(deps.storage)?;
    state.available_rewards += amount;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "fund_rewards")
        .add_attribute("amount", amount))
}

pub fn update_end_time(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_end_time: u64,
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 更新结束日期
    END_TIME.save(deps.storage, &new_end_time)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "update_end_time")
        .add_attribute("new_end_time", new_end_time.to_string()))
}

pub fn extract_fund(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("Funding abnormality")))
    }

    // 更新全局状态中的可发放收益
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    if amount > state.available_rewards {
        return Err(ContractError::Std(StdError::generic_err("Insufficient staked amount")))
    }

    state.available_rewards -= amount;
    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount,
            }],
        })
        .add_attribute("action", "extract_fund")
        .add_attribute("amount", amount))
}

pub fn reset_un_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
)  -> Result<Response, ContractError> {
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    let mut info = STAKING_INFO.load(deps.storage, &user)?;

    if info.unstake_requests.is_empty() {
        return Ok(Response::new()
            .add_attribute("action", "reset_un_stake")
            .add_attribute("user", user.to_string())
            .add_attribute("status", "no_requests_to_update"));
    }


    let current_time = env.block.time.seconds();

    for request in info.unstake_requests.iter_mut() {
        request.unlock_time = current_time;
    }

    STAKING_INFO.save(deps.storage, &user, &info)?;

    Ok(Response::new()
        .add_attribute("action", "reset_un_stake")
        .add_attribute("user", user.to_string())
        .add_attribute("new_unlock_time", current_time.to_string()))
}
