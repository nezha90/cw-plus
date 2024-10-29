use cw_storage_plus::{Map, Item};
use cosmwasm_std::Uint128;

use crate::type_order::{Order};

const CPU_UNIT_PRICE: u128 = 1;
const MEM_UNIT_PRICE: u128 = 1;
const DISK_UNIT_PRICE: u128 = 1;

pub const ORDER_MIN_DURATION: u64 = 600;

pub const EARNINGS: Item<Uint128> = Item::new("earnings");

pub const ORDER_MAP: Map<String, Order> = Map::new("orders");
