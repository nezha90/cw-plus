use cosmwasm_std::{Env, Uint128, Binary, StdResult, wasm_execute, WasmMsg, Storage};
use cw20::Cw20ExecuteMsg;

use crate::ContractError;
use crate::consts::{LOCKED, EARNINGS};

pub fn send(cw20: String, contract: String, amount: Uint128, msg: Binary) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20,
        &Cw20ExecuteMsg::Send { contract, amount, msg },
        Vec::new(),
    )
}

pub fn transfer(cw20: String, recipient: String, amount: Uint128) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20,
        &Cw20ExecuteMsg::Transfer { recipient, amount },
        Vec::new(),
    )
}

pub enum MoneyAction {
    AddLocked,
    DelLocked,
    AddEarnings,
    DelEarnings,
}

pub fn money_action(storage: &mut dyn Storage,action: MoneyAction, amount: Uint128) -> Result<(), ContractError> {
    match action {
        MoneyAction::AddLocked => {
            let mut locked = LOCKED.load(storage)?;

            locked += amount;

            LOCKED.save(storage, &locked)?;
        }
        MoneyAction::DelLocked => {
            let mut locked = LOCKED.load(storage)?;

            if locked < amount {
                return Err(ContractError::InsufficientFunds)
            }
            locked -= amount;

            LOCKED.save(storage, &locked)?;
        }
        MoneyAction::AddEarnings => {
            let mut earnings = EARNINGS.load(storage)?;

            earnings += amount;

            EARNINGS.save(storage, &earnings)?;
        }
        MoneyAction::DelEarnings => {
            let mut earnings = EARNINGS.load(storage)?;

            if earnings < amount {
                return Err(ContractError::InsufficientFunds)
            }

            earnings -= amount;

            EARNINGS.save(storage, &earnings)?;
        }
    }

    Ok(())
}