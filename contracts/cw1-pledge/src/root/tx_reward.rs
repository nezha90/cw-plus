use cosmwasm_std::{Uint128, Deps, Addr, StdResult, DepsMut, Env, MessageInfo, Response, StdError};

use crate::node::level::Level;
use crate::status::pledge::{PledgeOption, PLEDGE_RULE};
use crate::ContractError;
use crate::state::ADMIN_LIST;
use crate::node::reward::Reward;
use crate::node::global::NODE_MAP;
use crate::status::global::CONTRACT_STATE;

pub fn add_reward(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    node: Addr,
    reward: Reward,
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 加载全局状态
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // 不允许设置 node 已领取奖励
    // 锁仓奖励 或 待领取奖励 至少有一个不是 0
    if reward.claimed != 0 || !(reward.locked != 0 || reward.pending != 0) {
        return Err(ContractError::BadRequest)
    }

    let mut node_info = NODE_MAP.load(deps.storage, &node)?;
    node_info.reward.locked += reward.locked;
    node_info.reward.pending += reward.pending;

    state.pending_rewards += reward.pending;
    state.locked_rewards += reward.locked;

    NODE_MAP.save(deps.storage, &node, &node_info)?;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "add_reward"))
}

pub fn set_reward(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    node: Addr,
    reward: Reward,
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 加载全局状态
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // 不允许设置 node 已领取奖励
    if reward.claimed != 0 {
        return Err(ContractError::BadRequest)
    }

    let mut node_info = NODE_MAP.load(deps.storage, &node)?;

    state.pending_rewards = state.pending_rewards - node_info.reward.pending + reward.pending;
    state.locked_rewards = state.locked_rewards - node_info.reward.locked + reward.locked;

    node_info.reward.pending = reward.pending;
    node_info.reward.locked = reward.locked;

    NODE_MAP.save(deps.storage, &node, &node_info)?;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "set_reward"))
}