use cosmwasm_std::{
    entry_point, DepsMut, Deps, Env, MessageInfo, Response, Empty,
    StdResult, Binary
};
use cw721_base::{InstantiateMsg, Cw721Contract, ContractError};
use crate::extension::HashedBatchExtension;

use crate::execute::*;

const CONTRACT_NAME: &str = "crates.io:cw721-hashed";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Define the custom CW721 contract type using your extension
pub type CustomCw721Contract<'a> = Cw721Contract<'a, HashedBatchExtension, Empty, Empty, Empty>;

// Define the custom ExecuteMsg type using your extension
pub type ExecuteMsg = cw721_base::ExecuteMsg<HashedBatchExtension, Empty>;

/// Entry point for contract instantiation
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Set the contract version (used for migrations)
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Use standard CW721 instantiation
    let contract = CustomCw721Contract::default();
    contract.instantiate(deps, env, info, msg)
}

/// Entry point for queries
#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: cw721_base::QueryMsg<Empty>,
) -> StdResult<Binary> {
    let contract = CustomCw721Contract::default();
    contract.query(deps, env, msg)
}

/// Entry point for execution
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract = CustomCw721Contract::default();

    match msg {
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension,
        } =>  execute_mint(deps, env, info, token_id, owner, token_uri, extension),

        ExecuteMsg::Burn { token_id } =>  execute_burn(deps, env, info, token_id),
        // Other Cw721 messages
        other => contract.execute(deps, env, info, other),
    }
}


