use cosmwasm_std::{Addr, Deps, MessageInfo, StdError, StdResult};
use cw721_base::ContractError;
use crate::contract::CustomCw721Contract;

/// Utility function 
pub fn get_owner(deps: Deps) -> StdResult<Addr> {
    let contract = CustomCw721Contract::default();

    // Load the optional minter from CW721
    let owner_string = contract
        .minter(deps)?
        .minter
        .ok_or_else(|| StdError::generic_err("Contract has no owner"))?;

    deps.api.addr_validate(&owner_string)
}

pub fn require_owner(info: &MessageInfo, expected_owner: &Addr) -> Result<(), ContractError> {
    if &info.sender != expected_owner {
        Err(ContractError::Std(StdError::generic_err(format!(
            "Only owner: {} can mint|burn",
            expected_owner
        ))))
    } else {
        Ok(())
    }
}


