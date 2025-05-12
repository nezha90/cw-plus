use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status {
    Init,       // 节点初始化
    Active,     // 管理员激活
    Abnormal,   // 异常
    Tombstone   // 墓碑化
}