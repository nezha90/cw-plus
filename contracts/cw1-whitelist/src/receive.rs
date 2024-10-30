use cosmwasm_std::{Binary, DepsMut, Env, from_json, MessageInfo, Response, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::consts::{ORDER_MAP};
use crate::ContractError;
use crate::type_order::{Order};
use crate::common::{money_action, MoneyAction};
use crate::type_resource::{RESOURCE, TotalResource};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum ReceiveMsg {
    CreateOrder { order_id: String },
    ExtendOrder { order_id: String, locked_funds: u128, duration: u64 },
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    match from_json(&msg)? {
        ReceiveMsg::CreateOrder { order_id } => {
            ORDER_MAP.update(deps.storage, order_id.clone(), |order: Option<Order>| {
                let mut order = order.ok_or(ContractError::NotFound)?;

                if order.locked_funds != u128::from(amount) {
                    return Err(ContractError::InsufficientFunds);
                }

                // 增加对应的总资源使用量
                RESOURCE.update(deps.storage, |mut total_resource| {
                    total_resource.add_used(order.resource)?;

                    Ok::<TotalResource, ContractError>(total_resource)
                })?;

                // 设置订单为活跃状态
                order.activation()?;

                Ok::<Order, ContractError>(total_resource)
            })?;

            // 增加总锁定金额数量
            money_action(deps.storage,MoneyAction::AddLocked, amount)?;

            Ok(Response::new()
                .add_attribute("action", "receive")
                .add_attribute("internal", "create_order")
                .add_attribute("sender", sender)
                .add_attribute("amount", amount)
                .add_attribute("order_id", order_id))
        }
        ReceiveMsg::ExtendOrder { order_id, locked_funds, duration} => {
            ORDER_MAP.update(deps.storage, order_id.clone(), |order: Option<Order>| {
                let mut order = order.ok_or(ContractError::NotFound)?;

                if order.locked_funds + u128::from(amount) != locked_funds {
                    return Err(ContractError::InsufficientFunds);
                }

                order.locked_funds = locked_funds;

                if duration < order.duration {
                    return Err(ContractError::BadRequest);
                }
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