use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::reousrce_types::ResourceInfo;

pub const NODE_INFO: Map<&Addr, NodeInfo> = Map::new("node_info");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeInfo {
    pub user: Addr,                  // 节点地址
    pub owner: Addr,                 // 管理地址
    pub beneficiary: Addr,           // 受益人

    pub resource: ResourceInfo,      // 资源信息
    pub level: Level,                // 资源等级(大盒子/小盒子)

    pub reward: Uint128,             // 已经领取的奖励
    pub pending_reward: Uint128,     // 未领取的奖励

    pub pledge: Uint128,             // 质押数量

    pub score: u32,                  // 评分
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Level {
    SmallBox,
    BigBox,
}