use cosmwasm_std::{Uint128, StdError};

use crate::node::level::Level;
use crate::node::resource_info::ResourceInfo;
use crate::ContractError;
use crate::status::pledge::PledgeOption;


pub fn check_pledge(amount: Uint128, level: Level, option: &PledgeOption) -> Result<(), ContractError> {
    return Ok(())
}