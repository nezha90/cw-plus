use cosmwasm_std::{Binary, DepsMut, Env, from_json, MessageInfo, Response, Uint128, wasm_execute, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::consts::{ORDER_MAP, RESOURCE, ORDER_MAX_DURATION, ORDER_MIN_DURATION};
use crate::ContractError;
use crate::type_order::{Order, OrderStatus};
use crate::common::{money_action, MoneyAction};
use crate::type_resource::{TotalResource, Resource};
use crate::msg::ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum ReceiveMsg {
    CreateOrder { order_id: String, cpu: u32, memory: u32, disk: u32, duration: u64},
    ExtendOrder { order_id: String, duration: u64 },
    UpdateOrder { order_id: String, new_order_id: String, cpu: u32, memory: u32, disk: u32, duration: u64},
}

pub fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    match from_json(&msg)? {
        ReceiveMsg::CreateOrder { order_id, cpu, memory, disk, duration } =>
            {
                let resource= Resource{cpu, memory, disk};
                create_order(deps, env,info, sender, amount, order_id, resource, duration)
            },
        ReceiveMsg::ExtendOrder { order_id, duration} =>
            {
                extend_order(deps, env, info, sender, amount, order_id, duration)
            },
        ReceiveMsg::UpdateOrder { order_id, new_order_id, cpu, memory, disk,duration} =>
            {
                let resource= Resource{cpu, memory, disk};
                update_order(deps, env, info, sender, amount, order_id, new_order_id, resource, duration)
            },
    }
}

pub fn create_order_inner(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    sender: String,
    amount: Uint128,
    order_id: String,
    resource: Resource,
    duration: u64,
) -> Result<(), ContractError> {
    //确认 Order id 唯一
    if ORDER_MAP.may_load(deps.storage, order_id.clone())?.is_some() {
        return Err(ContractError::AlreadyExists {});
    }

    // 资源最少使用权限
    if duration < ORDER_MIN_DURATION || duration > ORDER_MAX_DURATION {
        return Err(ContractError::BadRequest);
        //return Err(StdError::generic_err("Resource is not available"));
    }

    if !resource.check() {
        return Err(ContractError::BadRequest);
    }

    // 计算总费用
    let total_cost = resource.calc_price(duration)?;

    if total_cost != u128::from(amount) {
        return Err(ContractError::InsufficientFunds);
    }

    // 创建订单
    let order = Order::new(
        order_id.clone(),
        env.block.height,
        duration,
        total_cost,
        sender.clone(),
        resource,
    );

    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 增加总锁定金额数量
    money_action(deps.storage,MoneyAction::AddLocked, amount)?;

    // 增加对应的总资源使用量
    RESOURCE.update(deps.storage, |mut total_resource| {
        total_resource.add_used(order.resource.clone())?;

        Ok::<TotalResource, ContractError>(total_resource)
    })?;

    Ok(())
}

pub fn create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    order_id: String,
    resource: Resource,
    duration: u64,
) -> Result<Response, ContractError> {
    create_order_inner(deps, env, info, sender, amount, order_id, resource, duration)?;

    Ok(Response::new()
        .add_attribute("action", "receive")
        .add_attribute("internal", "create_order")
    )
}

fn extend_order(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    sender: String,
    amount: Uint128,
    order_id: String,
    duration: u64,
) -> Result<Response, ContractError> {
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    if !order.is_initiator(sender) {
        return Err(ContractError::Unauthorized {})
    }
    // 不需要续期
    // 1. 订单未处于激活状态
    // 2. 订单已经过期
    // 3. 续期时长大于最大时长
    // 4. 续期时长小于等于当前时长
    if order.status != OrderStatus::Active||
        order.start_height + order.duration > env.block.height ||
        duration > ORDER_MAX_DURATION ||
        duration <= order.duration {
        return Err(ContractError::BadRequest {});
    }

    // 新的总金额
    let price = order.resource.calc_price(duration)?;

    // 需补充的
    let shortage = price - order.locked_funds;

    // 如果转账金额 不等于 目标金额,则退出
    if u128::from(amount) != shortage {
        return Err(ContractError::InsufficientFunds);
    }

    order.locked_funds = price;

    order.duration = duration;

    // 增加总锁定金额
    money_action(deps.storage, MoneyAction::AddLocked, amount)?;

    Ok(Response::new()
        .add_attribute("action", "receive")
        .add_attribute("internal", "extend")
    )
}

pub fn update_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    order_id: String,
    new_order_id: String,
    resource: Resource,
    duration: u64,
) -> Result<Response, ContractError>{
    let order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 仅订单所有者可以升级订单
    if !order.is_initiator(sender.clone()) {
        return Err(ContractError::Unauthorized {})
    }

    // 新订单结束时间不能小于当前订单
    if env.block.height + duration < order.start_height + order.duration {
        return Err(ContractError::ShortenedDuration {})
    }

    // 至少一项资源大于现有,且所有资源不能小于之前
    if !resource.is_greater_than(&order.resource) {
        return Err(ContractError::BadRequest {})
    }

    // 释放旧订单消息
    let release_msg = wasm_execute(
        env.contract.address.clone(),
        &ExecuteMsg::<Empty>::ReleaseOrder {order_id: order_id.clone()},
        Vec::new(),
    )?;

    create_order_inner(deps, env, info, sender, amount, new_order_id, resource, duration)?;

    Ok(Response::new()
        .add_message(release_msg)
        .add_attribute("action", "receive")
        .add_attribute("internal", "update_order")
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::HOUR;
    use cosmwasm_std::to_json_binary;

    #[test]
    fn get_receive_msg() {
        let resource = Resource{
            cpu: 4,
            memory: 4,
            disk: 50,
        };

        let duration = HOUR * 12;

        let order_id = "1".to_string();

        let msg = ReceiveMsg::CreateOrder {order_id, resource: resource.clone(), duration};
        let binary = to_json_binary(&msg).unwrap();


        println!("msg binary");
        println!("{}", binary);

        println!("price");
        let price = resource.calc_price(duration).unwrap();
        println!("{}", Uint128::from(price));
    }
}
