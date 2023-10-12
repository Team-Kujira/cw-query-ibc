//!    Bindings for message execution on Kujira Core

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CustomQuery, CustomMsg, Binary};

#[cw_serde]
pub enum KujiraMsg {}
impl CustomMsg for KujiraMsg {}


#[cw_serde]
pub enum KujiraQuery {
  Ibc(IbcQuery),
}
impl CustomQuery for KujiraQuery {}

/// This contains all queries that can be made to the IBC
#[cw_serde]
pub enum IbcQuery {
  // VerifyMembership will verifies the membership of a merkle proof against the given connection, revision height, path, and value
  VerifyMembership {
    connection: String,
    revision_number: u64, 
    revision_height: u64, 
    proof: Binary,
    value: Binary,
    path: String,
  },
  // VerifyMembership will verifies the absence of a merkle proof against the given connection, revision height, and path
  VerifyNonMembership {
    connection: String,
    revision_number: u64, 
    revision_height: u64, 
    proof: Binary,
    path: String,
  },
}

/// VerifyMembershipResponse is data format returned from IbcRequest::VerifyMembership query
#[cw_serde]
pub struct VerifyMembershipResponse {
    pub is_valid: bool,
}

/// VerifyMembershipResponse is data format returned from IbcRequest::VerifyNonMembership query
#[cw_serde]
pub struct VerifyNonMembershipResponse {
    pub is_valid: bool,
}
