use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::node::resource_info::ResourceInfo;
use crate::node::level::Level;
use crate::node::status::Status;
use crate::node::un_pledge::UnPledgeRequest;
use crate::status::pledge::PledgeOption;
use crate::node::reward::Reward;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeInfo {
    //pub id:  String,                 // TITAN 网络上的全局唯一 id
    pub worker: Addr,                  // 工作地址
    pub owner: Addr,                 // 管理地址
    //pub beneficiary: Addr,           // 受益人

    pub resource: ResourceInfo,      // 资源信息
    pub level: Level,                // 资源等级(大盒子/小盒子)

    pub reward: Reward,             // 奖励

    pub integral: Uint128,           // 积分

    pub pledge: Option<PledgeOption>,// 质押
    pub pledge_start: Option<u64>,            // 质押开始时间
    //pub un_pledge_requests: Vec<UnPledgeRequest>, // 解质押列表
    pub score: u32,                  // 评分

    pub status: Status,              // 状态
}