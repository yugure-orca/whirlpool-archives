use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolProtocolFeeRateUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolProtocolFeeRateUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,

    #[serde(rename = "opfr")]
    pub old_protocol_fee_rate: u16,
    #[serde(rename = "npfr")]
    pub new_protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolProtocolFeeRateUpdatedEventOrigin {
    #[serde(rename = "spfr")]
    SetProtocolFeeRate,
}
