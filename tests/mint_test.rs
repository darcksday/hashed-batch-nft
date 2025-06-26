use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use cw721_base::InstantiateMsg;
use cw721_hashed::extension::HashedBatchExtension;
use cw721_hashed::{contract::*};

fn mock_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn mint() {
    let mut app = App::default();

    let code_id = app.store_code(mock_contract());

    let sender = Addr::unchecked("creator");

    let contract_addr = app
        .instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                name: "HashedNFT".to_string(),
                symbol: "HSHD".to_string(),
                minter: sender.to_string(),
            },
            &[],
            "Hashed NFT",
            None,
        )
        .unwrap();

    let hash = "txhash1";
    let batch_date = "2025-06-26";

    let res = app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &ExecuteMsg::Mint {
            token_id: "nft1".to_string(),
            owner: sender.to_string(),
            token_uri: Some("https://example.com/1.json".to_string()),
            extension: HashedBatchExtension {
                batch_date: batch_date.to_string(),
                hashes: vec![hash.to_string()],
            },
        },
        &[],
    );

    assert!(res.is_ok());
}