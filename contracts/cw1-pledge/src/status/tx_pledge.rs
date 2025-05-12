use cosmwasm_std::{Uint128, Deps, Addr, StdResult, DepsMut, Env, MessageInfo, Response, StdError};

use crate::node::level::Level;
use crate::status::pledge::{PledgeOption, PLEDGE_RULE};
use crate::ContractError;
use crate::state::ADMIN_LIST;
use crate::status::global::PLEDGE_RULE;

pub fn update_pledge_rule(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    level: Level,
    pledge_options: Vec<PledgeOption>
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 更新质押规则
    PLEDGE_RULE.save(deps.storage, &level, &pledge_options)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "update_pledge_rule"))
}

