use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

pub const DEFAULT_DENOM: &str = "uttnt";

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum OrderStatus {
    Active,       // 订单活跃

    Expired,      // 订单到期

    #[default]
    Exception,    // 订单异常

    Terminated,   // 订单异常终止
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum HandleAction {
    MarkAsNormal,     // 将异常订单标记为正常
    MarkAsException,  // 将正常订单标记为异常

    #[default]
    Terminate,        // 终止异常订单
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Order {
    pub id: String,              // 订单ID
    pub resource_id: String,     // 资源ID
    pub initiator: Addr,         // 订单发起者
    pub start_height: u64,       // 订单开始区块高度
    pub end_height: u64,         // 订单结束区块高度
    pub locked_funds: u128,      // 锁定的资金
    pub status: OrderStatus,     // 订单状态
}





