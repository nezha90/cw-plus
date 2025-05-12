use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::node::types::NodeInfo;
use crate::node::resource_state::ResourceState;

pub const NODE_MAP: Map<&Addr, NodeInfo> = Map::new("node_map");

pub const RESOURCE_STATE: Item<ResourceState> = Item::new("resource_state");
