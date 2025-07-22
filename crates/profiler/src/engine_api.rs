use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkChoiceStateV3 {
    pub head_block_hash: String,
    pub safe_block_hash: String,
    pub finalized_block_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadV3 {
    pub block_hash: String,
    pub block_number: String,
    pub parent_hash: String,
    pub fee_recipient: String,
    pub gas_limit: String,
    pub gas_used: Option<String>,
    pub prev_randao: String,
    pub receipts_root: String,
    pub state_root: String,
    pub timestamp: String,
    pub transactions: Option<Vec<String>>,
    pub withdrawals: Option<Vec<Withdrawal>>,

    pub blob_gas_used: String,
    pub excess_blob_gas: String,
    pub base_fee_per_gas: String,
    pub extra_data: String,
    pub logs_bloom: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub index: String,
    pub validator_index: String,
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPayloadV3Params(pub ExecutionPayloadV3, pub Vec<String>, pub String);

type ForkChoiceUpdatedV3Params = Vec<ForkChoiceStateV3>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum EngineApiRequest {
    #[serde(rename = "engine_newPayloadV3")]
    NewPayloadV3 {
        jsonrpc: String,
        id: u64,
        params: NewPayloadV3Params,
    },
    #[serde(rename = "engine_forkchoiceUpdatedV3")]
    ForkchoiceUpdatedV3 {
        jsonrpc: String,
        id: u64,
        params: ForkChoiceUpdatedV3Params,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadStatus {
    pub status: String,
    #[serde(default)]
    pub witness: Option<String>,
    #[serde(default)]
    pub latest_valid_hash: Option<String>,
    #[serde(default)]
    pub validation_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkChoiceUpdatedResult {
    pub payload_status: PayloadStatus,
    pub payload_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPayloadResult {
    pub status: String,
    #[serde(default)]
    pub witness: Option<String>,
    #[serde(default)]
    pub latest_valid_hash: Option<String>,
    #[serde(default)]
    pub validation_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultType {
    ForkChoiceUpdated(ForkChoiceUpdatedResult),
    NewPayload(NewPayloadResult),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineApiResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: ResultType,
}

pub struct TimedEngineApiResponse {
    pub time_taken_microseconds: u128,
    pub response: EngineApiResponse,
}

impl EngineApiRequest {
    pub fn gas_used(&self) -> Option<String> {
        if let EngineApiRequest::NewPayloadV3 {
            jsonrpc: _,
            id: _,
            params,
        } = self
        {
            return params.0.gas_used.clone();
        }
        return None;
    }
}
