#[cfg(not(feature = "library"))]

use cosmwasm_std::{
    entry_point, to_binary,Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult,
};
use crate::{
    InstantiateMsg, ExecuteMsg, QueryMsg, 
    binding::{KujiraMsg, KujiraQuery},
    querier::KujiraQuerier,
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
        } => to_binary(&query_verify_membership(
            deps,
            connection,
            revision_number,
            revision_height,
            proof,
            value,
        )),

        QueryMsg::VerifyNonMembership {
            connection,
            revision_number,
            revision_height,
            proof,
        } => to_binary(&query_verify_non_membership(
            deps,
            connection,
            revision_number,
            revision_height,
            proof,
        )),

        _ => unimplemented!(),
    }
}

fn query_verify_membership(
    deps: Deps<KujiraQuery>,
    connection: String,
    revision_number: u64,
    revision_height: u64,
    proof: Binary,
    value: Binary,
) -> Binary {
    let querier = KujiraQuerier::new(&deps.querier);
    let response = querier
        .query_verify_membership(
            connection,
            revision_number,
            revision_height,
            proof,
            value,
        )
        .unwrap();

    response
}

fn query_verify_non_membership(
    deps: Deps<KujiraQuery>,
    connection: String,
    revision_number: u64,
    revision_height: u64,
    proof: Binary,
) -> Binary {
    let querier = KujiraQuerier::new(&deps.querier);
    let response = querier
        .query_verify_non_membership(
            connection,
            revision_number,
            revision_height,
            proof,
        )
        .unwrap();

    response
}