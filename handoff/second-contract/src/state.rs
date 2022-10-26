use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr, Storage, Coin};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: HumanAddr,
    pub first_contract_addr: HumanAddr,
    pub first_contract_hash: String,
    pub migration_secret: Option<Binary>,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub name: String,
    pub decimals: u8,
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub addr: HumanAddr,
    pub funds: Vec<Coin>,
}
