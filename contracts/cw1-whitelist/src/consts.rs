use cw_storage_plus::{Map, Item};
use cosmwasm_std::Uint128;

use crate::type_order::{Order};

pub const CPU_UNIT_PRICE: u128 = 1;
pub const MEM_UNIT_PRICE: u128 = 1;
pub const DISK_UNIT_PRICE: u128 = 1;

pub const ORDER_MIN_DURATION: u64 = 600;

pub const ORDER_MAP: Map<String, Order> = Map::new("orders");

pub const LOCKED: Item<Uint128> = Item::new("locked");

pub const EARNINGS: Item<Uint128> = Item::new("earnings");
