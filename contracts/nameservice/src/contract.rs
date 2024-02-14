use cosmwasm_std::{
    entry_point, to_binary, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult,
};
use cw2::{set_contract_version, CONTRACT};

use crate::{
    helpers::{assert_sent_sufficient_coin, validate_name},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RecordResponse},
    state::{NameRecord, NAME_RECORDS, PURCHASE_PRICE},
    ContractError,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PURCHASE_PRICE.save(deps.storage, &msg.purchase_price);
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => register(deps, info, name),
        ExecuteMsg::Transfer { name, to } => transfer(deps, info, name, to),
    }
}

pub fn register(deps: DepsMut, info: MessageInfo, name: String) -> Result<Response, ContractError> {
    validate_name(&name);
    let init_price = PURCHASE_PRICE.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, Some(init_price.clone()));

    let key = name.as_bytes();
    if (NAME_RECORDS.may_load(deps.storage, key)?).is_some() {
        return Err(ContractError::NameTaken { name });
    }

    let record = NameRecord {
        owner: info.sender,
        price: init_price,
    };
    NAME_RECORDS.save(deps.storage, key, &record);
    Ok(Response::default())
}

pub fn transfer(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    to: String,
) -> Result<Response, ContractError> {
    let key = name.as_bytes();

    let record = if let Some(data) = NAME_RECORDS.may_load(deps.storage, key)? {
        data
    } else {
        return Err(ContractError::NameNotExists { name: name.clone() });
    };

    let coin = assert_sent_sufficient_coin(&info.funds, Some(record.price))?;
    let new_owner = deps.api.addr_validate(&to)?;
    NAME_RECORDS.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
            record.price = coin.unwrap().clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Record { name } => {
            let key = name.as_bytes();

            let record = NAME_RECORDS.may_load(deps.storage, key)?;
            let resp = RecordResponse {
                address: String::from(&record.clone().unwrap().owner),
                price: record.unwrap().price,
            };
            to_json_binary(&resp)
        }
    }
}
