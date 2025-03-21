use cw_storage_plus::{Item};
use cosmwasm_std::Uint128;

pub const START_TIME: Item<u64> = Item::new("start_time");
pub const END_TIME: Item<u64> = Item::new("end_time");

pub const INTEREST_RATE: u128 = 100;
pub const PRECISION: u128 = 100;
pub const MIN_PLEDGE: Uint128 = Uint128::new(1000);

pub const DENOM: &str = "uttnt";

pub const HOUR: u64 = 60 * 60;
pub const DAY: u64 = HOUR * 24;
pub const YEAR: u64 = DAY * 365;

pub const PENDING_TIME: u64 = DAY * 5;