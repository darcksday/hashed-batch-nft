use cosmwasm_std::{Addr, Empty};
use cw721_hashed::{
    contract::{
        instantiate,
        execute,
    },
    extension::HashedBatchExtension,
};
use cw721_base::msg::{InstantiateMsg, ExecuteMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

fn mock_app() -> App {
    App::default()
}

fn store_code(app: &mut App) -> u64 {
    let contract = ContractWrapper::new(execute, instantiate, cw721_hashed::contract::query);
    app.store_code(Box::new(contract))
}

fn instantiate_contract(app: &mut App, code_id: u64, sender: &str) -> Addr {
    let instantiate_msg = InstantiateMsg {
        name: "Hashed NFT".to_string(),
        symbol: "HASH".to_string(),
        minter: sender.to_string(),
    };

    app.instantiate_contract(code_id, Addr::unchecked(sender), &instantiate_msg, &[], "HashedNFT", None)
        .unwrap()
}

#[test]
fn mint_duplicate_hash() {
    let mut app = mock_app();
    let sender = "creator";
    let code_id = store_code(&mut app);
    let contract_addr = instantiate_contract(&mut app, code_id, sender);

    let hash = "txhash123";
    let extension = HashedBatchExtension {
        batch_date: "2024-01-01".to_string(),
        hashes: vec![hash.to_string()],
    };

    let mint_msg: ExecuteMsg<_, Empty> = ExecuteMsg::Mint {
        token_id: "nft1".to_string(),
        owner: sender.to_string(),
        token_uri: None,
        extension: extension.clone(),
    };

    // First mint should succeed
    app.execute_contract(Addr::unchecked(sender), contract_addr.clone(), &mint_msg, &[])
        .unwrap();

    // Second mint with same hash should fail
    let mint_msg_2: ExecuteMsg<_, Empty> = ExecuteMsg::Mint {
        token_id: "nft2".to_string(),
        owner: sender.to_string(),
        token_uri: None,
        extension,
    };


    let result = app.execute_contract(Addr::unchecked(sender), contract_addr, &mint_msg_2, &[]);
    assert!(result.is_err());
}