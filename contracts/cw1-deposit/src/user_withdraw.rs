use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdError, Coin, BankMsg};

use crate::types::{STAKING_INFO, CONTRACT_STATE};
use crate::rewards::update_rewards;
use crate::consts::{DENOM};
use crate::ContractError;

pub fn withdraw_principal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user = info.sender;

    // 加载用户质押信息
    let mut staking_info = STAKING_INFO.load(deps.storage, &user)?;

    // 获取当前时间
    let current_time = env.block.time.seconds();

    // 过滤出已解锁的解押请求
    let mut total_amount = Uint128::zero();
    staking_info.unstake_requests.retain(|request| {
        if request.unlock_time <= current_time {
            total_amount += request.amount;
            false // 移除已解锁的请求
        } else {
            true // 保留未解锁的请求
        }
    });

    // 如果没有可领取的本金
    if total_amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("No funds available to withdraw")));
    }

    // 保存更新后的用户质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 将本金发送给用户
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: user.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount: total_amount,
            }],
        })
        .add_attribute("action", "withdraw_principal")
        .add_attribute("amount", total_amount))
}

// 领取收益函数
pub fn claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user = info.sender;

    // 加载用户质押信息
    let mut staking_info = STAKING_INFO.load(deps.storage, &user)?;

    // 加载全局状态
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // 更新用户收益
    update_rewards(deps.storage, &mut staking_info,env.block.time.seconds())?;

    // 确保可发放收益足够
    if staking_info.pending_reward > state.available_rewards {
        return Err(ContractError::Std(StdError::generic_err("Insufficient available rewards")));
    }

    // 发放收益
    let reward_amount = staking_info.pending_reward;
    state.available_rewards -= reward_amount;

    //设置已经领取的奖励
    staking_info.reward += staking_info.pending_reward;

    // 重置用户的待领取收益
    staking_info.pending_reward = Uint128::zero();

    // 更新已经领取的收益
    state.total_rewards += reward_amount;

    // 保存更新后的数据
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;
    CONTRACT_STATE.save(deps.storage, &state)?;

    // 将收益发送给用户
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: user.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount: reward_amount,
            }],
        })
        .add_attribute("action", "claim_rewards")
        .add_attribute("amount", reward_amount))
}