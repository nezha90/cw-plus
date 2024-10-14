use cosmwasm_std::{Deps, StdResult};

use crate::type_order::{Order};
use crate::tx_order::ORDER_MAP;

pub fn query_orders(deps: Deps, ids: Vec<String>) -> StdResult<Vec<Order>> {
    let mut orders = Vec::new();

    for id in ids {
        if let Some(order) = ORDER_MAP.may_load(deps.storage, id)? {
            orders.push(order);
        }
    }

    Ok(orders)
}
