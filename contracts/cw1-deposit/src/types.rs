use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// 存储全局状态
pub const CONTRACT_STATE: Item<ContractState> = Item::new("contract_state");

// 存储用户质押信息
pub const STAKING_INFO: Map<&Addr, StakingInfo> = Map::new("staking_info");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingInfo {
    pub user: Addr,                 // 用户

    pub principal: Uint128,          // 质押的本金
    pub pending_reward: Uint128,     // 待领取的收益
    pub reward: Uint128,             // 已领取的收益

    pub last_update_time: u64,       // 上次收益更新时间（s）
    pub start_time: u64,             // 质押开始时间（s）

    pub unstake_requests: Vec<UnstakeRequest>, // 解押请求队列

    pub remainder: u128,             // 余数，用于高精度计算
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractState {
    pub total_staked: Uint128,           // 总质押量
    pub total_rewards: Uint128,          // 总已发放收益
    pub available_rewards: Uint128,      // 可发放的收益数量
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnstakeRequest {
    pub amount: Uint128, // 解押金额
    pub unlock_time: u64, // 可领取本金的时间（秒）
}