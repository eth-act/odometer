use serde::{Deserialize, Serialize};

use crate::engine_api::{EngineApiRequest, EngineApiResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchInput {
    // name of the benchmark
    pub name: String,
    // description of the benchmark
    pub description: String,
    // Sequence of engine api calls needed to
    //  execute the benchmark.
    //
    // Example, to make a 30M block, we first deploy a contract, then call that contract and consume
    // 30M Gas. Only the contract call is benchmarked, but we still need to engine api calls to
    // deploy the contract and fork choice update.
    pub sequence: Vec<SequenceItem>,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct SequenceItem {
    pub description: String,
    // expect_measurement specifies whether
    // we should keep the measurement for this
    // engine api request.
    pub expect_measurement: bool,
    // Raw engine API request that the CL
    // would send to the EL
    pub request: EngineApiRequest,
}

#[derive(Debug)]
pub struct BenchEngineAPIRequestSummary {
    pub description: String,
    pub time_taken_microseconds: u128,
    pub gas_used: String,
    pub response: EngineApiResponse,
}
