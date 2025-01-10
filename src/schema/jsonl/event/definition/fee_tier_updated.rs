use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FeeTierUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: FeeTierUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "ft")]
    pub fee_tier: PubkeyBase58String,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,

    #[serde(rename = "odfr")]
    pub old_default_fee_rate: u16,
    #[serde(rename = "ndfr")]
    pub new_default_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FeeTierUpdatedEventOrigin {
    #[serde(rename = "sdfr")]
    SetDefaultFeeRate,
}
