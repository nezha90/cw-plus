use cosmwasm_std::{Deps, StdResult};

use crate::type_resource::{Resource};
use crate::tx_resource::{RESOURCE_MAP};

pub fn query_resources(deps: Deps, ids: Vec<String>) -> StdResult<Vec<Resource>> {
    let mut resources = Vec::new();

    for id in ids {
        if let Some(resource) = RESOURCE_MAP.may_load(deps.storage, id)? {
            resources.push(resource);
        }
    }

    Ok(resources)
}
