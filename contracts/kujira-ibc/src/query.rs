use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub enum QueryMsg {
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