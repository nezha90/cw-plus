use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, BankMsg};
use crate::tx_resource::{RESOURCE_MAP, update_status_by_resource_map};
use crate::type_resource::Status;
use crate::ContractError;
use crate::type_order::{DEFAULT_DENOM, Order, OrderStatus};
use crate::common::{check_deposit, create_order_id, create_batch_order_id};
use crate::tx_order::ORDER_MAP;
use cw_storage_plus::Map;
use std::borrow::BorrowMut;
use std::io::SeekFrom::Start;

pub const BATCH_ORDER_MAP: Map<String, BatchOrder> = Map::new("batch_order");

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
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