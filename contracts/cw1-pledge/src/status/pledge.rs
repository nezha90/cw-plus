use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::node::level::Level;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct PledgeOption {
    pub amount: Uint128,        // 质押数量
    pub duration: u64,          // 质押天数
}

