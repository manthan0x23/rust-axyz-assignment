use std::str::FromStr;

use crate::{state::AppState, utils::t::ApiResponse};
use actix_web::{post, web};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;

#[derive(Deserialize)]
pub struct TokenCreateRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct TokenCreateResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaDto>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaDto {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[post("/token/create")]
pub async fn create_token(
    req: web::Json<TokenCreateRequest>,
    state: web::Data<AppState>,
) -> ApiResponse<TokenCreateResponse> {
    let ledger = &state.ledger;

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid mint pubkey"),
    };

    let authority = match Pubkey::from_str(&req.mint_authority) {
        Ok(pk) => pk,
        Err(_) => return ApiResponse::error("Invalid authority pubkey"),
    };

    let ix = match initialize_mint(&spl_token::id(), &mint, &authority, None, req.decimals) {
        Ok(i) => i,
        Err(_) => return ApiResponse::error("Failed to build mint instruction"),
    };

    ledger.tokens.insert(
        mint,
        crate::utils::t::TokenMint {
            authority,
            decimals: req.decimals,
        },
    );

    let accounts: Vec<AccountMetaDto> = ix
        .accounts
        .into_iter()
        .map(|meta| AccountMetaDto {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let result = TokenCreateResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: data_encoding::BASE64.encode(&ix.data).to_string(),
    };

    ApiResponse::success(result)
}
