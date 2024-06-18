use crate::{
    binding::{KujiraMsg, KujiraQuery},
    querier::KujiraQuerier,
    ExecuteMsg, InstantiateMsg, QueryMsg,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<KujiraMsg>> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response<KujiraMsg>> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifyMembership {
            connection,
            revision_number,
            revision_height,
            proof,
            value,
            path_prefix,
            path_key,
        } => to_binary(&query_verify_membership(
            deps,
            connection,
            revision_number,
            revision_height,
            proof,
            value,
            path_prefix,
            path_key,
        )?),

        QueryMsg::VerifyNonMembership {
            connection,
            revision_number,
            revision_height,
            proof,
            path_prefix,
            path_key,
        } => to_binary(&query_verify_non_membership(
            deps,
            connection,
            revision_number,
            revision_height,
            proof,
            path_prefix,
            path_key,
        )?),
    }
}

fn query_verify_membership(
    deps: Deps<KujiraQuery>,
    connection: String,
    revision_number: u64,
    revision_height: u64,
    proof: Binary,
    value: Binary,
    path_prefix: String,
    path_key: Binary,
) -> StdResult<String> {
    let querier = KujiraQuerier::new(&deps.querier);
    let res = querier.query_verify_membership(
        connection,
        revision_number,
        revision_height,
        proof,
        value,
        path_prefix,
        path_key,
    );
    match res {
        Ok(_) => Ok("success".to_string()),
        Err(e) => Err(e),
    }
}

fn query_verify_non_membership(
    deps: Deps<KujiraQuery>,
    connection: String,
    revision_number: u64,
    revision_height: u64,
    proof: Binary,
    path_prefix: String,
    path_key: Binary,
) -> StdResult<String> {
    let querier = KujiraQuerier::new(&deps.querier);
    let res = querier.query_verify_non_membership(
        connection,
        revision_number,
        revision_height,
        proof,
        path_prefix,
        path_key,
    );

    match res {
        Ok(_) => Ok("success".to_string()), // Change this to your actual success response
        Err(e) => Err(e),
    }
}
