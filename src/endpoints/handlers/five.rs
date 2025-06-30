use crate::utils::t::ApiResponse;
use actix_web::{post, web};
use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    valid: bool,
    message: String,
    pubkey: String,
}

#[post("message/verify")]
pub async fn verify_message(
    req: web::Json<VerifyMessageRequest>,
) -> ApiResponse<VerifyMessageResponse> {
    let pubkey = match Pubkey::from_str(&req.pubkey) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid public key"),
    };

    let sig_bytes = match BASE64.decode(&req.signature.as_bytes()) {
        Ok(bytes) => bytes,
        Err(_) => return ApiResponse::error("Invalid base64 signature"),
    };

    let signature = match Signature::try_from(sig_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return ApiResponse::error("Invalid signature format"),
    };

    let is_valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());

    ApiResponse::success(VerifyMessageResponse {
        valid: is_valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    })
}
