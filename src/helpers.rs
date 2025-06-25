use cosmwasm_std::{Addr, Deps, StdError, StdResult};
use crate::contract::CustomCw721Contract;

/// Utility function to get  contract owner 
pub fn get_owner(deps: Deps) -> StdResult<Addr> {
    let contract = CustomCw721Contract::default();

    // Load the optional minter from CW721
    let owner_string = contract
        .minter(deps)?
        .minter
        .ok_or_else(|| StdError::generic_err("Contract has no owner"))?;

    deps.api.addr_validate(&owner_string)
}