use cw_storage_plus::{Item, Map};
use crate::status::types::ContractState;
use crate::node::level::Level;
use crate::status::pledge::PledgeOption;

// 存储全局状态
pub const CONTRACT_STATE: Item<ContractState> = Item::new("contract_state");

pub const PLEDGE_RULE: Map<&Level, Vec<PledgeOption>> = Map::new("pledge_rule");
