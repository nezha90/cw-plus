use cosmwasm_std::{Addr, BankMsg, Binary, Coin, DepsMut, Env, from_json, MessageInfo, Response, StdResult, Uint128, wasm_execute, WasmMsg, WasmQuery};

use crate::consts::ORDER_MAP;
use crate::ContractError;
use crate::tx_order::ORDER_MAP;
use crate::type_order::{Order, OrderStatus};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum ReceiveMsg {
    CreateOrder { order_id: String },
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

                order.activation()?;

                Ok::<Order, ContractError>(order)
            })?;

            Ok(Response::new()
                .add_attribute("action", "receive")
                .add_attribute("sender", sender)
                .add_attribute("amount", amount)
                .add_attribute("order_id", order_id))
        }
    }
}