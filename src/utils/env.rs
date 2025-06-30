use std::env;

use lazy_static::lazy_static;

fn set_port() -> u16 {
    dotenv::dotenv().ok();

    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

lazy_static! {
    pub static ref PORT: u16 = set_port();
}
