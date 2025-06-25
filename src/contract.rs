use cosmwasm_std::{
    entry_point, DepsMut, Deps, Env, MessageInfo, Response, Empty,
    StdResult, Binary, StdError
};
use cw721_base::{InstantiateMsg, Cw721Contract, ContractError};
use crate::extension::HashedBatchExtension;
use crate::helpers::*;
use crate::state::USED_HASHES;

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

    // Retrieve contract owner (the minter)
    let expected_owner = get_owner(deps.as_ref())?;

    match msg {
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension,
        } => {
            //  Only the contract owner can mint
            if info.sender != expected_owner {
                return Err(ContractError::Std(
                    StdError::generic_err(format!("Only owner: {} can mint|burn", expected_owner)),
                ));
            }

            // Ensure each hash is unique in the contract
            for hash in &extension.hashes {
                if USED_HASHES.has(deps.storage, hash) {
                    return Err(ContractError::Std(
                        StdError::generic_err(format!("Hash {} already used", hash)),
                    ));
                }
            }

            //  Save all hashes as used
            for hash in &extension.hashes {
                USED_HASHES.save(deps.storage, hash, &true)?;
            }

            // Call standard  mint logic
            contract.execute(
                deps,
                env,
                info,
                ExecuteMsg::Mint {
                    token_id,
                    owner,
                    token_uri,
                    extension,
                },
            )
        }

        ExecuteMsg::Burn { token_id } => {
            //  Only the contract owner can burn
            if info.sender != expected_owner {
                return Err(ContractError::Std(
                    StdError::generic_err(format!("Only owner: {} can mint|burn", expected_owner)),
                ));
            }

            // Load the NFT 
            let token = contract.tokens.load(deps.storage, &token_id)?;

            // Remove linked hashed
            for hash in &token.extension.hashes {
                USED_HASHES.remove(deps.storage, hash);
            }

            // Call standard CW721 burn logic
            contract.execute(
                deps,
                env,
                info,
                ExecuteMsg::Burn { token_id },
            )
        }

        // Call other messages of cw721-base 
        other => contract.execute(deps, env, info, other),
    }
}



