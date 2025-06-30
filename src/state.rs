use std::sync::Arc;

use crate::utils::t::InMemoryLedger;

#[derive(Clone, Debug)]
pub struct AppState {
    pub ledger: Arc<InMemoryLedger>,
}
