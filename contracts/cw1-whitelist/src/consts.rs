use std::borrow::BorrowMut;

use cosmwasm_std::{Addr, BankMsg, Binary, Coin, DepsMut, Env,
                   MessageInfo, Response, StdResult, to_json_binary, Uint128,
                   wasm_execute, WasmMsg, WasmQuery};
use cw20_base::contract::{execute, query};
use cw20_base::msg::{ExecuteMsg, QueryMsg};
use cw_storage_plus::{Item, Map};
use sha2::{Digest, Sha256};

use crate::common::{check_deposit, create_order_id, send, transfer};
use crate::ContractError;
use crate::state::ADMIN_LIST;
use crate::type_order::{DEFAULT_DENOM, HandleAction, Order, OrderStatus, Resource};

const CPU_UNIT_PRICE: u128 = 1;
const MEM_UNIT_PRICE: u128 = 1;
const DISK_UNIT_PRICE: u128 = 1;

pub const ORDER_MAP: Map<String, Order> = Map::new("orders");

pub const ORDER_MIN_DURATION: u64 = 600;

pub const EARNINGS: Uint128 = IntK::new("earnings");