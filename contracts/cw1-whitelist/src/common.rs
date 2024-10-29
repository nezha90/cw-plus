use cosmwasm_std::{Env, Uint128, Binary, StdResult, wasm_execute, WasmMsg, Storage};
use cw20_base::msg::ExecuteMsg;
use sha2::{Digest, Sha256};

use crate::ContractError;
use crate::consts::{LOCKED, EARNINGS};

pub fn create_order_id(env: Env, index: usize) -> String {
    let mut hasher = Sha256::new();

    // 将区块高度和发送者地址作为输入进行哈希
    hasher.update(env.block.height.to_be_bytes());
    hasher.update(env.transaction.unwrap().index.to_be_bytes());
    hasher.update(index.to_be_bytes());

    let result = hasher.finalize();

    format!("{:?}", result)
}

pub fn send(cw20_contract: String, contract: String, amount: Uint128, msg: Binary) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20_contract,
        &ExecuteMsg::Send { contract, amount, msg },
        Vec::new(),
    )
}

pub fn transfer(cw20_contract: String, recipient: String, amount: Uint128) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20_contract,
        &ExecuteMsg::Transfer { recipient, amount },
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