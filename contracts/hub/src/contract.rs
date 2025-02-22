use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, StdResult,
};
use terp_sdk::Response;

use tea::{
    hub::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg},
    Tea,
};

use crate::{error::ContractError, execute, query, upgrades};

pub const CONTRACT_NAME: &str = "crates.io:tea-hub";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    execute::init(deps, info.sender, msg.fee_rate)
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::SetFeeRate {
            fee_rate,
        } => execute::set_fee_rate(deps, fee_rate),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateTea {
            manager,
            metadata,
            transferrable,
            rule,
            expiry,
            max_supply,
        } => {
            let tea = Tea {
                manager: deps.api.addr_validate(&manager)?,
                metadata,
                transferrable,
                rule,
                expiry,
                max_supply,
                current_supply: 0,
            };
            execute::create_tea(deps, env, info, tea)
        },
        ExecuteMsg::EditTea {
            id,
            metadata,
        } => execute::edit_tea(deps, info, id, metadata),
        ExecuteMsg::AddKeys {
            id,
            keys,
        } => execute::add_keys(deps, env, info, id, keys),
        ExecuteMsg::PurgeKeys {
            id,
            limit,
        } => execute::purge_keys(deps, env, id, limit),
        ExecuteMsg::PurgeOwners {
            id,
            limit,
        } => execute::purge_owners(deps, env, id, limit),
        ExecuteMsg::MintByMinter {
            id,
            owners,
        } => execute::mint_by_minter(deps, env, id, owners, info.sender),
        ExecuteMsg::MintByKey {
            id,
            owner,
            signature,
        } => execute::mint_by_key(deps, env, id, owner, signature),
        ExecuteMsg::MintByKeys {
            id,
            owner,
            pubkey,
            signature,
        } => execute::mint_by_keys(deps, env, id, owner, pubkey, signature),
        ExecuteMsg::SetNft {
            nft,
        } => execute::set_nft(deps, info.sender, &nft),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query::config(deps)?),
        QueryMsg::Tea {
            id,
        } => to_json_binary(&query::tea(deps, id)?),
        QueryMsg::AllTea {
            start_after,
            limit,
        } => to_json_binary(&query::all_tea(deps, start_after, limit)?),
        QueryMsg::Key {
            id,
            pubkey,
        } => to_json_binary(&query::key(deps, id, pubkey)),
        QueryMsg::Keys {
            id,
            start_after,
            limit,
        } => to_json_binary(&query::keys(deps, id, start_after, limit)?),
        QueryMsg::Owner {
            id,
            user,
        } => to_json_binary(&query::owner(deps, id, user)),
        QueryMsg::Owners {
            id,
            start_after,
            limit,
        } => to_json_binary(&query::owners(deps, id, start_after, limit)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let cw2::ContractVersion {
        contract,
        version,
    } = cw2::get_contract_version(deps.storage)?;

    if contract != CONTRACT_NAME {
        return Err(ContractError::incorrect_contract_name(CONTRACT_NAME, contract));
    }

    // in the previous v1.1 update, we forgot to set the contract version to `1.1.0`
    // so for now it's still `1.0.0`
    if version != "1.0.0" {
        return Err(ContractError::incorrect_contract_version("1.0.0", version));
    }

    upgrades::v1_2::migrate(deps).map_err(ContractError::from)
}