use cw_storage_plus::{Map, Item};
use cosmwasm_std::Uint128;

use crate::type_order::{Order};
use crate::type_resource::TotalResource;

// CPU,MEM,DISK 每小时价格
pub const CPU_UNIT_PRICE: u128 = 100;
pub const MEM_UNIT_PRICE: u128 = 100;
pub const DISK_UNIT_PRICE: u128 = 2;

// CPU
// 单位: 核
pub const MIN_USED_CPU: i32 = 1;
pub const MAX_USED_CPU: i32 = 32;

// 内存
// 单位: G
pub const MIN_USED_MEN: i32 = 1;
pub const MAX_USED_MEM: i32 = 64;

// 硬盘
// 单位: G
pub const MIN_USED_DISK: i32 = 40;
pub const MAX_USED_DISK: i32 = 4000;

// 订单时长限制 1hour - 30days
pub const ORDER_MIN_DURATION: u64 = HOUR;
pub const ORDER_MAX_DURATION: u64 = HOUR * 720;

// 订单
pub const ORDER_MAP: Map<String, Order> = Map::new("orders");

// 待分配锁仓
pub const LOCKED: Item<Uint128> = Item::new("locked");

// 总锁仓奖励
pub const EARNINGS: Item<Uint128> = Item::new("earnings");

// 资源使用情况
pub const RESOURCE: Item<TotalResource> = Item::new("total_resource");

pub const CW20: Item<String> = Item::new("cw20_contract");

// 现实时间单位转换
pub const MINUTE: u64 = 10;
pub const HOUR: u64 = MINUTE * 60;