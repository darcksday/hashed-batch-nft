use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw721_base::msg::{ InstantiateMsg};
use cw721_hashed::extension::HashedBatchExtension;

pub type ExecuteMsg = cw721_base::ExecuteMsg<HashedBatchExtension, Empty>;
pub type  QueryMsg  = cw721_base::QueryMsg<HashedBatchExtension>;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
