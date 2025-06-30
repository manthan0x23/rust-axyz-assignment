use actix_web::{HttpResponse, Responder, body::BoxBody, http::StatusCode};
use dashmap::DashMap;
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct TokenMint {
    pub authority: Pubkey,
    pub decimals: u8,
}

#[derive(Clone, Debug)]
pub struct InMemoryLedger {
    pub tokens: DashMap<Pubkey, TokenMint>,
    pub balances: DashMap<(Pubkey, Pubkey), u64>,
}

/// The enum to control API response
#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T: Serialize> {
    Success { success: bool, data: T },
    Error { success: bool, error: String },
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse::Success {
            success: true,
            data,
        }
    }

    pub fn error(message: &str) -> Self {
        ApiResponse::Error {
            success: false,
            error: message.to_string(),
        }
    }

    pub fn with_status(self, status: StatusCode) -> HttpResponse {
        HttpResponse::build(status)
            .content_type("application/json")
            .json(self)
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let status = match &self {
            ApiResponse::Success { .. } => StatusCode::OK,
            ApiResponse::Error { .. } => StatusCode::BAD_REQUEST,
        };
        self.with_status(status)
    }
}
