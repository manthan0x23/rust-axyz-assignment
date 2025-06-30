use crate::utils::t::ApiResponse;
use actix_web::{post, web};
use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    signature: String,
    public_key: String,
    message: String,
}

#[post("/message/sign")]
pub async fn sign_message(req: web::Json<SignMessageRequest>) -> ApiResponse<SignMessageResponse> {
    if req.message.is_empty() || req.secret.is_empty() {
        return ApiResponse::error("Missing required fields");
    }

    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return ApiResponse::error("Invalid base58 secret"),
    };

    let keypair = match Keypair::try_from(&secret_bytes[..]) {
        Ok(kp) => kp,
        Err(_) => return ApiResponse::error("Failed to parse secret into Keypair"),
    };

    let signature: Signature = keypair.sign_message(req.message.as_bytes());

    let response = SignMessageResponse {
        signature: BASE64.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message.clone(),
    };

    ApiResponse::success(response)
}
