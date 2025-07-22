use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::json;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::engine_api::{EngineApiRequest, EngineApiResponse, TimedEngineApiResponse};

type HmacSha256 = Hmac<Sha256>;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("HMAC error: {0}")]
    HmacError(String),
    #[error("HTTP error: {0}")]
    RequestError(#[from] reqwest::Error),
    // TODO: Move from this error variant
    #[error("could not deserialize reqwest response: {response_txt}")]
    CouldNotDeserializeResponse {
        response_txt: String,
        serde_err: String,
    },
}

pub struct JwtClient {
    client: Client,
    secret: Vec<u8>,
    rpc_url: String,
}

impl JwtClient {
    pub fn new(secret: Vec<u8>, rpc_url: String) -> Self {
        Self {
            client: Client::new(),
            secret,
            rpc_url,
        }
    }

    pub async fn create_jwt(&self) -> Result<String, JwtError> {
        let header = json!({"alg": "HS256", "typ": "JWT"}).to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let payload = json!({"iat": timestamp}).to_string();

        let header_b64 = URL_SAFE_NO_PAD.encode(header);
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload);
        let unsigned = format!("{}.{}", header_b64, payload_b64);

        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|e| JwtError::HmacError(e.to_string()))?;
        mac.update(unsigned.as_bytes());
        let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

        Ok(format!("{}.{}.{}", header_b64, payload_b64, signature))
    }

    pub async fn send_request(
        &self,
        request: &EngineApiRequest,
    ) -> Result<TimedEngineApiResponse, JwtError> {
        let jwt = self.create_jwt().await?;
        let request_string = serde_json::to_string_pretty(&request).unwrap();

        let start = std::time::Instant::now();
        let response = self
            .client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", jwt))
            .body(request_string)
            .send()
            .await?;
        let duration = start.elapsed().as_micros();

        let response_text = response.text().await?;
        let parsed_response: EngineApiResponse =
            serde_json::from_str(&response_text).map_err(|err| {
                JwtError::CouldNotDeserializeResponse {
                    response_txt: response_text,
                    serde_err: err.to_string(),
                }
            })?;

        let summary = TimedEngineApiResponse {
            time_taken_microseconds: duration,
            response: parsed_response,
        };

        Ok(summary)
    }
}
