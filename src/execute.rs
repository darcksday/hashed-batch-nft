use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw721_base::ContractError;
use crate::contract::{CustomCw721Contract, ExecuteMsg};
use crate::extension::HashedBatchExtension;
use crate::helpers::{get_owner, require_owner};
use crate::state::USED_HASHES;

pub fn execute_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
    extension: HashedBatchExtension,
) -> Result<Response, ContractError> {
    let contract = CustomCw721Contract::default();
    let expected_owner = get_owner(deps.as_ref())?;

    // Only owner can mint
    require_owner(&info, &expected_owner)?;

    // Check uniqueness before trying to mint
    for hash in &extension.hashes {
        if USED_HASHES.has(deps.storage, hash) {
            return Err(ContractError::Std(
                StdError::generic_err(format!("Hash {} already used", hash)),
            ));
        }
    }

    // Try standard mint first
    let res = contract.execute(
        deps.branch(),
        env,
        info,
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension: extension.clone(), // clone needed for later use
        },
    )?;

    // Save hashes only after successful mint
    for hash in &extension.hashes {
        USED_HASHES.save(deps.storage, hash, &true)?;
    }

    Ok(res)
}



pub fn execute_burn(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let expected_owner = get_owner(deps.as_ref())?;
    // Only owner can mint
    require_owner(&info, &expected_owner)?;

    let contract = CustomCw721Contract::default();

    // Load token before burn to access its extension (hashes)
    let token = contract.tokens.load(deps.storage, &token_id)?;

    // Perform standard burn logic
    let res = contract.execute(
        deps.branch(),
        env,
        info,
        ExecuteMsg::Burn { token_id },
    )?;

    // After successful burn — clean hashes
    for hash in &token.extension.hashes {
        USED_HASHES.remove(deps.storage, hash);
    }

    Ok(res)
}