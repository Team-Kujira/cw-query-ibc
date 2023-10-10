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

#[cw_serde]
pub enum IbcQuery {
  VerifyMembership {
    connection: String,
    revision_number: u64, 
    revision_height: u64, 
    proof: Binary,
    value: Binary,
  },
  VerifyNonMembership {
    connection: String,
    revision_number: u64, 
    revision_height: u64, 
    proof: Binary,
  },
}
