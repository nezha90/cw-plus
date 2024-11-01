use cosmwasm_std::{Deps, StdResult};

use crate::type_order::Order;
use crate::consts::{ORDER_MAP};

pub fn query_orders(deps: Deps, order_ids: Vec<String>) -> StdResult<Vec<Order>> {
    let mut orders: Vec<_> = Vec::new();

    for order_id in order_ids {
        // 尝试从 ORDER_MAP 获取订单
        if let Some(order) = ORDER_MAP.may_load(deps.storage, order_id.clone())? {
            orders.push(order);
        } else {
            // 如果订单 ID 不存在，可以选择跳过或返回错误
            // 此处我们选择跳过
            continue;
        }
    }

    Ok(orders)
}