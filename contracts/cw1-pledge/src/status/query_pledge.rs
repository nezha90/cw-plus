use cosmwasm_std::{Deps, StdResult, Addr};

use crate::status::pledge::PLEDGE_RULE;
use crate::node::level::Level;
use crate::msg::PledgeRuleResponse;
use crate::status::global::PLEDGE_RULE;


pub fn query_pledge_rule(deps: Deps, level: &Level) -> StdResult<PledgeRuleResponse> {
    let pledge_rule = PLEDGE_RULE.load(deps.storage, &level)?;

    Ok(PledgeRuleResponse{pledge_rule})
}