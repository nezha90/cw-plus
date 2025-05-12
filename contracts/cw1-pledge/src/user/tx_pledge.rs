use cosmwasm_std::{Uint128, Deps, Addr, StdResult, DepsMut, Env, MessageInfo, Response, StdError, BankMsg, Coin};

use crate::ContractError;
use crate::node::level::Level;
use crate::node::resource_info::ResourceInfo;
use crate::node::global::NODE_MAP;
use crate::consts::{DENOM, PENDING_TIME};
use crate::user::common::check_pledge;
use crate::node::types::NodeInfo;
use crate::node::un_pledge::UnPledgeRequest;
use crate::node::status::Status;
use crate::status::pledge::PledgeOption;
use crate::status::global::CONTRACT_STATE;

pub fn pledge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,  // Added missing parameter
    node: Addr,
    pledge_option: PledgeOption,
) -> Result<Response, ContractError> {
    // Validate the sent funds
    let sent_amount = info.funds
        .iter()
        .find(|c| c.denom == DENOM)
        .map(|c| c.amount)
        .ok_or(ContractError::NoFundsSent)?;

    if sent_amount.is_zero() {
        return Err(ContractError::NoFundsSent);
    }

    if sent_amount != pledge_option.amount {
        return Err(ContractError::InsufficientAmount);
    }

    // 加载全局状态
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // Load and validate node info
    let mut node_info = NODE_MAP.load(deps.storage, &node)?;

    if node_info.pledge.is_some() {
        return Err(ContractError::PledgeInProgress);
    }

    // Additional validation
    check_pledge(sent_amount, node_info.level, &pledge_option)?;

    // Update node pledge info
    node_info.pledge = Some(pledge_option);  // Store the entire pledge option
    node_info.pledge_start = Some(env.block.time.seconds());

    state.total_pledge += sent_amount;

    // Save updated node info
    NODE_MAP.save(deps.storage, &node, &node_info)?;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // Build response
    Ok(Response::new()
        .add_attribute("action", "pledge")
        .add_attribute("amount", sent_amount.to_string())
        .add_attribute("node", node.to_string()))
}

pub fn un_pledge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    node: Addr,
) -> Result<Response, ContractError> {
    // 加载全局状态
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // 加载用户质押信息
    let mut node_info = NODE_MAP.load(deps.storage, &node)?;

    // 仅有owner可以解除质押
    if !info.sender.eq(&node_info.owner) {
        return Err(ContractError::Unauthorized);
    }

    if node_info.pledge_start.is_none() || node_info.pledge.is_none() {
        return Err(ContractError::NotPledge);
    }

    let pledge = node_info.pledge.as_ref().ok_or(ContractError::NotPledged)?;
    let pledge_start = node_info.pledge_start.as_ref().ok_or(ContractError::NotPledged)?;


    let current_time = env.block.time.seconds();
    if current_time < pledge_start + pledge.duration {
        return Err(ContractError::Std(StdError::generic_err("The pledge time has not come")));
    }

    let amount = pledge.amount;
    node_info.pledge = None;
    node_info.pledge_start = None;

    state.total_pledge -= amount;

    // 保存用户质押信息
    NODE_MAP.save(deps.storage, &node, &node_info)?;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: node_info.owner.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount,
            }],
        })
        .add_attribute("action", "un_pledge"))
}


pub fn renew_pledge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    node: Addr,
) -> Result<Response, ContractError> {
    // 1. Load node info
    let mut node_info = NODE_MAP.load(deps.storage, &node)?;

    // 2. Authorization check - only owner can renew
    if info.sender != node_info.owner {
        return Err(ContractError::Unauthorized);
    }

    // 3. Check if pledge exists
    node_info.pledge.as_mut().ok_or(ContractError::NoActivePledge)?;

    // 4. Update pledge start time to now
    node_info.pledge_start = Some(env.block.time.seconds());

    // 5. Save updated info
    NODE_MAP.save(deps.storage, &node, &node_info)?;

    // 6. Return success response
    Ok(Response::new()
        .add_attribute("action", "renew_pledge")
        .add_attribute("node", node.to_string())
        )
}