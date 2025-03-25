use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdError};

use crate::types::{STAKING_INFO, CONTRACT_STATE, UnstakeRequest};
use crate::rewards::update_rewards;
use crate::ContractError;
use crate::consts::PENDING_TIME;

// 解押函数
pub fn un_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let user = info.sender;

    // 加载用户质押信息
    let mut staking_info = STAKING_INFO.load(deps.storage, &user)?;

    // 计算已解押但未领取的金额
    let total_unstaked_amount: Uint128 = staking_info.unstake_requests
        .iter()
        .map(|request| request.amount)
        .sum();

    // 确保用户的质押本金足够
    if staking_info.principal < total_unstaked_amount + amount {
        return Err(ContractError::Std(StdError::generic_err("Insufficient staked amount")));
    }

    // 更新用户收益
    update_rewards(deps.storage, &mut staking_info,env.block.time.seconds())?;

    // 更新用户的质押本金
    staking_info.principal -= amount;

    // 保存用户质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 添加解押请求
    let unlock_time = env.block.time.seconds() + PENDING_TIME;
    staking_info.unstake_requests.push(UnstakeRequest {
        amount,
        unlock_time,
    });

    // 保存用户质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 更新全局状态中的总质押量
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    state.total_staked -= amount;

    // 保存全局状态
    CONTRACT_STATE.save(deps.storage, &state)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "unstake")
        .add_attribute("amount", amount)
        .add_attribute("unlock_time", unlock_time.to_string()))
}