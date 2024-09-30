use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Map;

use crate::ContractError;
use crate::resource_type::{Resource, ResourceType, Status};
use crate::state::{ADMIN_LIST};

pub const RESOURCE_MAP: Map<String, Resource> = Map::new("resources");

pub fn query_resources(deps: Deps, ids: Vec<String>) -> StdResult<Vec<Resource>> {
    let mut resources = Vec::new();

    for id in ids {
        if let Some(resource) = RESOURCE_MAP.may_load(deps.storage, id)? {
            resources.push(resource);
        }
    }

    Ok(resources)
}


// 使用一组资源
pub fn use_resources(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ids: Vec<String>,
) -> Result<Response, ContractError> {
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    for id in ids {
        RESOURCE_MAP.update(deps.storage, id, |resource: Option<Resource>|{
            let mut resource = resource.ok_or(ContractError::NotFound)?;

            resource.use_resource()?;

            Ok::<Resource, ContractError>(resource)
        })?;
    }

    Ok(Response::new().add_attribute("action", "use_resources"))
}

// 更新一组资源
pub fn update_resources(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ids: Vec<String>,
    resource_types: Vec<ResourceType>,
) -> Result<Response, ContractError> {
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    if ids.len() != resource_types.len() {
        return Err(ContractError::OtherError)
    }

    for (id, resource_type) in ids.into_iter().zip(resource_types.into_iter()) {
        RESOURCE_MAP.update(deps.storage, id, |resource: Option<Resource>|{
            let mut resource = resource.ok_or(ContractError::NotFound)?;

            resource.update_resource(resource_type)?;

            Ok::<Resource, ContractError>(resource)
        })?;
    }

    Ok(Response::new().add_attribute("action", "update_resources"))
}

// 添加资源
pub fn add_resources(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    resources: Vec<Resource>,
) -> Result<Response, ContractError> {
    // 检查消息发送者是否为管理员
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    for resource in resources {
        // 检查资源是否已经存在
        if RESOURCE_MAP.may_load(deps.storage, resource.get_id())?.is_some() {
            return Err(ContractError::AlreadyExists {});
        }

        // 将资源添加到 MAP 中
        RESOURCE_MAP.save(deps.storage, resource.get_id(), &resource)?;
    }

    Ok(Response::new().add_attribute("action", "add_resources"))
}

// 释放一组资源
pub fn release_resources(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ids: Vec<String>,
) -> Result<Response, ContractError> {
    // 检查消息发送者是否为管理员
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    for id in ids {
        RESOURCE_MAP.update(deps.storage, id, |resource: Option<Resource>|{
            let mut resource = resource.ok_or(ContractError::NotFound)?;

            // 释放资源
            resource.release_resource()?;

            Ok::<Resource, ContractError>(resource)
        })?;
    }

    Ok(Response::new().add_attribute("action", "release_resources"))
}


pub fn delete_resources(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ids: Vec<String>,
) -> Result<Response, ContractError> {
    // 检查消息发送者是否为管理员
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    for id in ids {
        // 检查资源是否存在
        let resource = RESOURCE_MAP.may_load(deps.storage, id.clone())?;
        let resource = resource.ok_or(ContractError::NotFound)?;

        // 只有空闲才能删除
        if resource.get_status() != Status::Unused {
            return Err(ContractError::OtherError)
        }

        // 从 RESOURCE_MAP 中删除资源
        RESOURCE_MAP.remove(deps.storage, id);
    }

    Ok(Response::new().add_attribute("action", "delete_resources"))
}