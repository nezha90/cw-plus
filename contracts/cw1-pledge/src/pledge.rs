use cosmwasm_std::{Uint128, Deps, Addr, StdResult, DepsMut, Env, MessageInfo, Response, StdError};

use crate::node_types::{Level, NodeInfo, NODE_INFO};
use crate::reousrce_types::ResourceInfo;
use crate::msg::PledgeResponse;
use crate::ContractError;
use crate::consts::DENOM;

// 根据资源种类决定质押量
pub fn get_pledge(_level: Level, _resource: ResourceInfo) -> Uint128 {
    return Uint128::from(0);
}

pub fn check_pledge(amount: Uint128, level: Level, resource: ResourceInfo) -> Result<(), ContractError> {
    return if get_pledge(level, resource) != amount {
        Err(ContractError::Std(StdError::generic_err("Anomaly pledge quantity")))
    } else {
        Ok(())
    }
}

pub fn query_pledge(_deps: Deps, level: Level, resource: ResourceInfo) -> StdResult<StakingInfoResponse> {
    // 计算对应资源应有的质押
    let pledge = get_pledge(level, resource);

    Ok(PledgeResponse{pledge})
}

pub fn pledge(
    deps: DepsMut,
    _env: Env,
    node: Addr) -> Result<Response, ContractError> {
    // 获取转账金额
    let amount = info.funds
        .iter()
        .find(|c| c.denom == DENOM)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);

    // 加载节点信息
    let mut node_info = NODE_INFO.load(deps.storage, &node)?;

    // 检查节点配置与转入资金是否符合
    check_pledge(amount, node_info.level, node_info.resource)?;

    // 检查通过后设置质押金额
    node_info.pledge = amount;

    // 保存节点质押信息
    STAKING_INFO.save(deps.storage, &user, &staking_info)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "pledge")
        .add_attribute("amount", amount))
}

