use cosmwasm_std::{Binary, DepsMut, Env, from_json, MessageInfo, Response, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::consts::{ORDER_MAP, RESOURCE, ORDER_MAX_DURATION, ORDER_MIN_DURATION};
use crate::ContractError;
use crate::type_order::{Order, OrderStatus};
use crate::common::{money_action, MoneyAction};
use crate::type_resource::{TotalResource, Resource};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum ReceiveMsg {
    CreateOrder { order_id: String, initiator: String, resource: Resource, duration: u64},
    ExtendOrder { order_id: String, locked_funds: u128, duration: u64 },
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
        ReceiveMsg::CreateOrder { order_id, initiator, resource, duration } => {
            //确认 Order id 唯一
            if ORDER_MAP.may_load(deps.storage, order_id.clone())?.is_some() {
                return Err(ContractError::AlreadyExists {});
            }

            create_order(deps, env,info, amount, order_id, initiator, resource, duration)
        }
        ReceiveMsg::ExtendOrder { order_id, locked_funds, duration} => {
            ORDER_MAP.update(deps.storage, order_id.clone(), |order: Option<Order>| {
                let mut order = order.ok_or(ContractError::NotFound)?;

                // 仅使用者可以续期订单
                if !order.is_initiator(info.sender){
                    return Err(ContractError::Unauthorized {});
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

                // 如果转账金额+已锁定金额不等于目标金额,则退出
                if order.locked_funds + u128::from(amount) != locked_funds {
                    return Err(ContractError::InsufficientFunds);
                }

                order.locked_funds = locked_funds;

                order.duration = duration;

                Ok::<Order, ContractError>(order)
            })?;

            // 增加总锁定金额
            money_action(deps.storage, MoneyAction::AddLocked, amount)?;

            Ok(Response::new()
                .add_attribute("action", "receive")
                .add_attribute("internal", "extend")
                .add_attribute("sender", sender)
                .add_attribute("amount", amount)
                .add_attribute("order_id", order_id))
        }
    }
}

pub fn create_order(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    amount: Uint128,
    order_id: String,
    initiator: String,
    resource: Resource,
    duration: u64,

) -> Result<Response, ContractError> {
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
        initiator.clone(),
        resource,
    );

    // 增加对应的总资源使用量
    RESOURCE.update(deps.storage, |mut total_resource| {
        total_resource.add_used(order.resource.clone())?;

        Ok::<TotalResource, ContractError>(total_resource)
    })?;

    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 增加总锁定金额数量
    money_action(deps.storage,MoneyAction::AddLocked, amount)?;

    Ok(Response::new()
        .add_attribute("action", "receive")
        .add_attribute("internal", "create_order")
        .add_attribute("sender", initiator)
        .add_attribute("amount", amount)
        .add_attribute("order_id", order_id))

}