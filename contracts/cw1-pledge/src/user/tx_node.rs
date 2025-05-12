use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, Uint128, Addr};

use crate::node::resource_info::{ResourceInfo, check_resource};
use crate::node::level::Level;
use crate::ContractError;
use crate::node::global::NODE_MAP;
use crate::node::types::NodeInfo;
use crate::node::status::Status;
use crate::status::pledge::PledgeOption;
use crate::node::reward::Reward;

// 用户创建节点
pub fn create_node(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    resource: ResourceInfo,
    level: Level,
) -> Result<Response, ContractError> {
    // let owner = deps.api.addr_validate(owner.as_str())?;
    // let beneficiary = deps.api.addr_validate(beneficiary.as_str())?;

    let worker = info.sender;

    if NODE_MAP.may_load(deps.storage, &worker)?.is_so() {
        return Err(ContractError::Std(StdError::generic_err("Node already exist")));
    }

    if !check_resource(&resource, &level) {
        return Err(ContractError::Std(StdError::generic_err("resource exception")));
    }
    let node = NodeInfo{
        worker: worker.clone(),
        owner: worker.clone(),

        resource,
        level,

        reward: Reward::zero(),

        integral: Uint128::zero(),

        pledge: None,
        pledge_start: None,

        score: 0,

        status: Status::Init,
    };

    // 保存用户质押信息
    NODE_MAP.save(deps.storage, &worker, &node)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "create_node")
        .add_attribute("user", worker))
}

pub fn update_node_owner(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let worker = info.sender;

    let new_owner = deps.api.addr_validate(new_owner.as_str())?;

    // 加载node质押信息
    let mut node_info = NODE_MAP.load(deps.storage, &worker)?;

    let old_owner = node_info.owner;

    node_info.owner = new_owner.clone();

    // 保存用户质押信息
    NODE_MAP.save(deps.storage, &worker, &node_info)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "update_node_owner")
        .add_attribute("worker", worker)
        .add_attribute("old_owner", old_owner)
        .add_attribute("new_owner", new_owner))
}

pub fn update_node_resource(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    node: Addr,
    resource: ResourceInfo,
    level: Level,
) -> Result<Response, ContractError> {
    // 加载node质押信息
    let mut node_info = NODE_MAP.load(deps.storage, &worker)?;

    if !node_info.owner.eq(&node) && !node_info.worker.eq(&node) {
        return Err(ContractError::Unauthorized {})
    }

    if !check_resource(&resource, &level) {
        return Err(ContractError::Std(StdError::generic_err("resource exception")));
    }

    node_info.resource = resource;
    node_info.level = level;

    // 保存用户质押信息
    NODE_MAP.save(deps.storage, &worker, &node_info)?;

    // 返回成功响应
    Ok(Response::new()
        .add_attribute("action", "update_node_resource"))
}