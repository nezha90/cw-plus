use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct BatchOrder {
    pub batch_id: String,         // 批量订单ID
    pub order_ids: Vec<String>,   // 关联订单的ID列表
    pub initiator: Addr,          // 批量订单发起者
    pub total_locked_funds: u128, // 总锁定资金
    pub start_height: u64,       // 订单开始区块高度
    pub end_height: u64,         // 订单结束区块高度
}

impl BatchOrder {
    pub fn new(batch_id: String, order_ids: Vec<String>, initiator: Addr, total_locked_funds: u128, start_height: u64, end_height: u64) -> Self {
        BatchOrder{
            batch_id,
            order_ids,
            initiator,
            total_locked_funds,
            start_height,
            end_height,
        }
    }
}