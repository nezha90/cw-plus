use cosmwasm_std::{Deps, StdResult};
use crate::type_resource::TotalResource;
use crate::consts::RESOURCE;

pub fn query_resources(deps: Deps) -> StdResult<TotalResource> {

    RESOURCE.load(deps.storage)
}