use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdError};

use crate::types::{CONTRACT_STATE, STAKING_INFO, StakingInfo};
use crate::consts::{DENOM, MIN_PLEDGE, START_TIME};
use crate::rewards::update_rewards;
use crate::ContractError;

pub fn stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let start_time = START_TIME.load(deps.storage)?;
    if env.block.time.seconds() < start_time {
        return Err(ContractError::Std(StdError::generic_err("Service not started yet")));
    }

    // 获取转账金额
    let amount = info.funds
        .iter()
        .find(|c| c.denom == DENOM)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);

    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("No funds sent")));
    } else if amount < MIN_PLEDGE {
        return Err(ContractError::Std(StdError::generic_err("Amount is small")));
    }

    // 更新用户质押信息
    let user = info.sender;
    let mut staking_info = STAKING_INFO.load(deps.storage, &user).unwrap_or_else(|_| StakingInfo {
        user: user.clone(),
        principal: Uint128::zero(),
        pending_reward: Uint128::zero(),
        reward: Uint128::zero(),
        last_update_time: env.block.time.seconds(),
        start_time: env.block.time.seconds(),
        unstake_requests: vec![],
        remainder: 0, // 初始余数为 0
    });

    // 更新用户收益
    update_rewards(deps.storage, &mut staking_info,env.block.time.seconds())?;

    // 更新用户的质押本金
    staking_info.principal += amount;

    // 更新全局状态中的总质押量
    let mut state = CONTRACT_STATE.load(deps.storage)?;
    state.total_staked += amount;
    CONTRACT_STATE.save(deps.storage, &state)?;

    // 保存用户质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "stake")
        .add_attribute("amount", amount))
}