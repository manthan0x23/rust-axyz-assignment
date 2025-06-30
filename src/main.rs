use std::sync::Arc;

use actix_web::{App, HttpServer, middleware::Logger, web};
use dashmap::DashMap;

use crate::{state::AppState, utils::t::InMemoryLedger};

mod endpoints;
mod state;
mod utils;

fn configure_logger() {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    configure_logger();

    let port = (*utils::env::PORT).clone();

    let ledger: Arc<InMemoryLedger> = Arc::new(InMemoryLedger {
        tokens: DashMap::new(),
        balances: DashMap::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                ledger: ledger.clone(),
            }))
            .wrap(Logger::default())
            .configure(endpoints::config::configure_endpoints)
    })
    .workers(1)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
