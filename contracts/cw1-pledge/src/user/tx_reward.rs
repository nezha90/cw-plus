use cosmwasm_std::{Uint128, Deps, Addr, StdResult, DepsMut, Env, MessageInfo, Response, StdError, BankMsg, Coin};
use crate::ContractError;
use crate::status::global::CONTRACT_STATE;
use crate::node::global::NODE_MAP;
use crate::consts::DENOM;


// 领取收益函数
pub fn claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    node: Addr,
) -> Result<Response, ContractError> {
    let user = info.sender;

    let mut node_info = NODE_MAP.load(deps.storage, &node)?;

    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // 确保可发放收益足够
    if node_info.reward.pending > state.available_rewards {
        return Err(ContractError::Std(StdError::generic_err("Insufficient available rewards")));
    }

    let amount = node_info.reward.pending;

    node_info.reward.pending = Uint128::zero();

    state.available_rewards -= amount;
    state.pending_rewards -= amount;
    state.total_pledge += amount;

    NODE_MAP.save(deps.storage, &node, &node_info)?;

    CONTRACT_STATE.save(deps.storage, &state)?;

    // 将收益发送给用户
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: node_info.owner.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount,
            }],
        })
        .add_attribute("action", "claim_rewards")
        .add_attribute("amount", reward_amount))
}