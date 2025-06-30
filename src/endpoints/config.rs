use actix_web::web::{self, scope};

use crate::endpoints::handlers;

pub fn configure_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(handlers::one::generate_keypair)
        .service(handlers::two::create_token)
        .service(handlers::three::mint_token)
        .service(handlers::five::verify_message)
        .service(handlers::test::test)
        .service(handlers::four::sign_message);
}
