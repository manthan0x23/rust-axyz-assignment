use crate::{endpoints::handlers::two::AccountMetaDto, state::AppState, utils::t::ApiResponse};
use actix_web::{post, web};
use serde::{Deserialize, Serialize};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use spl_token::instruction::mint_to;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaDto>,
    pub instruction_data: String,
}

#[post("/token/mint")]
pub async fn mint_token(
    req: web::Json<MintTokenRequest>,
    state: web::Data<AppState>,
) -> ApiResponse<MintTokenResponse> {
    let ledger = &state.ledger;

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid mint pubkey"),
    };

    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid destination pubkey"),
    };

    let authority = match Pubkey::from_str(&req.authority) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid authority pubkey"),
    };

    // Check if mint exists and authority is valid
    let token_info = match ledger.tokens.get(&mint) {
        Some(info) => info,
        None => return ApiResponse::error("Mint not found in ledger"),
    };

    if token_info.authority != authority {
        return ApiResponse::error("Invalid mint authority");
    }

    let ix = match mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[], // no multisig
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(_) => return ApiResponse::error("Failed to create mint_to instruction"),
    };

    let mut balance = ledger.balances.entry((destination, mint)).or_insert(0);
    *balance += req.amount;

    let accounts: Vec<AccountMetaDto> = ix
        .accounts
        .into_iter()
        .map(|meta| AccountMetaDto {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let response = MintTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: bs58::encode(ix.data).into_string(),
    };

    ApiResponse::success(response)
}
