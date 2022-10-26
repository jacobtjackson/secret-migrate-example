use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, QueryRequest,
    StdError, StdResult, Storage, WasmQuery, debug_print,
};

use crate::msg::{FirstContractQueryMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, State, ExportData};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: env.message.sender,
        first_contract_addr: msg.first_contract_addr,
        first_contract_hash: msg.first_contract_hash,
        migration_secret: None,
    };

    config(&mut deps.storage).save(&state)?;
    debug_print("hola senor");
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    debug_print("hola mundo");
    // Make sure to either not do anything else with your contract until the migration is over,
    // and never retry it, or add a mechanism for failing if a part of the migration is retried
    // and don't allow any operation before the migration is over.

    match msg {
        HandleMsg::SetMigrationSecret { secret } => set_migration_secret(deps, env, secret),
        HandleMsg::Migrate {/* paging parameters if needed */} => migrate(deps, env),
    }
}

pub fn set_migration_secret<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    secret: Binary,
) -> StdResult<HandleResponse> {
    debug_print("hola luna");
    let mut conf = config(&mut deps.storage);
    let mut state = conf.load()?;
    if env.message.sender != state.first_contract_addr {
        return Err(StdError::generic_err(
            "Only the contract set as the migration contract can set the migration secret!",
        ));
    }
    state.migration_secret = Some(secret);
    conf.save(&state)?;

    Ok(HandleResponse::default())
}

fn migrate<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let conf = config(&mut deps.storage);
    let state = conf.load()?;
    if env.message.sender != state.owner {
        return Err(StdError::generic_err(
            "Only the owner can trigger migrations of the contract!",
        ));
    }
    let secret = state.migration_secret.ok_or_else(|| {
        StdError::generic_err("The secret has not yet been set by the first contract")
    })?;

    let _response: ExportData = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.first_contract_addr,
        callback_code_hash: state.first_contract_hash,
        msg: to_binary(&FirstContractQueryMsg::Migrate { secret })?,
    }))?;

    // TODO store the exported data in this contract's state.

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::default())
}
