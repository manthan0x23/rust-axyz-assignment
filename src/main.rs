use actix_web::{App, HttpServer, middleware::Logger, web};

mod endpoints;
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
    let address = (*utils::env::ADDRESS).clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(endpoints::config::configure_endpoints)
    })
    .workers(1)
    .bind((address, port))?
    .run()
    .await
}
