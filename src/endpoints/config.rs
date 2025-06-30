use actix_web::web::{self, scope};

use crate::endpoints::handlers;

pub fn configure_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(handlers::one::generate_keypair)
        .service(handlers::two::create_token)
        .service(handlers::test::test);
}
