use actix_web::web::{self, scope};

pub fn configure_endpoints(config: &mut web::ServiceConfig) {
    config.service(scope("/"));
}
