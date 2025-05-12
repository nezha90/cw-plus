use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractState {
    pub total_pledge: Uint128,           // 总质押量
    pub total_rewards: Uint128,          // 总已发放收益
    pub available_rewards: Uint128,      // 可发放的收益数量
    pub pending_rewards: Uint128,        // 总待领取的收益
    pub locked_rewards: Uint128,         // 总锁仓的收益
}


