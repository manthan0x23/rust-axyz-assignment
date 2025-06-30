use actix_web::{HttpResponse, Responder, post};
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::utils::t::{ApiResponse};

#[derive(Serialize)]
struct KeypairResponse {
    pubkey: String,
    secret: String,
}

#[post("keypair")]
pub async fn generate_keypair() -> impl Responder {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let resp = ApiResponse::success(KeypairResponse { pubkey, secret });

    HttpResponse::Ok().json(resp)
}
