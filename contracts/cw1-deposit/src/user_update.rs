use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr};

use crate::ContractError;
use crate::types::{STAKING_INFO};
use crate::rewards::update_rewards;

pub fn update(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    user: Addr,
) -> Result<Response, ContractError> {
    let mut staking_info = STAKING_INFO.load(deps.storage, &user)?;

    // 更新用户收益
    update_rewards(deps.storage, &mut staking_info,env.block.time.seconds())?;

    // 保存用户质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "update")
        .add_attribute("new_rewards", staking_info.pending_reward)
    )
}