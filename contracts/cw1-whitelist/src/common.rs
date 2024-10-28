use sha2::{Digest, Sha256};
use cosmwasm_std::{Env, MessageInfo, Coin};
use crate::ContractError;
use crate::type_order::DEFAULT_DENOM;

pub fn create_order_id(_env: Env, index: usize) -> String {
    let mut hasher = Sha256::new();

    // 将区块高度和发送者地址作为输入进行哈希
    hasher.update(env.block.height.to_be_bytes());
    hasher.update(env.transaction.index);
    hasher.update(index.into());

    let result = hasher.finalize();

    result.to_string()
}

pub fn check_deposit( info: MessageInfo, total_cost: u128) -> Result<(), ContractError> {
    let sent_funds = info.funds.iter().find(|coin| coin.denom == DEFAULT_DENOM);

    return if let Some(Coin { amount, .. }) = sent_funds {
        if *amount < total_cost.into() {
            Err(ContractError::OtherError)
            //return Err(StdError::generic_err("Insufficient funds sent"));
        } else {
            Ok(())
        }
    } else {
        Err(ContractError::OtherError)
        //return Err(StdError::generic_err("No funds sent"));
    }
}


pub fn create_batch_order_id(_env: Env) -> String {
    let mut hasher = Sha256::new();

    // 将区块高度和发送者地址作为输入进行哈希
    hasher.update(env.block.height.to_be_bytes());
    hasher.update(env.transaction.index);

    let result = hasher.finalize();

    result.to_string()
}