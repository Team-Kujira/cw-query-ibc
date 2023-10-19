//!    Custom querier implementation for Kujira's chain core bindings

use cosmwasm_std::{Deps, DepsMut, QuerierWrapper, QueryRequest, StdResult, Binary};
use crate::binding::{KujiraQuery, IbcQuery, IbcVerifyResponse};

/// This is a helper wrapper to easily use our custom queries
pub struct KujiraQuerier<'a> {
    querier: &'a QuerierWrapper<'a, KujiraQuery>,
}

impl<'a> KujiraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, KujiraQuery>) -> Self {
        KujiraQuerier { querier }
    }

    // Query for the membership verification
    pub fn query_verify_membership(
      &self, 
      connection: String,
      revision_number: u64,
      revision_height: u64,
      proof: Binary,
      value: Binary,
      path_prefix: String,
      path_key: String,
    ) -> StdResult<IbcVerifyResponse> {
        let query = KujiraQuery::Ibc(IbcQuery::VerifyMembership {
            connection, revision_number, revision_height, proof, value, path_prefix, path_key,
        });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        self.querier.query(&request)
    }
    // Query for the non-membership verification
    pub fn query_verify_non_membership(
      &self, 
      connection: String,
      revision_number: u64,
      revision_height: u64,
      proof: Binary,
      path_prefix: String,
      path_key: String,
    ) -> StdResult<IbcVerifyResponse> {
        let query = KujiraQuery::Ibc(IbcQuery::VerifyNonMembership {
            connection, revision_number, revision_height, proof, path_prefix, path_key,
        });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        self.querier.query(&request)
    }

    pub fn inner(&self) -> &QuerierWrapper<'a, KujiraQuery> {
        self.querier
    }
}

impl<'a> From<&'a QuerierWrapper<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(querier: &'a QuerierWrapper<KujiraQuery>) -> Self {
        KujiraQuerier::new(querier)
    }
}

impl<'a> From<&'a Deps<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(deps: &'a Deps<KujiraQuery>) -> Self {
        KujiraQuerier::new(&deps.querier)
    }
}

impl<'a> From<&'a DepsMut<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(deps: &'a DepsMut<KujiraQuery>) -> Self {
        KujiraQuerier::new(&deps.querier)
    }
}
