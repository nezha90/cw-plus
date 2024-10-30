use crate::type_resource::{Resource, TotalResource};
use cosmwasm_std::{Env, DepsMut, MessageInfo, Response};
use crate::ContractError;
use crate::state::ADMIN_LIST;
use crate::consts::RESOURCE;

// 订单升级
pub fn execute_set_resource(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    resource: Resource,
) -> Result<Response, ContractError> {
    // 仅管理员可设置
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    let mut total_resource = RESOURCE.load(deps.storage)?;
    total_resource.set_total(resource)?;

    Ok(Response::new()
        .add_attribute("action", "set_resource")
    )
}