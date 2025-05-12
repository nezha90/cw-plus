use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
    pub claimed: Uint128,    // 已领取数量
    pub pending: Uint128,    // 待领取数量
    pub locked: Uint128,     // 锁仓数量
}


impl Reward {
    #[inline]
    pub const fn zero() -> Self {
        Reward{
            claimed: Uint128::zero(),
            pending: Uint128::zero(),
            locked: Uint128::zero(),
        }
    }
}