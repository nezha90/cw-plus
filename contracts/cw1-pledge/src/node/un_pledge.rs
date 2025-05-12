use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnPledgeRequest {
    pub amount: Uint128, // 解押金额
    pub unlock_time: u64, // 可领取本金的时间（秒）
}